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
    single_vdd2: true,
    diagnostic_mode: "normal",
};
pub const REFERENCE_SHA256: &str =
    "966ce9b7167c7432a225506df92d024cba4b7b15b45635557f45f09c9a69119b";
pub const REFERENCE_SIZE: usize = 79443;
