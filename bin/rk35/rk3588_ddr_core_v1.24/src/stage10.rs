//! Stage-10 DDR core memory primitives recovered from v1.24 at 0x820/0x83c.
/// # Safety
/// `dst` must be valid for `len` writable bytes.
pub unsafe fn memset_u8(dst:*mut u8,value:u8,len:usize)->*mut u8{let out=dst;let mut i=0usize;while i<len{unsafe{core::ptr::write(dst.add(i),value)};i+=1;}out}
/// # Safety
/// `dst` and `src` must be valid for `len` bytes and non-overlapping.
pub unsafe fn memcpy_u8(dst:*mut u8,src:*const u8,len:usize)->*mut u8{let out=dst;if dst as *const u8==src{return out;}let aligned=((dst as usize)|(src as usize))&7==0;let mut copied=0usize;if aligned{let words=len>>3;let mut i=0usize;while i<words{unsafe{let v=core::ptr::read(src.add(i*8)as*const u64);core::ptr::write(dst.add(i*8)as*mut u64,v);}i+=1;}copied=words*8;}let mut i=copied;while i<len{unsafe{core::ptr::write(dst.add(i),core::ptr::read(src.add(i)))};i+=1;}out}
#[cfg(test)]mod tests{use super::*;#[test]fn primitives(){let src=[1u8,2,3,4,5,6,7,8,9];let mut dst=[0u8;9];unsafe{memcpy_u8(dst.as_mut_ptr(),src.as_ptr(),9);}assert_eq!(dst,src);unsafe{memset_u8(dst.as_mut_ptr(),0x5A,4);}assert_eq!(&dst[..4],&[0x5A;4]);}}
