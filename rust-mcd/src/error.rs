use std::{ffi::CStr, fmt::Display};

use anyhow::Context;

use crate::{
    mcd_bindings::{mcd_core_st, mcd_error_info_st, MCD_ERR_NONE},
    raw::McdReturnError,
};

use super::{core::Core, MCD_LIB};

/// Obtain a more specific error description of the latest error
///
/// A core may be specified to get the last error that happened for the operation
/// on this core.
pub fn get_error(core: Option<&'_ Core<'_>>) -> Option<Error> {
    let mut output = mcd_error_info_st::default();
    let core_reference = core
        .map(|core| core.core.as_ptr() as *const mcd_core_st)
        .unwrap_or(std::ptr::null());
    unsafe { MCD_LIB.mcd_qry_error_info_f(core_reference, &mut output) };
    if output.return_status != MCD_ERR_NONE as u32 {
        Some(output.into())
    } else {
        None
    }
}

/// Extension trait for [Result<R, McdReturnError>]
pub trait McdError<R> {
    /// Add error information, parsed from the MCD library
    fn add_mcd_error_info<'a>(self, core: Option<&'a Core<'a>>) -> anyhow::Result<R>;
}

impl<R> McdError<R> for Result<R, McdReturnError> {
    fn add_mcd_error_info<'a>(self, core: Option<&'a Core<'a>>) -> anyhow::Result<R> {
        self.with_context(|| expect_error(core))
    }
}

/// Like [get_error], but it will panic if the library does not report an error
pub fn expect_error(core: Option<&'_ Core<'_>>) -> Error {
    get_error(core).expect("expected error, but library reported none")
}

#[derive(Debug)]
pub struct Error {
    inner: mcd_error_info_st,
}

impl Error {
    pub fn error_code(&self) -> McdErrorCode {
        McdErrorCode::from_code(self.inner.error_code)
    }

    pub fn event_error_code(&self) -> EventError {
        EventError::from_library_code(self.inner.error_events)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // SAFETY:
        // u8 and i8 share the same memory layout, so slices can be transmuted safely.
        let error_string = unsafe { std::mem::transmute::<&[i8], &[u8]>(&self.inner.error_str) };
        let info = CStr::from_bytes_until_nul(error_string)
            .unwrap()
            .to_str()
            .unwrap();

        f.write_fmt(format_args!(
            "{info}, error_code = {:?}, event_code = {:?}",
            self.error_code(),
            self.event_error_code()
        ))
    }
}

impl From<mcd_error_info_st> for Error {
    fn from(inner: mcd_error_info_st) -> Self {
        Error { inner }
    }
}

/// See the original header files for [crate::mcd_bindings::mcd_error_event_et]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventError {
    /// See [crate::mcd_bindings::MCD_ERR_EVT_RESET]
    Reset,
    /// See [crate::mcd_bindings::MCD_ERR_EVT_PWRDN]
    PowerDown,
    /// See [crate::mcd_bindings::MCD_ERR_EVT_HWFAILURE]
    HardwareFailure,
}

impl EventError {
    fn from_library_code(code: u32) -> EventError {
        match code as i32 {
            crate::mcd_bindings::MCD_ERR_EVT_RESET => Self::Reset,
            crate::mcd_bindings::MCD_ERR_EVT_PWRDN => Self::PowerDown,
            crate::mcd_bindings::MCD_ERR_EVT_HWFAILURE => Self::HardwareFailure,
            _ => panic!("Unsupported library event error code {:x}", code),
        }
    }
}

/// See the original header files for [crate::mcd_bindings::mcd_error_code_et]
#[derive(Debug)]
pub enum McdErrorCode {
    /// No error.
    McdErrNone,
    /// Called function is not implemented.
    McdErrFnUnimplemented,
    /// MCD API not correctly used.
    McdErrUsage,
    /// Passed invalid parameter.
    McdErrParam,
    /// Server connection error.
    McdErrConnection,
    /// Function call timed out.
    McdErrTimedOut,
    /// General error.
    McdErrGeneral,
    /// String to return is longer than the provided character array.
    McdErrResultTooLong,
    /// Could not start server.
    McdErrCouldNotStartServer,
    /// Server is locked.
    McdErrServerLocked,
    /// No memory spaces defined.
    McdErrNoMemSpaces,
    /// No memory blocks defined for the requested memory space.
    McdErrNoMemBlocks,
    /// No memory space with requested ID exists.
    McdErrMemSpaceId,
    /// No register groups defined.
    McdErrNoRegGroups,
    /// No register group with requested ID exists.
    McdErrRegGroupId,
    /// Register is not a compound register.
    McdErrRegNotCompound,
    /// Error retrieving overlay information.
    McdErrOverlays,
    /// Cannot access device (power-down, reset active, etc.).
    McdErrDeviceAccess,
    /// Device is locked.
    McdErrDeviceLocked,
    /// Read transaction of transaction list has failed.
    McdErrTxlistRead,
    /// Write transaction of transaction list has failed.
    McdErrTxlistWrite,
    /// Other error (no R/W failure) for a transaction of the transaction list.
    McdErrTxlistTx,
    /// Requested channel type is not supported by the implementation.
    McdErrChlTypeNotSupported,
    /// Addressed target does not support communication channels.
    McdErrChlTargetNotSupported,
    /// Channel setup is invalid or contains unsupported attributes.
    McdErrChlSetup,
    /// Sending or receiving of the last message has failed.
    McdErrChlMessageFailed,
    /// Trigger could not be created.
    McdErrTrigCreate,
    /// Error during trigger information access.
    McdErrTrigAccess,
    /// Library reported unknown error code
    Unknown(u32),
}
impl McdErrorCode {
    pub fn from_code(code: u32) -> Self {
        match code {
            0 => Self::McdErrNone,
            256 => Self::McdErrFnUnimplemented,
            257 => Self::McdErrUsage,
            258 => Self::McdErrParam,
            512 => Self::McdErrConnection,
            513 => Self::McdErrTimedOut,
            3840 => Self::McdErrGeneral,
            4096 => Self::McdErrResultTooLong,
            4352 => Self::McdErrCouldNotStartServer,
            4353 => Self::McdErrServerLocked,
            5121 => Self::McdErrNoMemSpaces,
            5122 => Self::McdErrNoMemBlocks,
            5136 => Self::McdErrMemSpaceId,
            5184 => Self::McdErrNoRegGroups,
            5185 => Self::McdErrRegGroupId,
            5186 => Self::McdErrRegNotCompound,
            5376 => Self::McdErrOverlays,
            6400 => Self::McdErrDeviceAccess,
            6401 => Self::McdErrDeviceLocked,
            8448 => Self::McdErrTxlistRead,
            8449 => Self::McdErrTxlistWrite,
            8450 => Self::McdErrTxlistTx,
            12544 => Self::McdErrChlTypeNotSupported,
            12545 => Self::McdErrChlTargetNotSupported,
            12546 => Self::McdErrChlSetup,
            12608 => Self::McdErrChlMessageFailed,
            12800 => Self::McdErrTrigCreate,
            12801 => Self::McdErrTrigAccess,
            code => Self::Unknown(code),
        }
    }
}
