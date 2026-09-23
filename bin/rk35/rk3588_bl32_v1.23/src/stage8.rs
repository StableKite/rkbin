//! Stage-8 semantic recovery for RK3588 BL32 v1.23 (OP-TEE lineage).

pub const LOAD_ADDRESS:u64=0x0840_0000;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct OpteeMemoryLayout{pub tee_os:u32,pub ta_ram:u32,pub shared:u32}
pub const MEMORY_LAYOUT:OpteeMemoryLayout=OpteeMemoryLayout{tee_os:0x0020_0000,ta_ram:0x00C0_0000,shared:0x0020_0000};
impl OpteeMemoryLayout{pub const fn total(self)->u32{self.tee_os+self.ta_ram+self.shared}}
#[cfg(test)]mod tests{use super::*;#[test]fn layout(){assert_eq!(MEMORY_LAYOUT.total(),0x0100_0000);assert_eq!(LOAD_ADDRESS,0x0840_0000);}}
