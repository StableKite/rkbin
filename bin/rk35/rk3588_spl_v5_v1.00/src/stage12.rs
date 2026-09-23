use rk3588_boot_support::{Mmio32,apply_grf_field};use crate::stage9::{pull_field,drive_field,schmitt_field};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum PinctrlError{InvalidPin}
pub fn set_pull_raw<I:Mmio32>(io:&mut I,base:u64,bank:u8,pin:u8,value:u32)->Result<(),PinctrlError>{let f=pull_field(bank,pin).ok_or(PinctrlError::InvalidPin)?;apply_grf_field(io,base,f,value);Ok(())}
pub fn set_drive_raw<I:Mmio32>(io:&mut I,base:u64,bank:u8,pin:u8,value:u32)->Result<(),PinctrlError>{let f=drive_field(bank,pin).ok_or(PinctrlError::InvalidPin)?;apply_grf_field(io,base,f,value);Ok(())}
pub fn set_schmitt_raw<I:Mmio32>(io:&mut I,base:u64,bank:u8,pin:u8,value:u32)->Result<(),PinctrlError>{let f=schmitt_field(bank,pin).ok_or(PinctrlError::InvalidPin)?;apply_grf_field(io,base,f,value);Ok(())}
#[cfg(test)]mod tests{use super::*;struct M{a:u64,v:u32}impl Mmio32 for M{fn read32(&mut self,_:u64)->u32{0}fn write32(&mut self,a:u64,v:u32){self.a=a;self.v=v}}#[test]fn drive_tail(){let mut m=M{a:0,v:0};set_drive_raw(&mut m,0x1000,4,28,3).unwrap();assert_eq!(m.a,0xC09C);assert_eq!(m.v&0xffff,3);}}
