use anyhow::Context;
use rust_mcd::core::Core;
use tricore_common::backtrace::csa::{ContextLinkWord, LowerContext, SavedContext, UpperContext};

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
