//! Custom overrides of FTDI functions that communicate over RPC.
//!
//! This module **must** be refactored.
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::{os::raw::c_void, sync::Mutex};

use anyhow::Context;
use libftd2xx_ffi::*;
use rpc_api::rpc::{request, request::RPCRequest, response, RPCResponse};
use rpc_api::win_daemon::log::PipeLogger;

/// Global state for the connection to the FTDI server.
static CONNECTION: Mutex<LocalState> = Mutex::new(LocalState::new());

/// Helper to do a RPC request to the FTDI server.
fn do_request(debug_name: &str, request: impl Into<RPCRequest>) -> anyhow::Result<RPCResponse> {
    let request = RPCRequest::from(request.into());
    log::trace!("[{debug_name}] request: {request:?}");
    
    let response = CONNECTION
        .lock()
        .unwrap()
        .with_connection(|con| con.request(request))?;

    log::trace!("[{debug_name}] response: {response:?}");
    Ok(response)
}

pub struct Connection {
    from_driver: File,
    to_driver: File,
}

fn get_file(env_variable: &str, read: bool, write: bool) -> File {
    let path = std::path::PathBuf::from_str(&std::env::var(env_variable).unwrap()).unwrap();
    let mut file = File::options();
    file.read(read).write(write).open(path).unwrap()
}

impl Connection {
    fn try_connect() -> Self {
        let from_driver = get_file("FTD2XX_PIPE_FROM_DRIVER", true, false);
        let to_driver = get_file("FTD2XX_PIPE_TO_DRIVER", false, true);

        if let Ok(path) = std::env::var("FTD2XX_LOGS") {
            let logger = PipeLogger::new(PathBuf::from_str(&path).unwrap());
            log::set_boxed_logger(Box::new(logger)).unwrap();
            log::set_max_level(log::LevelFilter::Trace);
        } else {
            println!("No log pipe detected")
        }

        Connection {
            from_driver,
            to_driver,
        }
    }

    fn request(&mut self, request: RPCRequest) -> anyhow::Result<RPCResponse> {
        ciborium::ser::into_writer(&request, &self.to_driver)
            .with_context(|| "Cannot send request to ftdi server")?;

        ciborium::de::from_reader(&self.from_driver)
            .with_context(|| "Error processing response to command")
    }
}

pub struct LocalState {
    connection: Option<Connection>,
}

impl LocalState {
    pub const fn new() -> Self {
        LocalState { connection: None }
    }

    pub fn with_connection<R>(&mut self, closure: impl FnOnce(&mut Connection) -> R) -> R {
        if self.connection.is_none() {
            self.connection = Some(Connection::try_connect());
        }

        closure(self.connection.as_mut().unwrap())
    }
}

// =========================
// FTDI implementations
// =====

/// [libftd2xx_ffi::FT_Open]
pub fn open(device_number: i32, p_handle: *mut FT_HANDLE) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_Open", request::Open {
        number: device_number,
    })?;

    response.map_result(|body: response::Open| unsafe {
        *p_handle = body.handle_value as *mut c_void;
    })
}

/// [libftd2xx_ffi::FT_CreateDeviceInfoList]
pub fn create_device_info_list(num_devices: *mut u32) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_CreateDeviceInfoList", request::CreateDeviceInfoList {})?;

    response.map_result(|body: response::CreateDeviceInfoList| unsafe {
        *num_devices = body.number_connected;
    })
}

/// [libftd2xx_ffi::FT_GetDeviceInfoDetail]
pub fn get_device_info_detail(
    dw_index: DWORD,
    lpdw_flags: LPDWORD,
    lpdw_type: LPDWORD,
    lpdw_id: LPDWORD,
    lpdw_loc_id: LPDWORD,
    lp_serial_number: LPVOID,
    lp_description: LPVOID,
    pft_handle: *mut FT_HANDLE,
) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_GetDeviceInfoDetail", request::GetDetails {
        device_index: dw_index,
    })?;

    response.map_result(|body: response::GetDetails| unsafe {
        *lpdw_flags = body.flags;
        *lpdw_id = body.device_id;
        *lpdw_loc_id = body.device_location;
        *pft_handle = body.handle_value as *mut c_void;
        *lpdw_type = body.device_type;

        let serial_number = lp_serial_number as *mut u8;
        let serial_number_data = core::slice::from_raw_parts_mut(serial_number, 15);
        let serial_number_bytes = body.serial_number.as_bytes();
        serial_number_data[..serial_number_bytes.len()].copy_from_slice(serial_number_bytes);
        serial_number_data[serial_number_bytes.len()] = 0;

        let description = lp_description as *mut u8;
        let description_data = core::slice::from_raw_parts_mut(description, 63);
        let description_bytes = body.description.as_bytes();
        description_data[..description_bytes.len()].copy_from_slice(description_bytes);
        description_data[description_bytes.len()] = 0;
    })
}

/// [libftd2xx_ffi::FT_ResetDevice]
pub fn reset_device(handle: FT_HANDLE) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_ResetDevice", request::ResetDevice {
        handle: handle as u32,
    })?;

    response.map_result(|_: response::ResetDevice| {})
}

/// [libftd2xx_ffi::FT_GetQueueStatus]
pub fn get_queue_status(handle: FT_HANDLE, queue_length: LPDWORD) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_GetQueueStatus", request::QueueLength {
        handle: handle as u32,
    })?;

    response.map_result(|body: response::QueueLength| unsafe {
        *queue_length = body.length;
    })
}

/// [libftd2xx_ffi::FT_SetUSBParameters]
pub fn set_usb_parameters(
    handle: FT_HANDLE,
    transfer_in: DWORD,
    transfer_out: DWORD,
) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_SetUSBParameters", request::SetUSBParameters {
        handle: handle as u32,
        transfer_size_in: transfer_in,
        transfer_size_out: transfer_out,
    })?;

    response.map_result(|_: response::SetUSBParameters| {})
}

/// [libftd2xx_ffi::FT_SetChars]
pub fn set_chars(
    handle: FT_HANDLE,
    event_character: u8,
    event_character_enable: u8,
    error_character: u8,
    error_character_enabled: u8,
) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_SetChars", request::SetChars {
        handle: handle as u32,
        error_character,
        error_character_enabled,
        event_character,
        event_character_enable,
    })?;

    response.map_result(|_: response::SetChars| {})
}

/// [libftd2xx_ffi::FT_SetTimeouts]
pub fn set_timeouts(handle: FT_HANDLE, read_ms: u32, write_ms: u32) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_SetTimeouts", request::SetTimeouts {
        handle: handle as u32,
        read_ms,
        write_ms,
    })?;

    response.map_result(|_: response::SetTimeouts| {})
}

/// [libftd2xx_ffi::FT_SetLatencyTimer]
pub fn set_latency_timer(handle: FT_HANDLE, timer_ms: u8) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_SetLatencyTimer", request::SetLatencyTimer {
        handle: handle as u32,
        timer_ms,
    })?;

    response.map_result(|_: response::SetLatencyTimer| {})
}

/// [libftd2xx_ffi::FT_SetFlowControl]
pub fn set_flow_control(
    handle: FT_HANDLE,
    flow_control: u16,
    on: u8,
    off: u8,
) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_SetFlowControl", request::SetFlowControl {
        handle: handle as u32,
        flow_control,
        off,
        on,
    })?;

    response.map_result(|_: response::SetFlowControl| {})
}

/// [libftd2xx_ffi::FT_SetBitMode]
pub fn set_bit_mode(handle: FT_HANDLE, mask: u8, mode: u8) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_SetBitMode", request::SetBitMode {
        handle: handle as u32,
        mask,
        mode,
    })?;

    response.map_result(|_: response::SetBitMode| {})
}

/// [libftd2xx_ffi::FT_Write]
pub fn write(
    handle: FT_HANDLE,
    buffer: *mut c_void,
    bytes_write: u32,
    bytes_written: *mut u32,
) -> anyhow::Result<FT_STATUS> {
    // Create the byte slice
    let data = buffer as *mut u8;
    let slice = unsafe { core::slice::from_raw_parts(data, bytes_write as usize) };

    let response = do_request("FT_Write", request::Write {
        handle: handle as u32,
        data: slice.to_vec(),
    })?;

    response.map_result(|body: response::Write| {
        unsafe { *bytes_written = body.length };
    })
}

/// [libftd2xx_ffi::FT_Read]
pub fn read(
    handle: FT_HANDLE,
    buffer: *mut c_void,
    buffer_length: u32,
    data_read: *mut u32,
) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_Read", request::Read {
        handle: handle as u32,
        max_data_len: buffer_length,
    })?;
    

    response.map_result(|body: response::Read| {
        let data = buffer as *mut u8;
        assert!(body.data.len() as u32 <= buffer_length);

        let slice = unsafe { core::slice::from_raw_parts_mut(data, body.data.len()) };
        slice.copy_from_slice(body.data.as_slice());

        unsafe { *data_read = body.data.len() as u32 };
    })
}

/// [libftd2xx_ffi::FT_GetLibraryVersion]
pub fn library_version(version: *mut u32) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_GetLibraryVersion", request::LibraryVersion {})?;

    response.map_result(|body: response::LibraryVersion| {
        unsafe { *version = body.version };
    })
}

/// [libftd2xx_ffi::FT_GetDriverVersion]
pub fn get_driver_version(handle: FT_HANDLE, version: *mut u32) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_GetDriverVersion", request::DriverVersion {
        handle: handle as u32,
    })?;

    response.map_result(|body: response::DriverVersion| {
        unsafe { *version = body.version };
    })
}

/// [libftd2xx_ffi::FT_Close]
pub fn close(handle: FT_HANDLE) -> anyhow::Result<FT_STATUS> {
    let response = do_request("FT_Close", request::Close {
        handle: handle as u32,
    })?;

    response.map_result(|_: response::Close| {})
}
