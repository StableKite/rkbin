#![no_std]
pub use rk3588_ddr_core_v1_24 as core;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Profile {
    pub lp4_mhz: u16,
    pub lp5_mhz: u16,
    pub eyescan: bool,
    pub single_vdd2: bool,
    pub diagnostic_mode: &'static str,
}
pub const PROFILE: Profile = Profile {
    lp4_mhz: 2112,
    lp5_mhz: 2400,
    eyescan: false,
    single_vdd2: false,
    diagnostic_mode: "max-freq-scan",
};
pub const REFERENCE_SHA256: &str =
    "eebfabf3aee2b355c680b046217b48691810631d431efa5d4a0dd41eaf805950";
pub const REFERENCE_SIZE: usize = 63376;
