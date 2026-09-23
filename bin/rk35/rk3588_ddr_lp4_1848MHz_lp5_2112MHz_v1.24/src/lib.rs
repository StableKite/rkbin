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
    lp4_mhz: 1848,
    lp5_mhz: 2112,
    eyescan: false,
    single_vdd2: false,
    diagnostic_mode: "normal",
};
pub const REFERENCE_SHA256: &str =
    "7d114862d4958841886afc97cf82affb2bd88bc3b968f636e7e9ed898410c353";
pub const REFERENCE_SIZE: usize = 79443;
