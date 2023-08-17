use rust_mcd::core::Core;
use tricore_common::backtrace::{csa::SavedContext, pcxi::PCXI};

use crate::backtrace::csa::ContextLinkWordExt;

/// Extension trait to iterate over the CSA link chain
///
/// Most notably implemented for [PCXI]
pub trait PCXIExt {
    /// Allows to iterate over all contexts using the specified core access
    fn walk_context<'a>(&self, core: &'a Core) -> ContextWalker<'a>;
}

impl PCXIExt for PCXI {
    fn walk_context<'a>(&self, core: &'a Core) -> ContextWalker<'a> {
        ContextWalker { pcxi: *self, core }
    }
}

/// Iterator over all contexts in the link chain
pub struct ContextWalker<'a> {
    pcxi: PCXI,
    core: &'a Core<'a>,
}

impl<'a> Iterator for ContextWalker<'a> {
    type Item = SavedContext;

    fn next(&mut self) -> Option<Self::Item> {
        let link_word = self.pcxi.get_context()?;

        match link_word.load(self.core) {
            Ok(ctx) => {
                self.pcxi = ctx.pcxi();
                Some(ctx)
            }
            Err(err) => {
                // We are bound to the Iterator trait so we cannot return an
                // error here
                log::error!(
                    "Failed to obtain full list of context from device: {:?}",
                    err
                );
                None
            }
        }
    }
}
