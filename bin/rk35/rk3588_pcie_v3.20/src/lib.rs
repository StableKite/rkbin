#![no_std]
//! AArch64 PCIe EP loader reconstruction project.
pub const REFERENCE_PATH: &str = "bin/rk35/rk3588_pcie_v3.20.bin";
pub const REFERENCE_SHA256: &str =
    "a63ade015a1969f7f50be362e0809b0b13d160e4e8d0582fdda4a2d1126aa0ce";
pub const REFERENCE_SIZE: usize = 6560;
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
