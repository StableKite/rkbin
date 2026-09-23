//! Stage-10 SPL v1.14: shared pinctrl algorithms backed by binary-extracted tables.
use rk3588_boot_support::{PinField,strided_pin_field};
use crate::stage9::{PULL_REGS,DRIVE_REGS,SCHMITT_REGS};
pub const fn pull_field(bank:u8,pin:u8)->Option<PinField>{strided_pin_field(PULL_REGS,bank as u32*32+pin as u32,pin,8,2)}
pub const fn drive_field(bank:u8,pin:u8)->Option<PinField>{strided_pin_field(DRIVE_REGS,bank as u32*32+pin as u32,pin,4,4)}
pub const fn schmitt_field(bank:u8,pin:u8)->Option<PinField>{strided_pin_field(SCHMITT_REGS,bank as u32*32+pin as u32,pin,8,1)}
#[cfg(test)]mod tests{use super::*;#[test]fn shared_fields(){assert_eq!(pull_field(0,13).unwrap().reg,0x4028);assert_eq!(drive_field(4,24).unwrap().reg,0xB098);}}
