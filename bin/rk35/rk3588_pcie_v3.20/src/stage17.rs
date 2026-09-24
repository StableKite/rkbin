use crate::stage14::HotResetWait;
pub const REFERENCE_HOT_RESET_CYCLES:u8=4;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ReferenceHotResetDecision{Repeat{next_cycle:u8},StableDone,WarmResetFromL2}
/// Corrects the Stage-16 terminal normalization to the recovered reference flow:
/// observed hot reset repeats through cycle indices 0..3; L2 takes reset path;
/// timeout exits as stable completion.
pub const fn reference_hot_reset_decision(cycle:u8,wait:HotResetWait)->ReferenceHotResetDecision{match wait{HotResetWait::ForcedFromL2=>ReferenceHotResetDecision::WarmResetFromL2,HotResetWait::Timeout=>ReferenceHotResetDecision::StableDone,HotResetWait::Observed=>if cycle+1<REFERENCE_HOT_RESET_CYCLES{ReferenceHotResetDecision::Repeat{next_cycle:cycle+1}}else{ReferenceHotResetDecision::StableDone}}}
#[cfg(test)]mod tests{use super::*;#[test]fn four_cycles(){assert_eq!(reference_hot_reset_decision(0,HotResetWait::Observed),ReferenceHotResetDecision::Repeat{next_cycle:1});assert_eq!(reference_hot_reset_decision(2,HotResetWait::Observed),ReferenceHotResetDecision::Repeat{next_cycle:3});assert_eq!(reference_hot_reset_decision(3,HotResetWait::Observed),ReferenceHotResetDecision::StableDone);}#[test]fn l2_and_timeout(){assert_eq!(reference_hot_reset_decision(0,HotResetWait::ForcedFromL2),ReferenceHotResetDecision::WarmResetFromL2);assert_eq!(reference_hot_reset_decision(0,HotResetWait::Timeout),ReferenceHotResetDecision::StableDone);}}
