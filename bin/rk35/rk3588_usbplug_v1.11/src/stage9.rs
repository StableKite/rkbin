//! Stage-9 common early-bootstrap ABI recovered independently in this image.

pub const SCTLR_EL3_OR_MASK:u64=0x100A;
pub const SCR_EL3_OR_MASK:u64=8;
pub const BOOT_CPU_AFFINITY_MASK:u64=0xFFFF;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct SecondaryMailbox{pub state:u64,pub entry:u64,pub jump:u64}
pub const MAILBOX:SecondaryMailbox=SecondaryMailbox{state:0xFF00_0004,entry:0xFF00_0008,jump:0xFF00_001C};
pub const fn is_boot_cpu(mpidr_el1:u64)->bool{(mpidr_el1&BOOT_CPU_AFFINITY_MASK)==0}
pub const fn relocation_words(start:u64,end:u64)->Option<usize>{if end<start || ((end-start)&3)!=0{None}else{Some(((end-start)/4)as usize)}}
#[cfg(test)]mod tests{use super::*;#[test]fn cpu(){assert!(is_boot_cpu(0));assert!(!is_boot_cpu(1));}#[test]fn control_masks(){assert_eq!(SCTLR_EL3_OR_MASK,0x100A);assert_eq!(SCR_EL3_OR_MASK,8);}}
