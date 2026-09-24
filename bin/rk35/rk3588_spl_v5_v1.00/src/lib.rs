#![no_std]
//! AArch64 SPL v5 reconstruction project.
pub const REFERENCE_PATH: &str = "bin/rk35/rk3588_spl_v5_v1.00.bin";
pub const REFERENCE_SHA256: &str =
    "9adef27f338b202dd8affefd0a3f86d13123b470e9a0e98c04fc87c5b23438dc";
pub const REFERENCE_SIZE: usize = 288981;
pub const OBSERVED_ENTRY: &str = "raw offset 0";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconstructionStatus {
    AnalysisPending,
    Recovered,
    HardwareTested,
}
pub const STATUS: ReconstructionStatus = ReconstructionStatus::AnalysisPending;

pub mod stage8;

pub mod stage9;

pub mod stage10;

pub mod stage11;

pub mod stage12;

pub mod stage13;

pub mod stage14;

pub mod stage15;

pub mod stage16;

pub mod stage17;
