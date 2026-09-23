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
    eyescan: true,
    single_vdd2: false,
    diagnostic_mode: "normal",
};
pub const REFERENCE_SHA256: &str =
    "93a50424431840010a26607a6fcadbc49652b8dddca539e55ad5f82e4cfbeda8";
pub const REFERENCE_SIZE: usize = 79443;
