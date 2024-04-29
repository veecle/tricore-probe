//! FTDI driver implementation based on RPC commands.
//!
//! This is linked to the patched DLL exported by `win-ftd2xx-dll`.
use std::fs::File;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::{bail, Context};
use libftd2xx::{BitMode, FtStatus, Ftdi, FtdiCommon};
use libftd2xx_ffi::{FT_FLOW_DTR_DSR, FT_FLOW_NONE, FT_FLOW_RTS_CTS, FT_FLOW_XON_XOFF};
use rpc_api::rpc::request::RPCRequest;
use rpc_api::rpc::*;

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
            log::warn!(
                "FTDI client returned with error {:?}",
                handle_client(input, output)
            );
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

/// Helper struct to manage the FTDI devices.
#[derive(Debug, Default)]
pub struct Devices {
    devices: HashMap<u32, Ftdi>,
}

impl Devices {
    /// Get the FTDI device by its handle.
    pub fn get(&mut self, handle: u32) -> anyhow::Result<&mut Ftdi> {
        self.devices
            .get_mut(&handle)
            .context("Invalid handle index")
    }
}

fn handle_client(input: &File, output: &File) -> anyhow::Result<()> {
    log::trace!("Client handling started");
    #[cfg(target_os = "linux")]
    {
        let vendor = 0x058b;
        let product = 0x0043;
        libftd2xx::set_vid_pid(vendor, product).expect("Failed to set VID/PID");
    }

    // List of devices mapped to their handle index.
    let mut devices = Devices::default();
    let mut handle_index = 1;

    while let Ok(request) = ciborium::de::from_reader(input) {
        let response = match request {
            RPCRequest::Close(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.close();

                RPCResponse::from_result(status, response::Close {})
            }
            RPCRequest::DriverVersion(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.driver_version();

                let mut version = 0u32;
                let status = status.map(|v| {
                    version = ((v.major as u32) << 16) | ((v.minor as u32) << 8) | (v.build as u32);
                });

                RPCResponse::from_result(status, response::DriverVersion { version })
            }
            RPCRequest::LibraryVersion(_) => {
                let status = libftd2xx::library_version();

                let mut version = 0u32;
                let status = status.map(|v| {
                    version = ((v.major as u32) << 16) | ((v.minor as u32) << 8) | (v.build as u32);
                });

                RPCResponse::from_result(status, response::LibraryVersion { version })
            }
            RPCRequest::Read(body) => {
                let handle = devices.get(body.handle)?;

                let mut buffer = vec![0u8; body.max_data_len as usize];
                let status = handle.read(buffer.as_mut_slice());

                let status = status.map(|read| {
                    buffer.truncate(read);
                });

                RPCResponse::from_result(status, response::Read { data: buffer })
            }
            RPCRequest::Write(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.write(body.data.as_slice());

                let mut length = 0;
                let status = status.map(|w| {
                    length = w as u32;
                });

                RPCResponse::from_result(status, response::Write { length })
            }
            RPCRequest::SetBitMode(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.set_bit_mode(body.mask, BitMode::from(body.mode));

                RPCResponse::from_result(status, response::SetBitMode {})
            }
            RPCRequest::SetFlowControl(body) => {
                let handle = devices.get(body.handle)?;

                let status = match body.flow_control as u32 {
                    FT_FLOW_NONE => handle.set_flow_control_none(),
                    FT_FLOW_RTS_CTS => handle.set_flow_control_rts_cts(),
                    FT_FLOW_DTR_DSR => handle.set_flow_control_dtr_dsr(),
                    FT_FLOW_XON_XOFF => handle.set_flow_control_xon_xoff(body.on, body.off),
                    _ => panic!("Invalid flow control value {}", body.flow_control),
                };

                RPCResponse::from_result(status, response::SetFlowControl {})
            }
            RPCRequest::SetLatencyTimer(body) => {
                let handle = devices.get(body.handle)?;
                let timer = Duration::from_millis(body.timer_ms as u64);

                let status = handle.set_latency_timer(timer);

                RPCResponse::from_result(status, response::SetLatencyTimer {})
            }
            RPCRequest::SetTimeouts(body) => {
                let handle = devices.get(body.handle)?;
                let read_timeout = Duration::from_millis(body.read_ms as u64);
                let write_timeout = Duration::from_millis(body.write_ms as u64);

                let status = handle.set_timeouts(read_timeout, write_timeout);

                RPCResponse::from_result(status, response::SetTimeouts {})
            }
            RPCRequest::SetChars(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.set_chars(
                    body.event_character,
                    body.event_character_enable != 0,
                    body.error_character,
                    body.error_character_enabled != 0,
                );

                RPCResponse::from_result(status, response::SetChars {})
            }
            RPCRequest::SetUSBParameters(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.set_usb_parameters(body.transfer_size_in);

                RPCResponse::from_result(status, response::SetUSBParameters {})
            }
            RPCRequest::QueueLength(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.queue_status();

                let mut length = 0_u32;
                let status = status.map(|l| {
                    length = l as u32;
                });

                RPCResponse::from_result(status, response::QueueLength { length })
            }
            RPCRequest::ResetDevice(body) => {
                let handle = devices.get(body.handle)?;
                let status = handle.reset();

                RPCResponse::from_result(status, response::ResetDevice {})
            }
            RPCRequest::Open(body) => {
                let handle = Ftdi::with_index(body.number);

                let status = handle.map(|device| {
                    handle_index += 1;
                    devices.devices.insert(handle_index, device);
                });

                RPCResponse::from_result(
                    status,
                    response::Open {
                        handle_value: handle_index,
                    },
                )
            }
            RPCRequest::CreateDeviceInfoList(_) => {
                let status = libftd2xx::num_devices();

                let mut connected = 0;
                let status = status.map(|n| {
                    connected = n;
                });

                RPCResponse::from_result(
                    status,
                    response::CreateDeviceInfoList {
                        number_connected: connected,
                    },
                )
            }
            // # TODO
            // The upstream library does not have the exact method we used before. I assume
            // the device index is the index of the vector.
            //
            // Also, the inner fields are not available in the upstream library. Not sure why.
            RPCRequest::GetDetails(body) => {
                let status = libftd2xx::list_devices();

                let mut details = response::GetDetails::default();
                let status = status.and_then(|devices| {
                    let device = devices
                        .get(body.device_index as usize)
                        .ok_or(FtStatus::DEVICE_NOT_FOUND);
                    if let Ok(device) = device {
                        details.handle_value = handle_index;
                        details.device_type = device.device_type as u32;
                        details.serial_number = device.serial_number.clone();
                        details.description = device.description.clone();
                    }

                    device.map(|_| {})
                });

                RPCResponse::from_result(status, details)
            }
        };

        ciborium::ser::into_writer(&response, output)?;
    }

    bail!("Failed to read from input")
}
