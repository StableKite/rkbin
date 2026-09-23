use rk3588_boot_support::{BootstrapRunError,ControlTransfer,El3BootstrapRuntime,complete_bootstrap};
pub fn run_and_dispatch<R:El3BootstrapRuntime,T:ControlTransfer>(rt:&mut R,sink:&mut T,marker:u32,mpidr:u64,secondary_spin_limit:u32)->Result<T::Output,BootstrapRunError>{let t=crate::stage12::run_entry(rt,marker,mpidr,secondary_spin_limit)?;Ok(complete_bootstrap(sink,t))}
#[cfg(test)]mod tests{#[test]fn target(){assert_eq!(crate::stage11::POST_RELOCATION,0x0300_5A44);}}
