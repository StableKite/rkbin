//! Stage-9 PCIe PHY lock/retry policy recovered from v3.20.

pub const PHY_LOCK_MASK:u32=0xF;
pub const PHY_LOCK_VALUE:u32=0xF;
pub const POLL_ITERATIONS:u32=500;
pub const POLL_DELAY_US:u32=10;
pub const RETRY_SCRATCH_MAGIC:u32=0x7072_7374;
pub const MAX_STORED_RETRY_BEFORE_STOP:u32=4;
pub const MAX_WARM_RESETS:u32=5;
pub const MAX_TOTAL_ATTEMPTS:u32=6;
pub const fn phy_locked(grf_904:u32)->bool{(grf_904&PHY_LOCK_MASK)==PHY_LOCK_VALUE}
pub const fn next_retry_count(stored:u32)->Option<u32>{if stored<=MAX_STORED_RETRY_BEFORE_STOP{Some(stored+1)}else{None}}
#[cfg(test)]mod tests{use super::*;#[test]fn locked(){assert!(phy_locked(0xF));assert!(phy_locked(0x10F));assert!(!phy_locked(0xE));}#[test]fn retries(){assert_eq!(next_retry_count(0),Some(1));assert_eq!(next_retry_count(4),Some(5));assert_eq!(next_retry_count(5),None);assert_eq!(MAX_TOTAL_ATTEMPTS,6);}}
