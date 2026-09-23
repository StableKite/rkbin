#![no_std]
//! RK3588 USB plug helper reconstruction project.
pub const REFERENCE_PATH: &str = "bin/rk35/rk3588_usbplug_v1.11.bin";
pub const REFERENCE_SHA256: &str =
    "fa38c130f3dd9160895543f0ef83833612963cc428c11f38ec9ecd66ae64d753";
pub const REFERENCE_SIZE: usize = 90268;
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
