use bitfield_struct::bitfield;

use super::csa::ContextLinkWord;

#[bitfield(u32)]
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
}
