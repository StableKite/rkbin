//! Stage-10 SPL-v5: shared pinctrl algorithms backed by binary-extracted tables.
use rk3588_boot_support::{PinField,direct_pin_field};
use crate::stage9::{PULL_REGS,DRIVE_REGS,SCHMITT_REGS};
pub const fn pull_field(bank:u8,pin:u8)->Option<PinField>{direct_pin_field(PULL_REGS,bank as u32*32+pin as u32,pin,8,2)}
pub const fn drive_field(bank:u8,pin:u8)->Option<PinField>{direct_pin_field(DRIVE_REGS,bank as u32*32+pin as u32,pin,4,4)}
pub const fn schmitt_field(bank:u8,pin:u8)->Option<PinField>{direct_pin_field(SCHMITT_REGS,bank as u32*32+pin as u32,pin,8,1)}
#[cfg(test)]mod tests{use super::*;#[test]fn v5_fields(){assert_eq!(drive_field(4,28).unwrap().reg,0xB09C);assert_eq!(pull_field(2,6).unwrap().reg,0xA120);}}
