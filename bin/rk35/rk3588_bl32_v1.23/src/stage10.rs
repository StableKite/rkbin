//! Stage-10 BL32: typed OTP window and CPU-ID extraction.
use crate::stage9::{otp_read_range_valid,CPU_ID_BYTES,CPU_ID_OTP_OFFSET};
pub const OTP_WINDOW_ADDRESS:u64=0x0848_CEBA;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct OtpWindow{pub address:u64,pub bytes:usize}
pub const OTP_WINDOW:OtpWindow=OtpWindow{address:OTP_WINDOW_ADDRESS,bytes:1024};
pub fn read_from_snapshot<'a>(snapshot:&'a[u8;1024],offset:usize,len:usize)->Option<&'a[u8]>{if !otp_read_range_valid(offset,len){None}else{Some(&snapshot[offset..offset+len])}}
pub fn cpu_id_from_snapshot(snapshot:&[u8;1024])->[u8;CPU_ID_BYTES]{let mut out=[0u8;CPU_ID_BYTES];out.copy_from_slice(&snapshot[CPU_ID_OTP_OFFSET..CPU_ID_OTP_OFFSET+CPU_ID_BYTES]);out}
#[cfg(test)]mod tests{use super::*;#[test]fn window(){assert_eq!(OTP_WINDOW.address,0x0848_CEBA);let mut x=[0u8;1024];for i in 0..CPU_ID_BYTES{x[CPU_ID_OTP_OFFSET+i]=i as u8;}assert_eq!(cpu_id_from_snapshot(&x)[15],15);assert!(read_from_snapshot(&x,1020,8).is_none());}}
