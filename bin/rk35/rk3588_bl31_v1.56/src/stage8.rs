//! Stage-8 semantic recovery for RK3588 BL31 v1.56.

pub const EL3_ENTRY:u64=0x0006_0000;
pub const DATA_START:u64=0x0008_0000;
pub const PMU_SRAM_TEXT_START:u64=0xFF10_0000;
pub const DDR_M0_IMAGE_START:u64=0x000F_0000;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Bl31Region{El3Code,DataOrBss,DdrM0Image,PmuSramCode,Other}

pub const fn classify_address(a:u64)->Bl31Region{
    if a>=0x0006_0000 && a<0x0008_0000 {Bl31Region::El3Code}
    else if a>=0x0008_0000 && a<0x000D_1000 {Bl31Region::DataOrBss}
    else if a>=0x000F_0000 && a<0x000F_6000 {Bl31Region::DdrM0Image}
    else if a>=0xFF10_0000 && a<0xFF10_9000 {Bl31Region::PmuSramCode}
    else {Bl31Region::Other}
}

#[cfg(test)]mod tests{use super::*;#[test]fn regions(){assert_eq!(classify_address(EL3_ENTRY),Bl31Region::El3Code);assert_eq!(classify_address(PMU_SRAM_TEXT_START),Bl31Region::PmuSramCode);}}
