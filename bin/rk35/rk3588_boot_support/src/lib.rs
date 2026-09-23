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
