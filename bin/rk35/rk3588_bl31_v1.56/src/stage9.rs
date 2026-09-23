//! Stage-9 behavior recovered directly from BL31 v1.56.

pub const DDR_ROUND_RATE_UNSUPPORTED:i32=-2;
pub const PMUMCU_IMAGE_BASE:u64=0xFF08_0000;
pub const PMUMCU_TAG_ADDR:u64=0xFF08_0140;
pub const PMUMCU_ACK_ADDR:u64=0xFF08_0148;
pub const PMUMCU_TAG:u32=0x5F4D_4355;
pub const PMUMCU_ACK:u32=0xDEAD_BEAF;
pub const PMUMCU_COPY_DEST:u64=0x0000_1000;
pub const PMUMCU_COPY_LEN:usize=0x17C00;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PmuMcuImageDescriptor{pub source:u64,pub destination:u64,pub length:usize}
pub const PMUMCU_IMAGE:PmuMcuImageDescriptor=PmuMcuImageDescriptor{source:PMUMCU_IMAGE_BASE,destination:PMUMCU_COPY_DEST,length:PMUMCU_COPY_LEN};
pub const fn pmumcu_image_ready(tag:u32,ack:u32)->bool{tag==PMUMCU_TAG && ack==PMUMCU_ACK}
pub const fn ddr_round_rate_default()->i32{DDR_ROUND_RATE_UNSUPPORTED}

#[cfg(test)]mod tests{use super::*;#[test]fn mcu_descriptor(){assert!(pmumcu_image_ready(PMUMCU_TAG,PMUMCU_ACK));assert!(!pmumcu_image_ready(0,PMUMCU_ACK));assert_eq!(PMUMCU_IMAGE.length,0x17C00);}#[test]fn round_rate(){assert_eq!(ddr_round_rate_default(),-2);}}
