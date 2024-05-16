use bitfield_struct::bitfield;
use rust_mcd::core::Core;

use super::csa::{ContextLinkWord, SavedContext};

/// Models the PCXI register of the tricore architecture.
#[bitfield(u32)]
pub struct PCXI {
    pub previous_context_pointer: u16,
    #[bits(4)]
    pub previous_segment_address: u8,
    pub is_upper: bool,
    #[bits(2)]
    _reserved: u8,
    #[bits(9)]
    _not_implemented: u16,
}

impl PCXI {
    /// This function extracts required bits from the register to form the current
    /// context link word, also known as 'previous context link word'.
    pub fn get_context(&self) -> Option<ContextLinkWord> {
        if self.previous_context_pointer() == 0 && self.previous_segment_address() == 0 {
            return None;
        }

        Some(ContextLinkWord {
            segment_address: self.previous_segment_address(),
            context_offset: self.previous_context_pointer(),
            is_upper: self.is_upper(),
        })
    }
    /// Allows to iterate over all contexts in the CSA link chain using the
    /// specified core access.
    pub fn walk_context<'a>(&self, core: &'a Core) -> ContextWalker<'a> {
        ContextWalker { pcxi: *self, core }
    }
}

/// Iterator over all contexts in the link chain.
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
