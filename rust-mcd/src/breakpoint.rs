//! Abstracts over breakpoints for a [crate::core::Core]
//!
//! TODO The implementation is very rudimentary and possibly wrong
use crate::mcd_bindings::{
    mcd_addr_st, mcd_trig_simple_core_st, MCD_TRIG_ACTION_DBG_DEBUG, MCD_TRIG_OPT_DEFAULT,
    MCD_TRIG_TYPE_IP, MCD_TRIG_TYPE_RW,
};

pub enum TriggerType {
    RW,
    IP,
}

impl TriggerType {
    fn as_type(&self) -> u32 {
        match self {
            TriggerType::RW => MCD_TRIG_TYPE_RW as u32,
            TriggerType::IP => MCD_TRIG_TYPE_IP as u32,
        }
    }
}

impl mcd_trig_simple_core_st {
    pub(crate) fn create_trigger(trigger_type: TriggerType, address: u64, size: u64) -> Self {
        // For the Aurix Lite Kit v2 connected over micro-USB, MCD_TRIG_OPT_DEFAULT
        // as the option flag and MCD_TRIG_ACTION_DBG_DEBUG as the action flag are
        // the only supported flags (? or are we missing something here)
        Self {
            struct_size: core::mem::size_of::<mcd_trig_simple_core_st>() as u32,
            type_: trigger_type.as_type(),
            option: MCD_TRIG_OPT_DEFAULT as u32,
            action: MCD_TRIG_ACTION_DBG_DEBUG as u32,
            action_param: Default::default(),
            modified: Default::default(),
            state_mask: Default::default(),
            addr_start: mcd_addr_st {
                address,
                mem_space_id: 0,
                addr_space_id: 0,
                addr_space_type: 0,
            },
            addr_range: size,
        }
    }
}
