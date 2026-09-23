//! Stage-8 recovered PCIe EP boot configuration fields.

pub const CONFIG_MAGIC:u32=0x6569_6370; // little-endian "pcie"
pub const RELEASE_VERSION:&str="V3.20";
pub const RELEASE_DATE:&str="Feb  4 2026";
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct PcieBootConfig{pub phy_mode:u8,pub generation:u8,pub lanes:u8}
pub const fn decode_config(magic:u32,word:u32)->Option<PcieBootConfig>{if magic!=CONFIG_MAGIC{return None}Some(PcieBootConfig{phy_mode:(word&7)as u8,generation:((word>>8)&7)as u8,lanes:((word>>12)&7)as u8})}
#[cfg(test)]mod tests{use super::*;#[test]fn config(){let c=decode_config(CONFIG_MAGIC,3|(4<<8)|(2<<12)).unwrap();assert_eq!(c,PcieBootConfig{phy_mode:3,generation:4,lanes:2});assert!(decode_config(0,0).is_none());}}
