use std::{ffi::CStr, fmt::Display};

use crate::mcd_bindings::{mcd_core_st, mcd_error_info_st, MCD_ERR_NONE};

use super::{core::Core, MCD_LIB};

/// Obtain a more specific error description of the latest error
///
/// A core may be specified to get the last error that happened for the operation
/// on this core.
pub fn get_error(core: Option<&'_ Core<'_>>) -> Option<Error> {
    let mut output = mcd_error_info_st::default();
    let core_reference = core
        .map(|core| core.core as *const mcd_core_st)
        .unwrap_or(std::ptr::null());
    unsafe { MCD_LIB.mcd_qry_error_info_f(core_reference, &mut output) };
    if output.error_code != MCD_ERR_NONE as u32 {
        Some(output.into())
    } else {
        None
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

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = unsafe { CStr::from_ptr(&self.inner.error_str[0] as *const i8) }
            .to_str()
            .unwrap();
        f.write_str(info)
    }
}

impl From<mcd_error_info_st> for Error {
    fn from(inner: mcd_error_info_st) -> Self {
        Error { inner }
    }
}
