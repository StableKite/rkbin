#![no_std]
//! AArch64 BL31 reconstruction project.
pub const REFERENCE_PATH: &str = "bin/rk35/rk3588_bl31_v1.56.elf";
pub const REFERENCE_SHA256: &str =
    "2864c2fc36333e03e7e3fe6459225d320d310850b4ac7620d1f3601efb7d4a9b";
pub const REFERENCE_SIZE: usize = 365344;
pub const OBSERVED_ENTRY: &str = "0x60000";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconstructionStatus {
    AnalysisPending,
    Recovered,
    HardwareTested,
}
pub const STATUS: ReconstructionStatus = ReconstructionStatus::AnalysisPending;
