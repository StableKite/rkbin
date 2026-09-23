//! Stage-8 recovered bootstrap/relocation behavior for USBPlug v1.11.

pub const STAGING_HEADER_BYTES:usize=0x800;
pub const PAYLOAD_RUNTIME_START:u64=0x0300_1000;
pub const RUNTIME_MINUS_FILE:u64=0x0300_0800;
pub const IMAGE_MAGIC:u32=0x4B41_5351;
pub const SECONDARY_HANDSHAKE_MAGIC:u32=0xDEAD_BEAF;
pub const SECONDARY_STATE:u64=0xFF00_0004;
pub const SECONDARY_ENTRY:u64=0xFF00_0008;
pub const SECONDARY_JUMP:u64=0xFF00_001C;

pub const fn runtime_address(file_offset:u64)->Option<u64>{if file_offset>=STAGING_HEADER_BYTES as u64{Some(file_offset+RUNTIME_MINUS_FILE)}else{None}}

/// Exact behavior of the early forward word-copy loop at file offset 0x2fc.
/// # Safety
/// The source and destination ranges must be valid for `words` u32 accesses.
pub unsafe fn copy_words_forward(src:*const u32,dst:*mut u32,words:usize){let mut i=0;while i<words{unsafe{core::ptr::write_volatile(dst.add(i),core::ptr::read_volatile(src.add(i))) }i+=1;}}

pub const COPY_END:u64=0x0301_6898;
pub const VBAR_EL3:u64=0x0301_3000;
pub const POST_RELOCATION_BRANCH:u64=0x0300_59FC;
#[cfg(test)]mod tests{use super::*;#[test]fn map(){assert_eq!(runtime_address(0x800),Some(0x0300_1000));assert_eq!(runtime_address(0x51fc),Some(POST_RELOCATION_BRANCH));}}
