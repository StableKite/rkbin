#![no_std]
//! Shared RK3588 early-boot support recovered independently from multiple rkbin images.

pub const STAGING_HEADER_BYTES:u64=0x800;
pub const PAYLOAD_RUNTIME_START:u64=0x0300_1000;
pub const RUNTIME_MINUS_FILE:u64=PAYLOAD_RUNTIME_START-STAGING_HEADER_BYTES;
pub const SCTLR_EL3_OR_MASK:u64=0x100A;
pub const SCR_EL3_OR_MASK:u64=8;
pub const BOOT_CPU_AFFINITY_MASK:u64=0xFFFF;

pub const fn runtime_address(file_offset:u64)->Option<u64>{if file_offset<STAGING_HEADER_BYTES{None}else{Some(file_offset+RUNTIME_MINUS_FILE)}}
pub const fn is_boot_cpu(mpidr_el1:u64)->bool{(mpidr_el1&BOOT_CPU_AFFINITY_MASK)==0}
pub const fn relocation_words(start:u64,end:u64)->Option<usize>{if end<start||((end-start)&3)!=0{None}else{Some(((end-start)/4)as usize)}}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct SecondaryMailbox{pub state:u64,pub entry:u64,pub jump:u64}
pub const MAILBOX:SecondaryMailbox=SecondaryMailbox{state:0xFF00_0004,entry:0xFF00_0008,jump:0xFF00_001C};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct PinReg{pub first_pin:u32,pub reg:u32}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct PinField{pub reg:u32,pub bit:u8,pub width:u8}
pub const fn select_pin_reg(table:&[PinReg],global_pin:u32)->Option<PinReg>{let mut i=table.len();while i>0{i-=1;if global_pin>=table[i].first_pin{return Some(table[i]);}}None}
pub const fn strided_pin_field(table:&[PinReg],global_pin:u32,pin_in_bank:u8,pins_per_reg:u32,width:u8)->Option<PinField>{match select_pin_reg(table,global_pin){Some(e)=>Some(PinField{reg:e.reg+((global_pin-e.first_pin)/pins_per_reg)*4,bit:(pin_in_bank as u32%pins_per_reg)as u8*width,width}),None=>None}}
pub const fn direct_pin_field(table:&[PinReg],global_pin:u32,pin_in_bank:u8,pins_per_reg:u32,width:u8)->Option<PinField>{match select_pin_reg(table,global_pin){Some(e)=>Some(PinField{reg:e.reg,bit:(pin_in_bank as u32%pins_per_reg)as u8*width,width}),None=>None}}
pub const fn grf_write_word(field:PinField,value:u32)->u32{let mask=if field.width>=16{0xFFFF}else{(1u32<<field.width)-1}<<field.bit;(mask<<16)|((value<<field.bit)&mask)}

/// # Safety
/// `src..src+words` and `dst..end` must be valid u32 ranges. They must not overlap
/// in a way forbidden by the forward-copy behavior observed in the reference code.
pub unsafe fn copy_words_forward(src:*const u32,dst:*mut u32,end:*mut u32)->usize{
    let mut s=src; let mut d=dst; let mut n=0usize;
    while (d as usize)<(end as usize){unsafe{let v=core::ptr::read_volatile(s);core::ptr::write_volatile(d,v);s=s.add(1);d=d.add(1);}n+=1;}n
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum RetryAction{Ready,WarmReset{next_count:u32},GiveUp}
pub const fn bounded_retry(ready:bool,stored:u32,max_stored_before_stop:u32)->RetryAction{if ready{RetryAction::Ready}else if stored<=max_stored_before_stop{RetryAction::WarmReset{next_count:stored+1}}else{RetryAction::GiveUp}}

#[cfg(test)]extern crate std;
#[cfg(test)]mod tests{use super::*;#[test]fn relocation(){assert_eq!(runtime_address(0x800),Some(0x0300_1000));assert_eq!(runtime_address(0x7ff),None);assert_eq!(relocation_words(0x1000,0x1010),Some(4));}#[test]fn pin(){let t=[PinReg{first_pin:0,reg:0x20},PinReg{first_pin:8,reg:0x24}];let f=strided_pin_field(&t,13,13,8,2).unwrap();assert_eq!(f,PinField{reg:0x24,bit:10,width:2});assert_eq!(grf_write_word(PinField{reg:0,bit:4,width:4},3),0x00F0_0030);}#[test]fn retry(){assert_eq!(bounded_retry(false,4,4),RetryAction::WarmReset{next_count:5});assert_eq!(bounded_retry(false,5,4),RetryAction::GiveUp);}}

/// Stage 11: minimal MMIO/delay interfaces used by recovered state machines.
pub trait Mmio32{fn read32(&mut self,addr:u64)->u32;fn write32(&mut self,addr:u64,value:u32);}
pub trait DelayUs{fn delay_us(&mut self,us:u32);}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct MmioWrite{pub addr:u64,pub value:u32}

/// Qword header beginning at file offset 0x320 in USBPlug/RAMBOOT.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct BootstrapImageHeader{
    pub vbar_el3:u64,pub magic:u64,pub reserved_330:u64,pub mailbox_jump:u64,pub mailbox_state:u64,pub mailbox_entry:u64,
    pub secondary_entry:u64,pub secondary_state:u64,pub copy_src:u64,pub copy_dst:u64,pub copy_end:u64,pub opaque_378:u64,pub marker_file_offset:u64,
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum BootstrapEntryAction{
    MarkerMismatch,
    BootCpu{vbar_el3:u64,copy_src:u64,copy_dst:u64,copy_end:u64,post_relocation:u64},
    SecondaryCpu{entry_mailbox:u64,state_mailbox:u64,jump_mailbox:u64,entry_value:u32,state_value:u32},
}
pub const fn bootstrap_entry_action(marker:u32,mpidr_el1:u64,h:BootstrapImageHeader,post_relocation:u64)->BootstrapEntryAction{
    if marker as u64!=h.magic{return BootstrapEntryAction::MarkerMismatch}
    if is_boot_cpu(mpidr_el1){BootstrapEntryAction::BootCpu{vbar_el3:h.vbar_el3,copy_src:h.copy_src,copy_dst:h.copy_dst,copy_end:h.copy_end,post_relocation}}
    else{BootstrapEntryAction::SecondaryCpu{entry_mailbox:h.mailbox_entry,state_mailbox:h.mailbox_state,jump_mailbox:h.mailbox_jump,entry_value:h.secondary_entry as u32,state_value:h.secondary_state as u32}}
}

#[cfg(test)]mod stage11_tests{use super::*;const H:BootstrapImageHeader=BootstrapImageHeader{vbar_el3:0x300B000,magic:0x4B415351,reserved_330:0x30000,mailbox_jump:0xFF00001C,mailbox_state:0xFF000004,mailbox_entry:0xFF000008,secondary_entry:0x26C,secondary_state:0xDEADBEAF,copy_src:0x800,copy_dst:0x3001000,copy_end:0x300C878,opaque_378:0x336BB48,marker_file_offset:0xC078};#[test]fn bootstrap(){assert!(matches!(bootstrap_entry_action(0,H.magic,H,0),BootstrapEntryAction::MarkerMismatch));assert!(matches!(bootstrap_entry_action(H.magic as u32,0,H,0x3005A44),BootstrapEntryAction::BootCpu{..}));assert!(matches!(bootstrap_entry_action(H.magic as u32,1,H,0),BootstrapEntryAction::SecondaryCpu{entry_value:0x26C,state_value:0xDEADBEAF,..}));}}

/// Stage 12: barriers/system-register hooks used to execute the recovered USBPlug/RAMBOOT
/// bootstrap path up to, but not including, the final control transfer.
pub trait BarrierOps{fn dsb(&mut self);fn isb(&mut self);}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum El3SysReg{SctlrEl3,VbarEl3,ScrEl3}
pub trait El3BootstrapRuntime:Mmio32+BarrierOps{fn read_sys(&mut self,reg:El3SysReg)->u64;fn write_sys(&mut self,reg:El3SysReg,value:u64);fn unmask_irq(&mut self);fn sev(&mut self);fn copy_words(&mut self,src:u64,dst:u64,end:u64);}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum BootstrapTransfer{Direct(u64),Indirect(u64)}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum BootstrapRunError{MarkerMismatch,InvalidCopyRange,SecondaryTimeout}
pub fn run_bootstrap_until_transfer<R:El3BootstrapRuntime>(rt:&mut R,marker:u32,mpidr_el1:u64,h:BootstrapImageHeader,post_relocation:u64,secondary_spin_limit:u32)->Result<BootstrapTransfer,BootstrapRunError>{if marker as u64!=h.magic{return Err(BootstrapRunError::MarkerMismatch)}if is_boot_cpu(mpidr_el1){if relocation_words(h.copy_dst,h.copy_end).is_none(){return Err(BootstrapRunError::InvalidCopyRange)}rt.write32(h.mailbox_state,1);let s=rt.read_sys(El3SysReg::SctlrEl3);rt.write_sys(El3SysReg::SctlrEl3,s|SCTLR_EL3_OR_MASK);rt.isb();rt.write_sys(El3SysReg::VbarEl3,h.vbar_el3);rt.isb();let scr=rt.read_sys(El3SysReg::ScrEl3);rt.write_sys(El3SysReg::ScrEl3,scr|SCR_EL3_OR_MASK);rt.unmask_irq();rt.copy_words(h.copy_src,h.copy_dst,h.copy_end);Ok(BootstrapTransfer::Direct(post_relocation))}else{rt.write32(h.mailbox_entry,h.secondary_entry as u32);rt.write32(h.mailbox_state,h.secondary_state as u32);rt.dsb();rt.sev();let mut left=secondary_spin_limit;while rt.read32(h.mailbox_state)!=1{if left==0{return Err(BootstrapRunError::SecondaryTimeout)}left-=1}Ok(BootstrapTransfer::Indirect(h.mailbox_jump))}}
pub fn apply_grf_field<I:Mmio32>(io:&mut I,regmap_base:u64,field:PinField,value:u32){io.write32(regmap_base+field.reg as u64,grf_write_word(field,value));}
#[cfg(test)]mod stage12_tests{use super::*;struct R{state:u32,sctlr:u64,scr:u64,vbar:u64,copies:u8}impl Mmio32 for R{fn read32(&mut self,a:u64)->u32{if a==0xFF000004{self.state}else{0}}fn write32(&mut self,a:u64,v:u32){if a==0xFF000004{self.state=v}}}impl BarrierOps for R{fn dsb(&mut self){}fn isb(&mut self){}}impl El3BootstrapRuntime for R{fn read_sys(&mut self,r:El3SysReg)->u64{match r{El3SysReg::SctlrEl3=>self.sctlr,El3SysReg::ScrEl3=>self.scr,El3SysReg::VbarEl3=>self.vbar}}fn write_sys(&mut self,r:El3SysReg,v:u64){match r{El3SysReg::SctlrEl3=>self.sctlr=v,El3SysReg::ScrEl3=>self.scr=v,El3SysReg::VbarEl3=>self.vbar=v}}fn unmask_irq(&mut self){}fn sev(&mut self){}fn copy_words(&mut self,_:u64,_:u64,_:u64){self.copies+=1}}const H:BootstrapImageHeader=BootstrapImageHeader{vbar_el3:0x300B000,magic:0x4B415351,reserved_330:0,mailbox_jump:0xFF00001C,mailbox_state:0xFF000004,mailbox_entry:0xFF000008,secondary_entry:0x26C,secondary_state:0xDEADBEAF,copy_src:0x800,copy_dst:0x3001000,copy_end:0x3001010,opaque_378:0,marker_file_offset:0};#[test]fn boot_run(){let mut r=R{state:0,sctlr:0,scr:0,vbar:0,copies:0};assert_eq!(run_bootstrap_until_transfer(&mut r,H.magic as u32,0,H,0x3005A44,10),Ok(BootstrapTransfer::Direct(0x3005A44)));assert_eq!(r.copies,1);assert_eq!(r.sctlr&SCTLR_EL3_OR_MASK,SCTLR_EL3_OR_MASK);assert_eq!(r.vbar,H.vbar_el3);}}
