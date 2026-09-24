//! Stage 17 shared exact boundaries recovered from RK3588 boot images.
use crate::{Mmio32,MmioWrite,RUNTIME_MINUS_FILE};use crate::stage16::BootstrapSpec;
pub const COMMON_EARLY_SOC_PRE_HOOK_WRITES:[MmioWrite;6]=[
    MmioWrite{addr:0xFD7C_0B34,value:0x01C0_01C0},MmioWrite{addr:0xFD5D_4008,value:0x2000_2000},MmioWrite{addr:0xFD5D_8008,value:0x2000_2000},MmioWrite{addr:0xFD5D_C008,value:0x2000_2000},MmioWrite{addr:0xFD7F_0A0C,value:0xB800_B800},MmioWrite{addr:0xFD7F_0A10,value:0x0003_0003},
];
pub const COMMON_EARLY_SOC_POST_HOOK_WRITE:MmioWrite=MmioWrite{addr:0xFD5A_C01C,value:0x0008_0008};
pub trait EarlySocTailHook{fn between_common_tail_writes(&mut self);}
pub fn apply_common_early_soc_tail<I:Mmio32,H:EarlySocTailHook>(io:&mut I,hook:&mut H){for w in COMMON_EARLY_SOC_PRE_HOOK_WRITES{io.write32(w.addr,w.value)}hook.between_common_tail_writes();io.write32(COMMON_EARLY_SOC_POST_HOOK_WRITE.addr,COMMON_EARLY_SOC_POST_HOOK_WRITE.value)}
pub const fn file_offset_for_runtime(spec:BootstrapSpec,runtime:u64)->Option<u64>{if runtime<spec.header.copy_dst||runtime>=spec.header.copy_end{return None}let off=runtime-RUNTIME_MINUS_FILE;if off<spec.header.copy_src{None}else{Some(off)}}
#[cfg(test)]mod tests{use super::*;struct M{n:u8,last:(u64,u32)}impl Mmio32 for M{fn read32(&mut self,_:u64)->u32{0}fn write32(&mut self,a:u64,v:u32){self.n+=1;self.last=(a,v)}}struct H(u8);impl EarlySocTailHook for H{fn between_common_tail_writes(&mut self){self.0+=1}}#[test]fn exact_tail(){let(mut m,mut h)=(M{n:0,last:(0,0)},H(0));apply_common_early_soc_tail(&mut m,&mut h);assert_eq!(m.n,7);assert_eq!(h.0,1);assert_eq!(m.last,(0xFD5A_C01C,0x0008_0008));}}
