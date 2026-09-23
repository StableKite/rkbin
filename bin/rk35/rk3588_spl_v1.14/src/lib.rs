#![no_std]
//! AArch64 SPL reconstruction project.
pub const REFERENCE_PATH: &str = "bin/rk35/rk3588_spl_v1.14.bin";
pub const REFERENCE_SHA256: &str =
    "b811b90b6847dc251838e82036c328f00eaacefe94659432265a0ccb0fc8f4e5";
pub const REFERENCE_SIZE: usize = 318282;
pub const OBSERVED_ENTRY: &str = "raw offset 0";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconstructionStatus {
    AnalysisPending,
    Recovered,
    HardwareTested,
}
pub const STATUS: ReconstructionStatus = ReconstructionStatus::AnalysisPending;
