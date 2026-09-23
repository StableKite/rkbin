//! Stage-10 PCIe: typed retry decision built on the shared boot-support policy.
use rk3588_boot_support::{RetryAction,bounded_retry};
use crate::stage9::{phy_locked,MAX_STORED_RETRY_BEFORE_STOP};
pub const RETRY_MAGIC_ADDR:u64=0xFF01_0000;
pub const RETRY_COUNT_ADDR:u64=0xFF01_0004;
pub const PHY_STATUS_ADDR:u64=0xFD5B_8904;
pub const fn retry_action(phy_status:u32,stored_count:u32)->RetryAction{bounded_retry(phy_locked(phy_status),stored_count,MAX_STORED_RETRY_BEFORE_STOP)}
#[cfg(test)]mod tests{use super::*;#[test]fn action(){assert_eq!(retry_action(0xF,4),RetryAction::Ready);assert_eq!(retry_action(0,4),RetryAction::WarmReset{next_count:5});assert_eq!(retry_action(0,5),RetryAction::GiveUp);}}
