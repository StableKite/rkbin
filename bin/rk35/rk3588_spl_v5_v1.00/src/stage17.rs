use rk3588_boot_support::Mmio32;use rk3588_boot_support::stage17::{EarlySocTailHook,apply_common_early_soc_tail};
pub fn apply_recovered_common_early_tail<I:Mmio32,H:EarlySocTailHook>(io:&mut I,hook:&mut H){apply_common_early_soc_tail(io,hook)}
#[cfg(test)]mod tests{use super::*;struct M(u8);impl Mmio32 for M{fn read32(&mut self,_:u64)->u32{0}fn write32(&mut self,_:u64,_:u32){self.0+=1}}struct H;impl EarlySocTailHook for H{fn between_common_tail_writes(&mut self){}}#[test]fn common_tail(){let(mut m,mut h)=(M(0),H);apply_recovered_common_early_tail(&mut m,&mut h);assert_eq!(m.0,7);}}
