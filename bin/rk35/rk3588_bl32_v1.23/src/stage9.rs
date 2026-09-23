//! Stage-9 pure OP-TEE/Rockchip OTP behavior recovered from BL32 v1.23.

pub const OTP_WINDOW_BYTES:usize=1024;
pub const CPU_ID_OTP_OFFSET:usize=7;
pub const CPU_ID_BYTES:usize=16;
pub const POPCOUNT_FUNCTION:u64=0x0840_FEA0;
pub const OTP_SECURITY_LEVEL_FUNCTION:u64=0x0840_F550;
pub const RK_OTP_GET_CPU_ID_FUNCTION:u64=0x0840_F7B4;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum OtpSecurityError{UnsupportedMode}

pub const fn popcount32(mut value:u32)->u32{let mut n=0;while value!=0{n+=value&1;value>>=1;}n}

/// Exact decision behavior of the recovered `otp_value_to_security_level` body.
/// Unsupported modes call an assertion path in the reference image; this pure
/// helper returns an error so callers can choose their terminal policy.
pub const fn otp_value_to_security_level(bytes:[u8;2],mode:u8)->Result<u8,OtpSecurityError>{
 match mode{
  1=>{if popcount32((bytes[0]&0xF0)as u32)>1{Ok(2)}else if popcount32((bytes[0]&0x0F)as u32)>1{Ok(1)}else{Ok(0)}},
  2=>{if popcount32(bytes[1]as u32)>3{Ok(2)}else if popcount32(bytes[0]as u32)>3{Ok(1)}else{Ok(0)}},
  _=>Err(OtpSecurityError::UnsupportedMode),
 }
}

pub const fn otp_read_range_valid(offset:usize,len:usize)->bool{len>=1 && len<=OTP_WINDOW_BYTES && offset<OTP_WINDOW_BYTES && offset.saturating_add(len)<=OTP_WINDOW_BYTES}

pub fn copy_cpu_id(window:&[u8;CPU_ID_BYTES],out:&mut [u8])->usize{let n=if out.len()<CPU_ID_BYTES{out.len()}else{CPU_ID_BYTES};out[..n].copy_from_slice(&window[..n]);n}

#[cfg(test)]mod tests{use super::*;#[test]fn popcount(){assert_eq!(popcount32(0xF0),4);assert_eq!(popcount32(0),0);}#[test]fn security(){assert_eq!(otp_value_to_security_level([0x03,0],1),Ok(1));assert_eq!(otp_value_to_security_level([0x30,0],1),Ok(2));assert_eq!(otp_value_to_security_level([0x0F,0x07],2),Ok(1));assert_eq!(otp_value_to_security_level([0,0x0F],2),Ok(2));assert_eq!(otp_value_to_security_level([0,0],9),Err(OtpSecurityError::UnsupportedMode));}#[test]fn bounds(){assert!(otp_read_range_valid(7,16));assert!(!otp_read_range_valid(1020,8));assert!(!otp_read_range_valid(0,0));}#[test]fn cpu_id_copy(){let src=[0x5Au8;16];let mut o=[0u8;5];assert_eq!(copy_cpu_id(&src,&mut o),5);assert_eq!(o,[0x5A;5]);}}
