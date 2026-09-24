//! Stage 16 shared composition boundaries.
use crate::BootstrapImageHeader;

/// Clock/reset operations remain a trait boundary until exact version-specific
/// CRU writes are frozen from the corresponding SPL image.
pub trait EarlyClockReset{type Error;fn prepare_clocks(&mut self)->Result<(),Self::Error>;fn release_resets(&mut self)->Result<(),Self::Error>;}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct BootstrapSpec{pub header:BootstrapImageHeader,pub post_relocation:u64,pub expected_copy_bytes:u64}
pub const fn bootstrap_spec_valid(s:BootstrapSpec)->bool{crate::bootstrap_layout_consistent(s.header)&&s.header.copy_end>=s.header.copy_dst&&(s.header.copy_end-s.header.copy_dst)==s.expected_copy_bytes&&s.post_relocation>=s.header.copy_dst&&s.post_relocation<s.header.copy_end}
#[cfg(test)]mod tests{use super::*;const H:BootstrapImageHeader=BootstrapImageHeader{vbar_el3:0,magic:0,reserved_330:0,mailbox_jump:0,mailbox_state:0,mailbox_entry:0,secondary_entry:0,secondary_state:0,copy_src:0x800,copy_dst:0x03001000,copy_end:0x03001010,opaque_378:0,marker_file_offset:0x810};#[test]fn spec(){let s=BootstrapSpec{header:H,post_relocation:0x03001004,expected_copy_bytes:0x10};assert!(bootstrap_spec_valid(s));}}
