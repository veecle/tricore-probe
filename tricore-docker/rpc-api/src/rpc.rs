//! RPC protocol definition for communication with FTDI server.
//!
//! This module requires heavy refactoring.
use anyhow::bail;
use libftd2xx::FtStatus;
use serde::{Deserialize, Serialize};

macro_rules! rpc_definition {
    ($($name:ident { $(pub $var_name:ident : $var_ty:ty ),*} => { $(pub $out_var:ident : $out_ty:ty),*}),*) => {
        pub mod request {
            use serde::{Serialize, Deserialize};
            #[derive(Serialize, Deserialize, Debug)]
            pub enum RPCRequest {
                $($name($name),)*
            }

            $(
                #[derive(Serialize, Deserialize, Debug)]
                pub struct $name {
                    $(pub $var_name: $var_ty,)*
                }

                impl From<$name> for RPCRequest {
                    fn from(value: $name) -> Self {
                        RPCRequest::$name(value)
                    }
                }
            )*
        }

        pub mod response {
            use serde::{Serialize, Deserialize};

            pub use super::RPCResponse;

            #[derive(Serialize, Deserialize, Debug)]
            pub enum ResponseBody {
                $($name($name),)*
            }

            $(
                #[derive(Serialize, Deserialize, Debug)]
                pub struct $name {
                    $(pub $out_var: $out_ty,)*
                }

                impl TryFrom<ResponseBody> for $name {
                    type Error = anyhow::Error;
                    fn try_from(body: ResponseBody) -> Result<$name, Self::Error> {
                        let ResponseBody::$name(value) = body else {
                            return Err(anyhow::Error::msg(format!("Expected type {:?}, got {:?}", stringify!($name), body)))
                        };
                        Ok(value)
                    }
                }
            )*
        }
    };
}

rpc_definition! {
    ResetDevice {
        pub handle: u32
    } => {},

    Open {
        pub number: i32
    } => {
        pub handle_value: u32
    },

    GetDetails {
        pub device_index: u32
    } => {
        pub flags: u32,
        pub device_type: u32,
        pub device_id: u32,
        pub device_location: u32,
        pub serial_number: String,
        pub description: String,
        pub handle_value: u32
    },

    SetUSBParameters {
        pub handle: u32,
        pub transfer_size_in: u32,
        pub transfer_size_out: u32
    } => {},

    Close {
        pub handle: u32
    } => {},

    QueueLength {
        pub handle: u32
    } => {
        pub length: u32
    },

    SetChars {
        pub event_character: u8,
        pub event_character_enable: u8,
        pub error_character: u8,
        pub error_character_enabled: u8,
        pub handle: u32
    } => {},

    CreateDeviceInfoList {} => {
        pub number_connected: u32
    },

    SetTimeouts {
        pub read_ms: u32,
        pub write_ms: u32,
        pub handle: u32
    } => {},

    SetLatencyTimer {
        pub timer_ms: u8,
        pub handle: u32
    } => {},

    SetFlowControl {
        pub flow_control: u16,
        pub on: u8,
        pub off: u8,
        pub handle: u32
    } => {},

    SetBitMode {
        pub mask: u8,
        pub mode: u8,
        pub handle: u32
    } => {},

    Write {
        pub data: Vec<u8>,
        pub handle: u32
    } => {
        pub length: u32
    },

    Read {
        pub max_data_len: u32,
        pub handle: u32
    } => {
        pub data: Vec<u8>
    },

    DriverVersion {
        pub handle: u32
    } => {
        pub version: u32
    },

    LibraryVersion {} => {
        pub version: u32
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RPCResponse {
    pub body: response::ResponseBody,
    pub status: Result<(), CommandError>,
}

impl RPCResponse {
    /// Implementation detail that transform the response to
    /// a FFI compatible result.
    pub fn map_result<T>(self, f: impl FnOnce(T)) -> anyhow::Result<u32>
    where
        T: TryFrom<response::ResponseBody, Error = anyhow::Error>,
    {
        if let Err(e) = self.status {
            bail!(e.0 as u32);
        }

        // If we are Ok(_) process the callback and return error code 0
        f(T::try_from(self.body)?);
        Ok(0)
    }
}

/// Error types returned by the FTDI server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandError(FtStatus);

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0 as u32)
    }
}

impl<'de> Deserialize<'de> for CommandError {
    fn deserialize<D>(deserializer: D) -> Result<CommandError, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(CommandError(FtStatus::from(value)))
    }
}
