//! Stage-8 semantic recovery for SPL-v5 v1.00.
//! The embedded build commit is public in rockchip-linux/u-boot: 0aa962ed3ab0eddecd44804605cff1adaddd5ac1.

pub const LINK_BASE:u64=0x03FE_0000;
pub const PUBLIC_UBOOT_COMMIT:&str="0aa962ed3ab0eddecd44804605cff1adaddd5ac1";
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ExceptionLevel{El1,El2,El3,Other(u8)}
pub const fn decode_current_el(raw:u64)->ExceptionLevel{match raw{4=>ExceptionLevel::El1,8=>ExceptionLevel::El2,12=>ExceptionLevel::El3,x=>ExceptionLevel::Other(x as u8)}}
#[cfg(test)]mod tests{use super::*;#[test]fn els(){assert_eq!(decode_current_el(12),ExceptionLevel::El3);assert_eq!(decode_current_el(8),ExceptionLevel::El2);}}
