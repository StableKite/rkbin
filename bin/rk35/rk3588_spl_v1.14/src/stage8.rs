//! Stage-8 semantic recovery for SPL v1.14.
//! The embedded build commit is public in rockchip-linux/u-boot: c28d9f4e21084689b4eca660e64abf59bc2144ff.

pub const LINK_BASE:u64=0x03FE_0000;
pub const VECTOR_BASE:u64=0x03FE_1800;
pub const PUBLIC_UBOOT_COMMIT:&str="c28d9f4e21084689b4eca660e64abf59bc2144ff";
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum VectorRegister{VbarEl1,VbarEl2,VbarEl3}
pub const fn vector_register_for_current_el(current_el:u64)->VectorRegister{match current_el{4=>VectorRegister::VbarEl1,8=>VectorRegister::VbarEl2,_=>VectorRegister::VbarEl3}}
#[cfg(test)]mod tests{use super::*;#[test]fn el_map(){assert_eq!(vector_register_for_current_el(4),VectorRegister::VbarEl1);assert_eq!(vector_register_for_current_el(8),VectorRegister::VbarEl2);assert_eq!(vector_register_for_current_el(12),VectorRegister::VbarEl3);}}
