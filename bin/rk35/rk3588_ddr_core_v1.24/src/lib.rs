#![no_std]
//! RK3588/RK3588S DDR v1.24 reconstruction.
//!
//! This crate contains only behavior that has been tied back to the v1.24
//! reference image by the reverse-engineering notes. It is not yet a complete
//! or hardware-tested DDR initializer.

pub mod volatile;

pub const TIMER_FREQUENCY_HZ: u64 = 24_000_000;

/// Raw firmware layout evidenced by `dram_capacity_bytes@0x0c1c` and
/// `print_dram_geometry@0x0ce0` in the v1.24 reference image.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DramGeometryRaw {
    pub rank_count: u32,      // +0x00
    pub column_bits: u32,     // +0x04
    pub bank_bits: u32,       // +0x08
    pub bus_width_shift: u32, // +0x0c, semantic confidence: medium
    pub die_width_shift: u32, // +0x10, semantic confidence: medium
    pub row_3_4: u32,         // +0x14
    pub cs0_row: u32,         // +0x18
    pub cs1_row: u32,         // +0x1c
    pub cs2_row: u32,         // +0x20
    pub cs3_row: u32,         // +0x24
    pub cs0_high_row: u32,    // +0x28, semantic confidence: medium
    pub cs1_high_row: u32,    // +0x2c
    pub cs2_high_row: u32,    // +0x30
    pub cs3_high_row: u32,    // +0x34
}

/// DesignWare UART register slice touched by `uart_debug_init@0x10dac`.
#[repr(C)]
pub struct DwUartRegisterSlice {
    pub rbr_thr_dll: u32, // +0x00
    _pad04: [u32; 2],
    pub lcr: u32, // +0x0c
    _pad10: [u32; 30],
    pub srr: u32, // +0x88
    _pad8c: [u32; 3],
    pub sfe: u32, // +0x98
}

pub fn dram_capacity_bytes(g: &DramGeometryRaw, rank: usize, dram_type_flag: bool) -> u64 {
    let extra = if dram_type_flag {
        0
    } else {
        u32::from(g.die_width_shift == 0) + 1
    };
    let common = g.bus_width_shift + g.column_bits + extra + g.bank_bits;
    let rows = [g.cs0_row, g.cs1_row, g.cs2_row, g.cs3_row];
    let mut cap = [0_u64; 4];

    cap[0] = 1_u64 << ((common + rows[0]) & 63);
    if g.rank_count > 1 {
        cap[1] = 1_u64 << ((common + rows[1]) & 63);
    }
    if g.rank_count == 4 {
        cap[2] = 1_u64 << ((common + rows[2]) & 63);
        cap[3] = 1_u64 << ((common + rows[3]) & 63);
    }

    if rank > 3 {
        cap.iter().sum()
    } else {
        cap[rank]
    }
}

pub const fn timer_ticks_for_us(counter_hz: u64, us: u64) -> u64 {
    (counter_hz / 1_000_000) * us
}

pub const fn uart_divisor_for_baud(baud: u32) -> u32 {
    match baud {
        115_200 => 13,
        750_000 => 2,
        _ => 1,
    }
}

/// Early register writes recovered from the v1.24 image.
///
/// # Safety
/// Must only run on the expected RK3588/RK3588S memory map during the proper
/// early-boot phase.
pub unsafe fn pre_ddr_hw_init() {
    use volatile::{read32, write32};

    if unsafe { read32(0xFF00_0010) } != 5 {
        unsafe { write32(0xFD5F_8098, 0xFF01_5500) };
    }

    for addr in [0xFE01_00F0_usize, 0xFE01_00F4, 0xFE01_00F8, 0xFE01_00FC] {
        unsafe { write32(addr, 0) };
    }
}

#[cfg(target_arch = "aarch64")]
/// Busy-wait using the architectural counter.
///
/// # Safety
/// Requires accessible `CNTPCT_EL0`/`CNTFRQ_EL0` in the current exception level.
pub unsafe fn udelay(us: u64) {
    let start: u64;
    let freq: u64;
    let mut now: u64;

    unsafe {
        core::arch::asm!("mrs {0}, cntpct_el0", out(reg) start);
        core::arch::asm!("mrs {0}, cntfrq_el0", out(reg) freq);
    }

    let end = start.wrapping_add(timer_ticks_for_us(freq, us));
    loop {
        unsafe { core::arch::asm!("mrs {0}, cntpct_el0", out(reg) now) };
        if now > end {
            break;
        }
    }
}

/// Offsets repeatedly accessed through function arguments in major DDR routines.
/// These are structural evidence only; unknown fields intentionally remain unnamed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateOffsetEvidence {
    pub offset: u32,
    pub hits: u16,
}

pub const DDR_STATE_OFFSET_EVIDENCE: &[StateOffsetEvidence] = &[
    StateOffsetEvidence {
        offset: 0x1B0,
        hits: 2,
    },
    StateOffsetEvidence {
        offset: 0x6C,
        hits: 2,
    },
    StateOffsetEvidence {
        offset: 0x1,
        hits: 1,
    },
    StateOffsetEvidence {
        offset: 0x1B4,
        hits: 1,
    },
    StateOffsetEvidence {
        offset: 0x14,
        hits: 1,
    },
    StateOffsetEvidence {
        offset: 0x228,
        hits: 1,
    },
    StateOffsetEvidence {
        offset: 0x230,
        hits: 1,
    },
    StateOffsetEvidence {
        offset: 0x10,
        hits: 1,
    },
];

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(size_of::<DramGeometryRaw>(), 0x38);
        assert_eq!(offset_of!(DramGeometryRaw, row_3_4), 0x14);
        assert_eq!(offset_of!(DramGeometryRaw, cs3_high_row), 0x34);
        assert_eq!(offset_of!(DwUartRegisterSlice, lcr), 0x0c);
        assert_eq!(offset_of!(DwUartRegisterSlice, srr), 0x88);
        assert_eq!(offset_of!(DwUartRegisterSlice, sfe), 0x98);
    }

    #[test]
    fn baud() {
        assert_eq!(uart_divisor_for_baud(115_200), 13);
        assert_eq!(uart_divisor_for_baud(750_000), 2);
    }

    #[test]
    fn capacity() {
        let g = DramGeometryRaw {
            rank_count: 2,
            column_bits: 10,
            bank_bits: 3,
            bus_width_shift: 1,
            die_width_shift: 0,
            row_3_4: 0,
            cs0_row: 15,
            cs1_row: 15,
            ..Default::default()
        };
        assert_eq!(
            dram_capacity_bytes(&g, 4, false),
            dram_capacity_bytes(&g, 0, false) + dram_capacity_bytes(&g, 1, false)
        );
    }

    #[test]
    fn evidence_nonempty() {
        assert!(!DDR_STATE_OFFSET_EVIDENCE.is_empty());
        assert!(DDR_STATE_OFFSET_EVIDENCE.iter().all(|x| x.hits > 0));
    }
}
