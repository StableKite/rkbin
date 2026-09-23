//! Stage-10 BL31: PMU-MCU wait policy and default DDR SMC behavior.
pub const PMU_PWR_GATE_STATUS:u64=0xFD7F_0A00;
pub const PMU_PWR_GATE_ACTIVE:u32=0x2000;
pub const PMU_MCU_STATUS:u64=0xFD58_A060;
pub const PMU_MCU_PROGRESS_MASK:u32=0x680;
pub const PMU_MCU_IDLE_WAIT_ITERATIONS:u32=100_000;
pub const PMU_MCU_WAKE_WAIT_ITERATIONS:u32=500_000;
pub const DDR_SMC_UNHANDLED:i64=-2;
pub const fn pmumcu_idle_fast_path(power_status:u32)->bool{power_status&PMU_PWR_GATE_ACTIVE!=0}
pub const fn pmumcu_progress_seen(status:u32)->bool{status&PMU_MCU_PROGRESS_MASK!=0}
pub const fn ddr_smc_default_result()->i64{DDR_SMC_UNHANDLED}
#[cfg(test)]mod tests{use super::*;#[test]fn status(){assert!(pmumcu_idle_fast_path(0x2000));assert!(!pmumcu_idle_fast_path(0));assert!(pmumcu_progress_seen(0x80));assert!(pmumcu_progress_seen(0x600));assert_eq!(ddr_smc_default_result(),-2);}}
