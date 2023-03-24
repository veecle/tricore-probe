use ftd_api::ftd2xx::FT_STATUS;
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

                impl TryInto<$name> for ResponseBody {
                    type Error = anyhow::Error;
                    fn try_into(self) -> Result<$name, Self::Error> {
                        let Self::$name(value) = self else {
                            return Err(anyhow::Error::msg(format!("Expected type {:?}, got {:?}", stringify!($name), self)))
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CommandError {
    FT_INVALID_HANDLE,
    FT_DEVICE_NOT_FOUND,
    FT_DEVICE_NOT_OPENED,
    FT_IO_ERROR,
    FT_INSUFFICIENT_RESOURCES,
    FT_INVALID_PARAMETER,
    FT_INVALID_BAUD_RATE,
    FT_DEVICE_NOT_OPENED_FOR_ERASE,
    FT_DEVICE_NOT_OPENED_FOR_WRITE,
    FT_FAILED_TO_WRITE_DEVICE,
    FT_EEPROM_READ_FAILED,
    FT_EEPROM_WRITE_FAILED,
    FT_EEPROM_ERASE_FAILED,
    FT_EEPROM_NOT_PRESENT,
    FT_EEPROM_NOT_PROGRAMMED,
    FT_INVALID_ARGS,
    FT_NOT_SUPPORTED,
    FT_OTHER_ERROR,
    FT_DEVICE_LIST_NOT_READY,
}

impl CommandError {
    pub fn from_status(status: u32) -> Result<(), CommandError> {
        #[cfg(target_os = "windows")]
        let status: i32 = status.try_into().unwrap();

        match status {
            ftd_api::ftd2xx::FT_OK => Ok(()),
            ftd_api::ftd2xx::FT_INVALID_HANDLE => Err(Self::FT_INVALID_HANDLE),
            ftd_api::ftd2xx::FT_DEVICE_NOT_FOUND => Err(Self::FT_DEVICE_NOT_FOUND),
            ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED => Err(Self::FT_DEVICE_NOT_OPENED),
            ftd_api::ftd2xx::FT_IO_ERROR => Err(Self::FT_IO_ERROR),
            ftd_api::ftd2xx::FT_INSUFFICIENT_RESOURCES => Err(Self::FT_INSUFFICIENT_RESOURCES),
            ftd_api::ftd2xx::FT_INVALID_PARAMETER => Err(Self::FT_INVALID_PARAMETER),
            ftd_api::ftd2xx::FT_INVALID_BAUD_RATE => Err(Self::FT_INVALID_BAUD_RATE),
            ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED_FOR_ERASE => {
                Err(Self::FT_DEVICE_NOT_OPENED_FOR_ERASE)
            }
            ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED_FOR_WRITE => {
                Err(Self::FT_DEVICE_NOT_OPENED_FOR_WRITE)
            }
            ftd_api::ftd2xx::FT_FAILED_TO_WRITE_DEVICE => Err(Self::FT_FAILED_TO_WRITE_DEVICE),
            ftd_api::ftd2xx::FT_EEPROM_READ_FAILED => Err(Self::FT_EEPROM_READ_FAILED),
            ftd_api::ftd2xx::FT_EEPROM_WRITE_FAILED => Err(Self::FT_EEPROM_WRITE_FAILED),
            ftd_api::ftd2xx::FT_EEPROM_ERASE_FAILED => Err(Self::FT_EEPROM_ERASE_FAILED),
            ftd_api::ftd2xx::FT_EEPROM_NOT_PRESENT => Err(Self::FT_EEPROM_NOT_PRESENT),
            ftd_api::ftd2xx::FT_EEPROM_NOT_PROGRAMMED => Err(Self::FT_EEPROM_NOT_PROGRAMMED),
            ftd_api::ftd2xx::FT_INVALID_ARGS => Err(Self::FT_INVALID_ARGS),
            ftd_api::ftd2xx::FT_NOT_SUPPORTED => Err(Self::FT_NOT_SUPPORTED),
            ftd_api::ftd2xx::FT_OTHER_ERROR => Err(Self::FT_OTHER_ERROR),
            ftd_api::ftd2xx::FT_DEVICE_LIST_NOT_READY => Err(Self::FT_DEVICE_LIST_NOT_READY),
            s => panic!("Invalid status {s}"),
        }
    }

    pub fn as_status(&self) -> FT_STATUS {
        #[allow(clippy::useless_conversion)]
        match self {
            Self::FT_INVALID_HANDLE => ftd_api::ftd2xx::FT_INVALID_HANDLE.try_into().unwrap(),
            Self::FT_DEVICE_NOT_FOUND => ftd_api::ftd2xx::FT_DEVICE_NOT_FOUND.try_into().unwrap(),
            Self::FT_DEVICE_NOT_OPENED => ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED.try_into().unwrap(),
            Self::FT_IO_ERROR => ftd_api::ftd2xx::FT_IO_ERROR.try_into().unwrap(),
            Self::FT_INSUFFICIENT_RESOURCES => ftd_api::ftd2xx::FT_INSUFFICIENT_RESOURCES
                .try_into()
                .unwrap(),
            Self::FT_INVALID_PARAMETER => ftd_api::ftd2xx::FT_INVALID_PARAMETER.try_into().unwrap(),
            Self::FT_INVALID_BAUD_RATE => ftd_api::ftd2xx::FT_INVALID_BAUD_RATE.try_into().unwrap(),
            Self::FT_DEVICE_NOT_OPENED_FOR_ERASE => ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED_FOR_ERASE
                .try_into()
                .unwrap(),
            Self::FT_DEVICE_NOT_OPENED_FOR_WRITE => ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED_FOR_WRITE
                .try_into()
                .unwrap(),
            Self::FT_FAILED_TO_WRITE_DEVICE => ftd_api::ftd2xx::FT_FAILED_TO_WRITE_DEVICE
                .try_into()
                .unwrap(),
            Self::FT_EEPROM_READ_FAILED => {
                ftd_api::ftd2xx::FT_EEPROM_READ_FAILED.try_into().unwrap()
            }
            Self::FT_EEPROM_WRITE_FAILED => {
                ftd_api::ftd2xx::FT_EEPROM_WRITE_FAILED.try_into().unwrap()
            }
            Self::FT_EEPROM_ERASE_FAILED => {
                ftd_api::ftd2xx::FT_EEPROM_ERASE_FAILED.try_into().unwrap()
            }
            Self::FT_EEPROM_NOT_PRESENT => {
                ftd_api::ftd2xx::FT_EEPROM_NOT_PRESENT.try_into().unwrap()
            }
            Self::FT_EEPROM_NOT_PROGRAMMED => ftd_api::ftd2xx::FT_EEPROM_NOT_PROGRAMMED
                .try_into()
                .unwrap(),
            Self::FT_INVALID_ARGS => ftd_api::ftd2xx::FT_INVALID_ARGS.try_into().unwrap(),
            Self::FT_NOT_SUPPORTED => ftd_api::ftd2xx::FT_NOT_SUPPORTED.try_into().unwrap(),
            Self::FT_OTHER_ERROR => ftd_api::ftd2xx::FT_OTHER_ERROR.try_into().unwrap(),
            Self::FT_DEVICE_LIST_NOT_READY => ftd_api::ftd2xx::FT_DEVICE_LIST_NOT_READY
                .try_into()
                .unwrap(),
        }
    }
}
