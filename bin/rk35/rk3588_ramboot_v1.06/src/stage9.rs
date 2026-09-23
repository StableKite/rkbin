//! Stage-9 common early-bootstrap ABI recovered independently in this image.

pub const SCTLR_EL3_OR_MASK:u64=0x100A;
pub const SCR_EL3_OR_MASK:u64=8;
pub const BOOT_CPU_AFFINITY_MASK:u64=0xFFFF;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct SecondaryMailbox{pub state:u64,pub entry:u64,pub jump:u64}
pub const MAILBOX:SecondaryMailbox=SecondaryMailbox{state:0xFF00_0004,entry:0xFF00_0008,jump:0xFF00_001C};
pub const fn is_boot_cpu(mpidr_el1:u64)->bool{(mpidr_el1&BOOT_CPU_AFFINITY_MASK)==0}
pub const fn relocation_words(start:u64,end:u64)->Option<usize>{if end<start || ((end-start)&3)!=0{None}else{Some(((end-start)/4)as usize)}}
#[cfg(test)]mod tests{use super::*;#[test]fn cpu(){assert!(is_boot_cpu(0));assert!(!is_boot_cpu(1));}#[test]fn control_masks(){assert_eq!(SCTLR_EL3_OR_MASK,0x100A);assert_eq!(SCR_EL3_OR_MASK,8);}}

#[repr(C)]#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct El3ReturnContext{pub reserved0:u64,pub elr_el3:u64,pub spsr_el3:u64}
pub const fn return_context_requests_el3(ctx:&El3ReturnContext)->bool{((ctx.spsr_el3>>2)&3)==3}
#[cfg(test)]mod return_tests{use super::*;#[test]fn mode(){let c=El3ReturnContext{reserved0:0,elr_el3:1,spsr_el3:12};assert!(return_context_requests_el3(&c));}}
