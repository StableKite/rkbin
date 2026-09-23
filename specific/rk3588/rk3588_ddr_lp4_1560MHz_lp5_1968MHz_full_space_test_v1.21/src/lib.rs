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
    lp4_mhz: 1560,
    lp5_mhz: 1968,
    eyescan: false,
    single_vdd2: false,
    diagnostic_mode: "full-space-test",
};
pub const REFERENCE_SHA256: &str =
    "780e9d32aa4c5e11fee4e00e3b53d6079dd9b404cc7f276faba3e8b0db5713cf";
pub const REFERENCE_SIZE: usize = 58928;
