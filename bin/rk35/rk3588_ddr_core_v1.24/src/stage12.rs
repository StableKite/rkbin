use rk3588_boot_support::DelayUs;
pub trait DebugUartIo:DelayUs{fn write32(&mut self,addr:u64,value:u32);}
pub const fn uart_divisor(baud:u32)->u32{match baud{115_200=>13,750_000=>2,_=>1}}
pub fn init_debug_uart<I:DebugUartIo>(io:&mut I,base:u64,baud:u32){io.write32(base+0x88,7);io.delay_us(1);io.write32(base+0x88,0);io.write32(base+0x0C,0x83);io.write32(base,uart_divisor(baud));io.write32(base+0x0C,3);io.write32(base+0x98,1)}
#[cfg(test)]mod tests{use super::*;struct U{n:u8,d:u32,div:u32}impl DelayUs for U{fn delay_us(&mut self,u:u32){self.d+=u}}impl DebugUartIo for U{fn write32(&mut self,a:u64,v:u32){self.n+=1;if a==0x1000{self.div=v}}}#[test]fn uart(){let mut u=U{n:0,d:0,div:0};init_debug_uart(&mut u,0x1000,750000);assert_eq!(u.n,6);assert_eq!(u.d,1);assert_eq!(u.div,2);}}
