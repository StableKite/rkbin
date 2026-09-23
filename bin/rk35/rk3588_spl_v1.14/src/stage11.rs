use rk3588_boot_support::MmioWrite;
pub const RESET_FIRST_OFFSET:u64=0xC08;pub const RESET_SECOND_OFFSET:u64=0xC0C;pub const SYSRESET_IN_PROGRESS:i32=-115;pub const SYSRESET_UNSUPPORTED:i32=-93;
pub const fn sysreset_plan(cru_base:u64,code:u32)->Result<MmioWrite,i32>{if code<2{Ok(MmioWrite{addr:cru_base+RESET_FIRST_OFFSET,value:0xFDB9})}else{Err(SYSRESET_UNSUPPORTED)}}
#[cfg(test)]mod tests{use super::*;#[test]fn binary_exact(){assert_eq!(sysreset_plan(0x1000,0).unwrap(),MmioWrite{addr:0x1C08,value:0xFDB9});assert_eq!(sysreset_plan(0x1000,1).unwrap().addr,0x1C08);assert_eq!(sysreset_plan(0,2),Err(SYSRESET_UNSUPPORTED));}}
