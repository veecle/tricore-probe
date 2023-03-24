use self::csa::{SavedContext, UpperContext};

pub mod csa;
pub mod pcxi;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BackTrace {
    pub current_pc: u32,
    pub current_upper: UpperContext,
    pub stack_frames: Vec<SavedContext>,
}
