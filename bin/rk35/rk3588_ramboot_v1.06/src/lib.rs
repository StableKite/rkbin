#![no_std]
//! RK3588 RAM boot helper reconstruction project.
pub const REFERENCE_PATH: &str = "bin/rk35/rk3588_ramboot_v1.06.bin";
pub const REFERENCE_SHA256: &str =
    "10bbe6e80740cb0b9511cc84a9347cb90dd1915744b6511336d4fb75029c3691";
pub const REFERENCE_SIZE: usize = 49276;
pub const OBSERVED_ENTRY: &str = "container/header at offset 0";
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
