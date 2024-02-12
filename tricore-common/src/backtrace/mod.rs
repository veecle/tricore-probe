//! This module defines a stacktrace for the tricore architecture that is obtained
//! by traversing the CSA link chain.
use self::csa::{SavedContext, UpperContext};

pub mod csa;
pub mod pcxi;

/// Models a stacktrace, e.g. a snapshot of call frames at runtime.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Stacktrace {
    pub current_pc: u32,
    pub current_upper: UpperContext,
    pub stack_frames: Vec<SavedContext>,
}
