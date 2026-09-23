//! Stage-10 USBPlug: image-specific bootstrap plan using shared early-boot support.
use rk3588_boot_support::{SecondaryMailbox,MAILBOX,is_boot_cpu,relocation_words,runtime_address};
pub const COPY_END:u64=0x0301_6898;
pub const VBAR_EL3:u64=0x0301_3000;
pub const POST_RELOCATION_ENTRY:u64=0x0300_59FC;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct BootstrapPlan{pub boot_cpu:bool,pub mailbox:SecondaryMailbox,pub vbar:u64,pub post_relocation:u64,pub words:Option<usize>}
pub const fn bootstrap_plan(mpidr:u64,copy_start:u64)->BootstrapPlan{BootstrapPlan{boot_cpu:is_boot_cpu(mpidr),mailbox:MAILBOX,vbar:VBAR_EL3,post_relocation:POST_RELOCATION_ENTRY,words:relocation_words(copy_start,COPY_END)}}
pub const fn runtime_from_file(offset:u64)->Option<u64>{runtime_address(offset)}
#[cfg(test)]mod tests{use super::*;#[test]fn plan(){let p=bootstrap_plan(0,0x0300_1000);assert!(p.boot_cpu);assert_eq!(p.vbar,VBAR_EL3);assert_eq!(runtime_from_file(0x800),Some(0x0300_1000));}}
