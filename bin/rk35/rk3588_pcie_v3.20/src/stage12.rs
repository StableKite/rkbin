use rk3588_boot_support::Mmio32;
pub fn apply_phy_mode_prelude<I:Mmio32>(io:&mut I,mode:u8){if mode==2{io.write32(0xFD5F_0004,0x0FF0_0110);io.write32(0xFD5F_0008,0xF0F0_1010);io.write32(0xFD7C_0838,0x2100_0000);io.write32(0xFD7C_0A38,0x2100_0000)}}
#[cfg(test)]mod tests{use super::*;struct M{n:u8,last:u32}impl Mmio32 for M{fn read32(&mut self,_:u64)->u32{0}fn write32(&mut self,_:u64,v:u32){self.n+=1;self.last=v}}#[test]fn mode2(){let mut m=M{n:0,last:0};apply_phy_mode_prelude(&mut m,1);assert_eq!(m.n,0);apply_phy_mode_prelude(&mut m,2);assert_eq!(m.n,4);assert_eq!(m.last,0x21000000);}}
