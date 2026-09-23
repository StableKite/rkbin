#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ZqCalibrationWarning{VssqOrFloating,VddqShortLike}
pub const fn classify_zq_calibration(raw:u32)->Option<ZqCalibrationWarning>{match(raw>>3)&3{1=>Some(ZqCalibrationWarning::VssqOrFloating),2=>Some(ZqCalibrationWarning::VddqShortLike),_=>None}}
#[cfg(test)]mod tests{use super::*;#[test]fn zq(){assert_eq!(classify_zq_calibration(8),Some(ZqCalibrationWarning::VssqOrFloating));assert_eq!(classify_zq_calibration(16),Some(ZqCalibrationWarning::VddqShortLike));}}
