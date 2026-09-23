use rk3588_boot_support::{BootstrapRunError,BootstrapTransfer,El3BootstrapRuntime,run_bootstrap_until_transfer};use crate::stage11::{BOOT_HEADER,POST_RELOCATION};
pub fn run_entry<R:El3BootstrapRuntime>(rt:&mut R,marker:u32,mpidr:u64,secondary_spin_limit:u32)->Result<BootstrapTransfer,BootstrapRunError>{run_bootstrap_until_transfer(rt,marker,mpidr,BOOT_HEADER,POST_RELOCATION,secondary_spin_limit)}
#[cfg(test)]mod tests{use super::*;#[test]fn target(){assert_eq!(POST_RELOCATION,0x0300_5A44);assert_eq!(BOOT_HEADER.vbar_el3,0x0300_B000);}}
