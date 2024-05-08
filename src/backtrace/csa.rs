//! Defines structure for CSA's.
//!
//! See also https://www.infineon.com/dgdl/tc1_6__architecture_vol1.pdf?fileId=db3a3043372d5cc801373b0f374d5d67#G8.6699641.


use anyhow::Context;
use rust_mcd::core::Core;
use crate::backtrace::pcxi::PCXI;

/// Represents a link word that points to a CSA.
///
/// See https://www.infineon.com/dgdl/tc1_6__architecture_vol1.pdf?fileId=db3a3043372d5cc801373b0f374d5d67#G8.6699687.
#[derive(Debug, Clone, Copy)]
pub struct ContextLinkWord {
    pub segment_address: u8,
    pub context_offset: u16,
    pub is_upper: bool,
}

impl ContextLinkWord {
    /// Returns the effective (absolute) address of the CSA segment.
    pub fn get_context_address(&self) -> u32 {
        ((self.segment_address as u32) << 28) + ((self.context_offset as u32) << 6)
    }
}

/// A saved context
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SavedContext {
    Upper(UpperContext),
    Lower(LowerContext),
}

impl SavedContext {
    /// Returns the [PCXI] register value stored in this context.
    ///
    /// This function is available on the type itself since both the upper context as well
    /// as the lower context hold a stored pcxi register.
    pub fn pcxi(&self) -> PCXI {
        match self {
            SavedContext::Upper(c) => c.pcxi,
            SavedContext::Lower(c) => c.pcxi,
        }
    }

    /// Returns the return address stored in this context.
    ///
    /// Useful for providing additional information like the source address.
    pub fn return_address(&self) -> u32 {
        match self {
            SavedContext::Upper(c) => c.a11,
            SavedContext::Lower(c) => c.a11,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub struct UpperContext {
    pub pcxi: PCXI,
    pub psw: u32,
    pub a10: u32,
    pub a11: u32,
    pub d8: u32,
    pub d9: u32,
    pub d10: u32,
    pub d11: u32,
    pub a12: u32,
    pub a13: u32,
    pub a14: u32,
    pub a15: u32,
    pub d12: u32,
    pub d13: u32,
    pub d14: u32,
    pub d15: u32,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub struct LowerContext {
    pub pcxi: PCXI,
    a11: u32,
    a2: u32,
    a3: u32,
    d0: u32,
    d1: u32,
    d2: u32,
    d3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    d4: u32,
    d5: u32,
    d6: u32,
    d7: u32,
}

pub trait ContextLinkWordExt {
    fn load(&self, core: &Core) -> anyhow::Result<SavedContext>;
}

impl ContextLinkWordExt for ContextLinkWord {
    fn load(&self, core: &Core) -> anyhow::Result<SavedContext> {
        log::trace!(
            "Loading stored context from {:#8X}",
            self.get_context_address()
        );
        if self.is_upper {
            let mut upper = UpperContext::default();
            let bytes = core
                .read_bytes(
                    self.get_context_address() as u64,
                    core::mem::size_of::<UpperContext>(),
                )
                .with_context(|| "Cannot read saved context from memory")?;
            assert_eq!(bytes.len(), core::mem::size_of::<UpperContext>());
            unsafe {
                core::ptr::copy(bytes.as_ptr() as *const UpperContext, &mut upper, 1);
            }
            Ok(SavedContext::Upper(upper))
        } else {
            let mut lower = LowerContext::default();
            let bytes = core
                .read_bytes(
                    self.get_context_address() as u64,
                    core::mem::size_of::<LowerContext>(),
                )
                .with_context(|| "Cannot read saved context from memory")?;
            assert_eq!(bytes.len(), core::mem::size_of::<LowerContext>());
            unsafe {
                core::ptr::copy(bytes.as_ptr() as *const LowerContext, &mut lower, 1);
            }
            Ok(SavedContext::Lower(lower))
        }
    }
}
