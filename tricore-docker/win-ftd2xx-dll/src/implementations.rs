use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::{os::raw::c_void, sync::Mutex};

use anyhow::Context;
use rpc_api::ftdi_commands::request::{
    CreateDeviceInfoList, GetDetails, Open, QueueLength, RPCRequest, ResetDevice, SetBitMode,
    SetChars, SetFlowControl, SetLatencyTimer, SetTimeouts, SetUSBParameters,
};
use rpc_api::ftdi_commands::response::{self};
use rpc_api::ftdi_commands::RPCResponse;

use ftd_api::ftd2xx::{FT_HANDLE, FT_OK, FT_STATUS};

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

static CONNECTION: Mutex<LocalState> = Mutex::new(LocalState::new());

macro_rules! send_request {
    ($request:tt, |$return_response:ident : $return_type:ty| $handle:tt ) => {
        let request = $request;
        let result = CONNECTION
            .lock()
            .unwrap()
            .with_connection(|con| con.request(request.into()))?;

        if let Err(command_error) = result.status {
            Ok(command_error.as_status())
        } else {
            let body: $return_type = result
                .body
                .try_into()
                .with_context(|| "RPC end send wrong response")?;
            let function = |$return_response: $return_type| $handle;
            let _: () = function(body);
            FT_OK
                .try_into()
                .with_context(|| "RPC end reported failed function")
        }
    };
}

pub fn open(device_number: i32, p_handle: *mut FT_HANDLE) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { Open { number: device_number}},
        |body: response::Open| {
            unsafe { *p_handle = body.handle_value as *mut c_void;}
        }
    }
}

pub fn create_device_info_list(num_devices: *mut u32) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { CreateDeviceInfoList {} },
        |body: response::CreateDeviceInfoList| {
            unsafe { *num_devices = body.number_connected; }
        }
    }
}

use ftd_api::ftd2xx::{DWORD, LPDWORD, LPVOID};
use rpc_api::win_daemon::log::PipeLogger;

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
    send_request! {
        { GetDetails{device_index: dw_index}},
        |body: response::GetDetails| {
            unsafe {
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
            }
        }
    }
}

pub fn reset_device(handle: FT_HANDLE) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { ResetDevice {handle: handle as u32}},
        |_body: response::ResetDevice| {
        }
    }
}

pub fn get_queue_status(handle: FT_HANDLE, queue_length: LPDWORD) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { QueueLength {handle: handle as u32}},
        |body: response::QueueLength| {
            unsafe {
                *queue_length = body.length;
            }
        }
    }
}
pub fn set_usb_parameters(
    handle: FT_HANDLE,
    transfer_in: DWORD,
    transfer_out: DWORD,
) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { SetUSBParameters {handle: handle as u32, transfer_size_in: transfer_in, transfer_size_out: transfer_out}},
        |_body: response::SetUSBParameters| {
        }
    }
}

pub fn set_chars(
    handle: FT_HANDLE,
    event_character: u8,
    event_character_enable: u8,
    error_character: u8,
    error_character_enabled: u8,
) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { SetChars { handle: handle as u32, error_character, event_character_enable, event_character, error_character_enabled}},
        |_body: response::SetChars| {
        }
    }
}

pub fn set_timeouts(handle: FT_HANDLE, read_ms: u32, write_ms: u32) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { SetTimeouts { handle: handle as u32, read_ms, write_ms}},
        |_body: response::SetTimeouts| {
        }
    }
}

pub fn set_latency_timer(handle: FT_HANDLE, timer_ms: u8) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { SetLatencyTimer { handle: handle as u32, timer_ms}},
        |_body: response::SetLatencyTimer| {
        }
    }
}

pub fn set_flow_control(
    handle: FT_HANDLE,
    flow_control: u16,
    on: u8,
    off: u8,
) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { SetFlowControl { handle: handle as u32, flow_control, off, on }},
        |_body: response::SetFlowControl| {
        }
    }
}
pub fn set_bit_mode(handle: FT_HANDLE, mask: u8, mode: u8) -> anyhow::Result<FT_STATUS> {
    send_request! {
        { SetBitMode { handle: handle as u32, mask, mode }},
        |_body: response::SetBitMode| {
        }
    }
}
pub fn write(
    handle: FT_HANDLE,
    buffer: *mut c_void,
    bytes_write: u32,
    bytes_written: *mut u32,
) -> anyhow::Result<FT_STATUS> {
    send_request! {
        {
            let data = buffer as *mut u8;
            let slice = unsafe { core::slice::from_raw_parts(data, bytes_write as usize)};
            rpc_api::ftdi_commands::request::Write {
                handle: handle as u32,
                data: slice.to_vec()
            }},
        |body: response::Write| {
            unsafe { *bytes_written = body.length };
        }
    }
}
pub fn read(
    handle: FT_HANDLE,
    buffer: *mut c_void,
    buffer_length: u32,
    data_read: *mut u32,
) -> anyhow::Result<FT_STATUS> {
    send_request! {
        {
            rpc_api::ftdi_commands::request::Read {
                handle: handle as u32,
                max_data_len: buffer_length
            }},
        |body: response::Read| {
            let data = buffer as *mut u8;
            assert!(body.data.len() as u32 <= buffer_length);
            let slice = unsafe { core::slice::from_raw_parts_mut(data, body.data.len())};
            slice.copy_from_slice(body.data.as_slice());
            unsafe { *data_read = body.data.len() as u32};
        }
    }
}

pub fn library_version(version: *mut u32) -> anyhow::Result<FT_STATUS> {
    send_request! {
        {
            rpc_api::ftdi_commands::request::LibraryVersion {}
        },
        |body: response::LibraryVersion| {
            unsafe { *version = body.version};
        }
    }
}
pub fn get_driver_version(handle: FT_HANDLE, version: *mut u32) -> anyhow::Result<FT_STATUS> {
    send_request! {
        {
            rpc_api::ftdi_commands::request::DriverVersion { handle: handle as u32}
        },
        |body: response::DriverVersion| {
            unsafe { *version = body.version};
        }
    }
}
pub fn close(handle: FT_HANDLE) -> anyhow::Result<FT_STATUS> {
    send_request! {
        {
            rpc_api::ftdi_commands::request::Close { handle: handle as u32}
        },
        |_body: response::Close| {
        }
    }
}
