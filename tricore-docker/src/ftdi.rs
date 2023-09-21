use std::ffi::{c_void, CStr};
use std::fs::File;
use std::sync::Arc;
use std::thread::JoinHandle;

use ftd_api::ftd2xx::FT_HANDLE;
use rpc_api::ftdi_commands::request::{Close, DriverVersion, GetDetails, RPCRequest};
use rpc_api::ftdi_commands::{response, CommandError};
use std::collections::HashMap;

use super::pipe::DuplexPipeConnection;

pub struct FTDIClient {
    _ftdi_thread: JoinHandle<()>,
}

impl FTDIClient {
    pub fn spawn(pipe_for_driver: Arc<DuplexPipeConnection>) -> Self {
        let ftdi_thread = std::thread::spawn(move || {
            let input = pipe_for_driver.to().open();
            let output = pipe_for_driver.from().open();
            log::debug!("Starting FTDI client");
            handle_client(input, output);
        });

        FTDIClient {
            _ftdi_thread: ftdi_thread,
        }
    }

    #[allow(dead_code)]
    pub fn wait(self) {
        self._ftdi_thread.join().unwrap();
    }
}

fn handle_client(input: &File, output: &File) {
    log::trace!("Client handling started");
    #[cfg(target_os = "linux")]
    {
        let vendor = 0x058b;
        let product = 0x0043;
        let result = CommandError::from_status(native::FT_SetVIDPID(vendor, product));
        assert_eq!(result, Ok(()));
    }
    let mut state = LocalState::new();
    let mut handle_counter = 1;
    while let Ok(request) = ciborium::de::from_reader(input) {
        let request: RPCRequest = request;
        let response = match request {
            RPCRequest::Close(Close { handle }) => {
                let current_handle = state.devices.get(&handle).unwrap();
                let result = unsafe { native::FT_Close(*current_handle) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::Close(response::Close {}),
                    status: result,
                }
            }
            RPCRequest::DriverVersion(DriverVersion { handle }) => {
                let current_handle = state.devices.get(&handle).unwrap();
                let mut version: u32 = 0;
                let result = unsafe { native::FT_GetDriverVersion(*current_handle, &mut version) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::DriverVersion(response::DriverVersion {
                        version,
                    }),
                    status: result,
                }
            }
            RPCRequest::LibraryVersion(_) => {
                let mut version: u32 = 0;
                let result = unsafe { native::FT_GetLibraryVersion(&mut version) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::LibraryVersion(response::LibraryVersion {
                        version,
                    }),
                    status: result,
                }
            }
            RPCRequest::Read(read) => {
                let current_handle = state.devices.get(&read.handle).unwrap();
                let mut data = Vec::<u8>::with_capacity(read.max_data_len as usize);
                let mut counter = read.max_data_len;
                while counter > 0 {
                    data.push(0);
                    counter -= 1;
                }
                let mut bytes_read: u32 = 0;
                let data_address = data.as_mut_slice().as_mut_ptr();
                let data_len = read.max_data_len;
                assert!(read.max_data_len as usize == data.len());
                let result = unsafe {
                    native::FT_Read(
                        *current_handle,
                        data_address as *mut c_void,
                        data_len,
                        &mut bytes_read,
                    )
                };
                let result = CommandError::from_status(result);
                data.truncate(bytes_read as usize);

                response::RPCResponse {
                    body: response::ResponseBody::Read(response::Read { data }),
                    status: result,
                }
            }
            RPCRequest::Write(mut input) => {
                let current_handle = state.devices.get(&input.handle).unwrap();
                let address = input.data.as_slice().as_ptr();
                assert_eq!(address, &mut input.data.as_mut_slice()[0] as *const u8);
                let mut bytes_written = 0;
                let result = unsafe {
                    native::FT_Write(
                        *current_handle,
                        address as *mut c_void,
                        input.data.len() as u32,
                        &mut bytes_written,
                    )
                };
                let result = CommandError::from_status(result);
                let response = response::RPCResponse {
                    body: response::ResponseBody::Write(response::Write {
                        length: bytes_written,
                    }),
                    status: result,
                };
                drop(input);
                response
            }
            RPCRequest::SetBitMode(t) => {
                let current_handle = state.devices.get(&t.handle).unwrap();
                let result = unsafe { native::FT_SetBitMode(*current_handle, t.mask, t.mode) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::SetBitMode(response::SetBitMode {}),
                    status: result,
                }
            }
            RPCRequest::SetFlowControl(t) => {
                let current_handle = state.devices.get(&t.handle).unwrap();
                let result = unsafe {
                    native::FT_SetFlowControl(*current_handle, t.flow_control, t.on, t.off)
                };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::SetFlowControl(response::SetFlowControl {}),
                    status: result,
                }
            }
            RPCRequest::SetLatencyTimer(t) => {
                let current_handle = state.devices.get(&t.handle).unwrap();
                let result = unsafe { native::FT_SetLatencyTimer(*current_handle, t.timer_ms) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::SetLatencyTimer(response::SetLatencyTimer {}),
                    status: result,
                }
            }
            RPCRequest::SetTimeouts(st) => {
                let current_handle = state.devices.get(&st.handle).unwrap();
                let result =
                    unsafe { native::FT_SetTimeouts(*current_handle, st.read_ms, st.write_ms) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::SetTimeouts(response::SetTimeouts {}),
                    status: result,
                }
            }
            RPCRequest::SetChars(s) => {
                let current_handle = state.devices.get(&s.handle).unwrap();
                let result = unsafe {
                    native::FT_SetChars(
                        *current_handle,
                        s.event_character,
                        s.event_character_enable,
                        s.error_character,
                        s.error_character_enabled,
                    )
                };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::SetChars(response::SetChars {}),
                    status: result,
                }
            }
            RPCRequest::SetUSBParameters(p) => {
                let current_handle = state.devices.get(&p.handle).unwrap();
                let result = unsafe {
                    native::FT_SetUSBParameters(
                        *current_handle,
                        p.transfer_size_in,
                        p.transfer_size_out,
                    )
                };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::SetUSBParameters(response::SetUSBParameters {}),
                    status: result,
                }
            }
            RPCRequest::QueueLength(q) => {
                let current_handle = state.devices.get(&q.handle).unwrap();
                let mut queue_length = 0;
                let result =
                    unsafe { native::FT_GetQueueStatus(*current_handle, &mut queue_length) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::QueueLength(response::QueueLength {
                        length: queue_length,
                    }),
                    status: result,
                }
            }
            RPCRequest::ResetDevice(d) => {
                let current_handle = state.devices.get(&d.handle).unwrap();
                // println!("Resetting device {:?}", current_handle);
                let result = unsafe { native::FT_ResetDevice(*current_handle) };
                let result = CommandError::from_status(result);

                response::RPCResponse {
                    body: response::ResponseBody::ResetDevice(response::ResetDevice {}),
                    status: result,
                }
            }
            RPCRequest::Open(r) => {
                let mut handle: FT_HANDLE = core::ptr::null_mut();
                let result = unsafe { native::FT_Open(r.number, &mut handle) };
                let result = CommandError::from_status(result);
                handle_counter += 1;
                if result.is_ok() {
                    // println!("Registering internal handle {:?} at index {}", handle, handle_counter);
                    state.devices.insert(handle_counter, handle);
                }

                response::RPCResponse {
                    body: response::ResponseBody::Open(response::Open {
                        handle_value: handle_counter,
                    }),
                    status: result,
                }
            }
            RPCRequest::CreateDeviceInfoList(_) => {
                let mut device: u32 = 0;
                let result = unsafe { native::FT_CreateDeviceInfoList(&mut device) };
                let result = CommandError::from_status(result);
                // println!("Requested device info list, device number are {:?}", device);

                response::RPCResponse {
                    body: response::ResponseBody::CreateDeviceInfoList(
                        response::CreateDeviceInfoList {
                            number_connected: device,
                        },
                    ),
                    status: result,
                }
            }
            RPCRequest::GetDetails(GetDetails { device_index }) => {
                let mut handle: FT_HANDLE = core::ptr::null_mut();
                let mut flags = 0;
                let mut device_type = 0;
                let mut device_id = 0;
                let mut device_location = 0;
                let mut serial_data = [0; 128];
                let mut description = [0; 128];
                let result = unsafe {
                    native::FT_GetDeviceInfoDetail(
                        device_index,
                        &mut flags,
                        &mut device_type,
                        &mut device_id,
                        &mut device_location,
                        serial_data.as_mut_ptr() as *mut c_void,
                        description.as_mut_ptr() as *mut c_void,
                        &mut handle,
                    )
                };
                let result = CommandError::from_status(result);
                handle_counter += 1;
                if result.is_ok() {
                    // println!("Registering internal handle {:?} at index {}", handle, handle_counter);
                    state.devices.insert(handle_counter, handle);
                }

                let detail_reponse = response::GetDetails {
                    flags,
                    device_type,
                    device_id,
                    device_location,
                    serial_number: str_from_null_terminated_utf8_safe(&serial_data).to_owned(),
                    description: str_from_null_terminated_utf8_safe(&description).to_owned(),
                    handle_value: handle_counter,
                };

                response::RPCResponse {
                    body: response::ResponseBody::GetDetails(detail_reponse),
                    status: result,
                }
            }
        };
        ciborium::ser::into_writer(&response, output).unwrap();
    }
    log::warn!("Failed to read from input");
}

pub struct LocalState {
    devices: HashMap<u32, FT_HANDLE>,
}

impl LocalState {
    fn new() -> Self {
        LocalState {
            devices: HashMap::new(),
        }
    }
}

fn str_from_null_terminated_utf8_safe(s: &[u8]) -> &str {
    if s.iter().any(|&x| x == 0) {
        unsafe { str_from_null_terminated_utf8(s) }
    } else {
        std::str::from_utf8(s).unwrap()
    }
}

// unsafe: s must contain a null byte
unsafe fn str_from_null_terminated_utf8(s: &[u8]) -> &str {
    CStr::from_ptr(s.as_ptr() as *const _).to_str().unwrap()
}
