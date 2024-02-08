//! Rusty wrappers around raw MCD bindings
//!
//! This should get rid of most C-like calls, while the overall MCD semantic still remains the same.
//! We refrain from introducing new types in this module if possible.

use crate::mcd_bindings::{
    mcd_return_et, MCD_RET_ACT_AGAIN, MCD_RET_ACT_CUSTOM_HI, MCD_RET_ACT_CUSTOM_LO,
    MCD_RET_ACT_HANDLE_ERROR, MCD_RET_ACT_HANDLE_EVENT, MCD_RET_ACT_NONE, MCD_RET_ACT_RESERVED_HI,
    MCD_RET_ACT_RESERVED_LO,
};

/// Simple wrapper around MCD API return codes, see [mcd_return_et]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum McdReturnError {
    #[error("Try to call the function again")]
    TryAgain,
    #[error("Handle the event or events")]
    HandleEvent,
    #[error("Handle the error")]
    HandleError,
    #[error("Action reserved for future API use")]
    Reserved(u32),
    #[error("Range: For user defined actions")]
    Custom(u32),
}

impl McdReturnError {
    /// Parse the return code from an MCD API call into a [Result]
    pub fn from_library_call(code: mcd_return_et) -> Result<(), Self> {
        match code {
            _ if code == MCD_RET_ACT_NONE as u32 => Ok(()),
            _ if code == MCD_RET_ACT_AGAIN as u32 => Err(Self::TryAgain),
            _ if code == MCD_RET_ACT_HANDLE_EVENT as u32 => Err(Self::HandleEvent),
            _ if code == MCD_RET_ACT_HANDLE_ERROR as u32 => Err(Self::HandleError),
            _ if code >= MCD_RET_ACT_RESERVED_LO as u32
                && code < MCD_RET_ACT_RESERVED_HI as u32 =>
            {
                Err(Self::Reserved(code))
            }
            _ if code >= MCD_RET_ACT_CUSTOM_LO as u32 && code < MCD_RET_ACT_CUSTOM_HI as u32 => {
                Err(Self::Custom(code))
            }
            _ => panic!("Unknown mcd_return_et code {code}"),
        }
    }
}

/// Utility for implementations in this module
/// 
/// Will automatically type the return value from the library call.
macro_rules! mcd_call {
    (unsafe { $rec:ident.$function_name:ident($($argument: expr),* $(,)?)  }) => {
        {
            let result = unsafe { $rec.$function_name($($argument),*)};
            let result = $crate::raw::McdReturnError::from_library_call(result);

            if let Err(err) = &result {
                log::warn!("{} failed with error {err}", stringify!($function_name))
            }

            result
        }
    };
}

pub mod query_core;
pub mod query_server;
pub mod query_system;
