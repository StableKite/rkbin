use rk3588_boot_support::bootstrap_layout_consistent;
pub const BOOT_COPY_BYTES:u64=crate::stage11::BOOT_HEADER.copy_end-crate::stage11::BOOT_HEADER.copy_dst;
pub const fn bootstrap_layout_valid()->bool{bootstrap_layout_consistent(crate::stage11::BOOT_HEADER)&&BOOT_COPY_BYTES==0xB878&&crate::stage11::POST_RELOCATION>=crate::stage11::BOOT_HEADER.copy_dst&&crate::stage11::POST_RELOCATION<crate::stage11::BOOT_HEADER.copy_end}
#[cfg(test)]mod tests{use super::*;#[test]fn layout(){assert!(bootstrap_layout_valid());assert_eq!(BOOT_COPY_BYTES,0xB878);}}
