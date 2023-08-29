use std::{
    cell::Cell,
    ffi::{c_void, CStr},
};

use anyhow::Context;

use super::{registers::RegisterGroups, reset::ResetClass, MCD_LIB};

use crate::{
    breakpoint::TriggerType,
    error::{expect_error, EventError},
    mcd_bindings::{
        mcd_core_con_info_st, mcd_core_event_et, mcd_core_st, mcd_core_state_et, mcd_core_state_st,
        mcd_trig_set_state_st, mcd_trig_simple_core_st, mcd_trig_state_st, mcd_tx_st,
        mcd_txlist_st, MCD_CORE_EVENT_CHL_PENDING, MCD_CORE_EVENT_MEMORY_CHANGE,
        MCD_CORE_EVENT_REGISTER_CHANGE, MCD_CORE_EVENT_STOPPED, MCD_CORE_EVENT_TRACE_CHANGE,
        MCD_CORE_EVENT_TRIGGER_CHANGE, MCD_CORE_STATE_CUSTOM_HI, MCD_CORE_STATE_CUSTOM_LO,
        MCD_CORE_STATE_DEBUG, MCD_CORE_STATE_HALTED, MCD_CORE_STATE_RUNNING,
        MCD_CORE_STATE_UNKNOWN, MCD_CORE_STEP_TYPE_INSTR, TRUE,
    },
    transaction::{create_transaction, Type},
};

#[derive(Debug)]
pub struct Core<'a> {
    pub core: &'a mcd_core_st,
    _core_connection: &'a mcd_core_con_info_st,
    payload_size: Cell<Option<u32>>,
}

impl<'a> Core<'a> {
    pub fn new(core: &'a mcd_core_st, core_connection: &'a mcd_core_con_info_st) -> Self {
        Core {
            core,
            _core_connection: core_connection,
            payload_size: Cell::new(None),
        }
    }

    pub fn reset(&self, reset_type: ResetClass, halt_after_reset: bool) -> anyhow::Result<()> {
        let reset_vector = reset_type.as_vector();
        let rst_and_halt = if halt_after_reset { 1 } else { 0 };
        let result = unsafe { MCD_LIB.mcd_rst_f(self.core, reset_vector, rst_and_halt) };
        if result != 0 {
            Err(expect_error(Some(self))).with_context(|| "Library reported an error")
        } else {
            Ok(())
        }
    }

    pub fn get_reset_classes(&self) -> anyhow::Result<impl Iterator<Item = ResetClass>> {
        let mut reset_classes = 0;
        let result = unsafe { MCD_LIB.mcd_qry_rst_classes_f(self.core, &mut reset_classes) };
        if result != 0 {
            return Err(expect_error(Some(self)))
                .with_context(|| "Could not obtain a list of available reset classes");
        }

        Ok((0..32)
            .filter(move |bit| (reset_classes & (1 << *bit)) != 0)
            .map(|bit_set| ResetClass::construct_reset_class(self, bit_set)))
    }
    /// Query the state of the core
    /// 
    /// If specified this function will exit gracefully with the [Option::None] value
    /// when an expected event happens.
    pub fn attempt_query_state(&self, tolerate_events: EventError) -> anyhow::Result<Option<CoreInfo>> {
        let mut output = mcd_core_state_st::default();
        let result = unsafe { MCD_LIB.mcd_qry_state_f(self.core, &mut output) };

        if result != 0 {
            let error = expect_error(Some(self));
            
            if error.event_error_code().intersects(tolerate_events) {
                return Ok(None)
            } else {
                return Err(error).context("Could not query device state")
            }
        }

        Ok(Some(output.into()))
    }

    /// Like [Self::attempt_query_state], but will never exit gracefully
    pub fn query_state(&self) -> anyhow::Result<CoreInfo> {
        self.attempt_query_state(EventError::empty()).map(|o| o.unwrap())
    }

    fn query_payload_size(&self) -> u32 {
        if let Some(payload) = self.payload_size.get() {
            return payload;
        }

        let mut max_payload = 0;
        let result = unsafe { MCD_LIB.mcd_qry_max_payload_size_f(self.core, &mut max_payload) };
        assert_eq!(result, 0);
        self.payload_size.replace(Some(max_payload));
        log::trace!("Maximum payload is {}", max_payload);
        max_payload
    }

    pub fn read_bytes(&self, mut addr: u64, mut length: usize) -> anyhow::Result<Vec<u8>> {
        let payload_length = self.query_payload_size();

        // TODO split the request into multiple transactions
        assert!(payload_length >= length as u32);

        let mut buffer: Vec<u8> = (0..length).map(|_| 0).collect();

        // TODO this logic might be wrong, check the [crate::transaction] module
        while length > 0 {
            let mut transaction = create_transaction(addr, Type::Read, &mut buffer);

            let mut transaction_list = mcd_txlist_st {
                tx: &mut transaction as *mut mcd_tx_st,
                num_tx: 1,
                num_tx_ok: 0,
            };
            let result = unsafe { MCD_LIB.mcd_execute_txlist_f(self.core, &mut transaction_list) };
            if result != 0 {
                return Err(expect_error(Some(self)))
                    .with_context(|| "Internal MCD library eror while trying to read data");
            }

            if transaction_list.num_tx_ok == 1 {
                break;
            }

            addr += transaction.num_bytes_ok as u64;
            length -= transaction.num_bytes_ok as usize;
        }

        Ok(buffer)
    }

    pub fn write(&self, mut address: u64, mut data: Vec<u8>) -> anyhow::Result<()> {
        let payload_length = self.query_payload_size();
        assert!(payload_length >= data.len() as u32);

        while !data.is_empty() {
            let mut transaction = create_transaction(address, Type::Write, &mut data);

            let mut transaction_list = mcd_txlist_st {
                tx: (&mut transaction) as *mut mcd_tx_st,
                num_tx: 1,
                num_tx_ok: 0,
            };

            let result = unsafe { MCD_LIB.mcd_execute_txlist_f(self.core, &mut transaction_list) };

            if result != 0 {
                return Err(expect_error(Some(self)))
                    .with_context(|| "Internal MCD library eror while trying to write data");
            }

            if transaction_list.num_tx_ok == 1 {
                break;
            }

            address += transaction.num_bytes as u64;
            data = data[(transaction.num_bytes as usize)..].into();
        }

        drop(data);

        Ok(())
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let result = unsafe { MCD_LIB.mcd_run_f(self.core, 0) };
        if result != 0 {
            Err(expect_error(Some(self))).with_context(|| "Internal library reported an error")
        } else {
            Ok(())
        }
    }

    pub fn step(&self) -> anyhow::Result<()> {
        let step_type = MCD_CORE_STEP_TYPE_INSTR as u32;

        let result = unsafe { MCD_LIB.mcd_step_f(self.core, 0, step_type, 1) };

        if result != 0 {
            Err(expect_error(Some(self))).with_context(|| "Internal library reported an error")
        } else {
            Ok(())
        }
    }

    pub fn create_breakpoint(
        &self,
        trigger_type: TriggerType,
        address: u64,
        size: u64,
    ) -> anyhow::Result<Trigger> {
        let mut trigger = mcd_trig_simple_core_st::create_trigger(trigger_type, address, size);
        let mut trigger_id = 0;

        let result = unsafe {
            MCD_LIB.mcd_create_trig_f(
                self.core,
                &mut trigger as *mut mcd_trig_simple_core_st as *mut c_void,
                &mut trigger_id,
            )
        };

        if result != 0 {
            return Err(expect_error(Some(self))).with_context(|| "Library reported an error");
        }

        log::trace!("trigger is modified: {:?}", trigger.modified == TRUE);

        Ok(Trigger {
            core: self,
            trigger_id,
        })
    }

    pub fn download_triggers(&self) {
        let _state = self.sample_triggers();

        let result = unsafe { MCD_LIB.mcd_activate_trig_set_f(self.core) };

        assert_eq!(result, 0);
    }

    pub fn sample_triggers(&self) -> TriggerSetState {
        let mut state = mcd_trig_set_state_st::default();

        let result = unsafe { MCD_LIB.mcd_qry_trig_set_state_f(self.core, &mut state) };
        assert_eq!(result, 0);

        state.into()
    }

    pub fn register_groups(&self) -> anyhow::Result<RegisterGroups> {
        RegisterGroups::from_core(self)
    }
}

#[derive(Debug)]
pub struct TriggerSetState {
    pub is_active: bool,
}

impl From<mcd_trig_set_state_st> for TriggerSetState {
    fn from(value: mcd_trig_set_state_st) -> Self {
        TriggerSetState {
            is_active: value.active == TRUE,
        }
    }
}

#[derive(Debug)]
pub struct TriggerState {
    pub active: bool,
    pub captured: Option<bool>,
    pub trigger_count: Option<u64>,
}

impl From<mcd_trig_state_st> for TriggerState {
    fn from(value: mcd_trig_state_st) -> Self {
        let captured = if value.captured_valid == 0 {
            None
        } else {
            Some(value.captured != 0)
        };
        let trigger_count = if value.count_valid == 0 {
            None
        } else {
            Some(value.count_value)
        };
        TriggerState {
            active: value.active != 0,
            captured,
            trigger_count,
        }
    }
}

pub struct Trigger<'a> {
    core: &'a Core<'a>,
    trigger_id: u32,
}

impl<'a> Trigger<'a> {
    pub fn get_state(&self) -> anyhow::Result<TriggerState> {
        let mut state_output = mcd_trig_state_st::default();
        let result = unsafe {
            MCD_LIB.mcd_qry_trig_state_f(self.core.core, self.trigger_id, &mut state_output)
        };
        if result != 0 {
            return Err(expect_error(Some(self.core)))
                .with_context(|| "Cannot query state for trigger");
        }
        assert_eq!(result, 0);

        Ok(state_output.into())
    }

    pub fn remove(self) -> anyhow::Result<()> {
        let result = unsafe { MCD_LIB.mcd_remove_trig_f(self.core.core, self.trigger_id) };
        if result != 0 {
            return Err(expect_error(Some(self.core))).with_context(|| "Cannot remove trigger");
        }

        Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CoreInfo {
    pub state: CoreState,
    pub events: CoreEvents,
    hw_thread_id: u32,
    trigger_id: u32,
    stop_reason: String,
    info: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CoreEvents {
    pub memory_change: bool,
    pub register_changed: bool,
    pub trace_changed: bool,
    pub trigger_changed: bool,
    pub stopped: bool,
    pub chl_pending: bool,
}

impl From<mcd_core_event_et> for CoreEvents {
    fn from(value: mcd_core_event_et) -> Self {
        let value = value as i32;
        CoreEvents {
            memory_change: (value & MCD_CORE_EVENT_MEMORY_CHANGE) != 0,
            register_changed: (value & MCD_CORE_EVENT_REGISTER_CHANGE) != 0,
            trace_changed: (value & MCD_CORE_EVENT_TRACE_CHANGE) != 0,
            trigger_changed: (value & MCD_CORE_EVENT_TRIGGER_CHANGE) != 0,
            stopped: (value & MCD_CORE_EVENT_STOPPED) != 0,
            chl_pending: (value & MCD_CORE_EVENT_CHL_PENDING) != 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CoreState {
    Unknown,
    Running,
    Halted,
    Debug,
    Custom,
}

impl From<mcd_core_state_et> for CoreState {
    fn from(value: mcd_core_state_et) -> Self {
        let value = value as i32;
        match value {
            _ if value == MCD_CORE_STATE_UNKNOWN => Self::Unknown,
            _ if value == MCD_CORE_STATE_RUNNING => Self::Running,
            _ if value == MCD_CORE_STATE_HALTED => Self::Halted,
            _ if value == MCD_CORE_STATE_DEBUG => Self::Debug,
            _ if (MCD_CORE_STATE_CUSTOM_LO..MCD_CORE_STATE_CUSTOM_HI).contains(&value) => {
                Self::Custom
            }
            _ => panic!("Invalid state {value}"),
        }
    }
}

impl From<mcd_core_state_st> for CoreInfo {
    fn from(value: mcd_core_state_st) -> Self {
        let stop_reason = unsafe { CStr::from_ptr(&value.stop_str[0] as *const i8) }
            .to_str()
            .unwrap()
            .to_owned();
        let info = unsafe { CStr::from_ptr(&value.info_str[0] as *const i8) }
            .to_str()
            .unwrap()
            .to_owned();
        CoreInfo {
            state: value.state.into(),
            events: value.event.into(),
            hw_thread_id: value.hw_thread_id,
            trigger_id: value.trig_id,
            stop_reason,
            info,
        }
    }
}
