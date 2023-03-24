use rust_mcd::core::Core;
use tricore_common::backtrace::{csa::SavedContext, pcxi::PCXI};

use crate::backtrace::csa::ContextLinkWordExt;

pub trait PCXIExt {
    fn walk_context<'a>(&self, core: &'a Core) -> ContextWalker<'a>;
}

impl PCXIExt for PCXI {
    fn walk_context<'a>(&self, core: &'a Core) -> ContextWalker<'a> {
        ContextWalker { pcxi: *self, core }
    }
}

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
                log::error!(
                    "Failed to obtain full list of context from device: {:?}",
                    err
                );
                None
            }
        }
    }
}
