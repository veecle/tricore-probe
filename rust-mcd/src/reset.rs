use anyhow::Context;

use crate::{error::expect_error, mcd_bindings::mcd_rst_info_st, MCD_LIB};

use super::core::Core;

use std::{
    ffi::CStr,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct ResetInfo {
    inner: mcd_rst_info_st,
}

impl Display for ResetInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = unsafe { CStr::from_ptr(&self.inner.info_str[0] as *const i8) }
            .to_str()
            .unwrap();
        f.write_fmt(format_args!("Reset[{:?}]", info))
    }
}

impl From<mcd_rst_info_st> for ResetInfo {
    fn from(inner: mcd_rst_info_st) -> Self {
        ResetInfo { inner }
    }
}

#[derive(Clone, Copy)]
pub struct ResetClass<'a> {
    bit_set: u8,
    core: &'a Core<'a>,
}

impl<'a> Debug for ResetClass<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResetClass")
            .field("bit_set", &self.bit_set)
            .finish()
    }
}

impl<'a> ResetClass<'a> {
    /// MCD internal representation of this reset class
    pub(crate) fn as_vector(&self) -> u32 {
        1u32 << self.bit_set
    }

    /// Construct a reset class from a known reset class
    ///
    /// The user should make sure that the class also exists for the specified core,
    /// otherwise this might create errors later in the execution which are not
    /// easy to track down
    pub fn construct_reset_class(core: &'a Core<'a>, class: u8) -> ResetClass<'a> {
        ResetClass {
            bit_set: class,
            core,
        }
    }

    pub fn get_info(&self) -> anyhow::Result<ResetInfo> {
        let mut output = mcd_rst_info_st::default();
        let result =
            unsafe { MCD_LIB.mcd_qry_rst_class_info_f(self.core.core, self.bit_set, &mut output) };

        if result != 0 {
            Err(expect_error(Some(self.core))).with_context(|| "Library reported an error")
        } else {
            Ok(output.into())
        }
    }
}
