#![no_std]
//! AArch64 BL32 reconstruction project.
pub const REFERENCE_PATH: &str = "bin/rk35/rk3588_bl32_v1.23.bin";
pub const REFERENCE_SHA256: &str =
    "cb4ec2d0d8986b22cc55dbcf57c499f10fcca124bcd1e9a5d62cbf9ce0f40636";
pub const REFERENCE_SIZE: usize = 530920;
pub const OBSERVED_ENTRY: &str = "unknown";
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
