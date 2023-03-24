
use ftd_api::ftd2xx::FT_STATUS;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Open {
    pub handle_value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Close;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueLength {
    pub length: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetUSBParameters;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDetails {
    pub flags: u32,
    pub device_type: u32,
    pub device_id: u32,
    pub device_location: u32,
    pub serial_number: String,
    pub description: String,
    pub handle_value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDeviceInfoList {
    pub number_connected: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SetChars;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetDevice;


#[derive(Serialize, Deserialize, Debug)]
pub struct Write {
    pub length: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Read {
    pub data: Vec<u8>,
}

impl std::fmt::Debug for Read {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.len() < 10 {
            f.debug_struct("ReadResponse")
                .field("data", &self.data)
                .finish()
        } else {
            f.debug_struct("ReadResponse")
                .field("data_snippet", &(&self.data[..10]))
                .field("complete_length", &self.data.len())
                .finish()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetLatencyTimer;

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryVersion {
    pub version: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverVersion {
    pub version: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetTimeouts;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetFlowControl;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetBitMode;

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseBody {
    Open(Open),
    CreateDeviceInfoList(CreateDeviceInfoList),
    GetDetails(GetDetails),
    ResetDevice(ResetDevice),
    QueueLength(QueueLength),
    SetUSBParameters(SetUSBParameters),
    SetCharsResponse(SetChars),
    SetTimeouts(SetTimeouts),
    SetLatencyTimer(SetLatencyTimer),
    SetFlowControl(SetFlowControl),
    SetBitMode(SetBitMode),
    Write(Write),
    Read(Read),
    LibraryVersion(LibraryVersion),
    DriverVersion(DriverVersion),
    Close(Close),
}

impl TryInto<Close> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<Close, Self::Error> {
        let Self::Close(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<DriverVersion> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<DriverVersion, Self::Error> {
        let Self::DriverVersion(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<LibraryVersion> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<LibraryVersion, Self::Error> {
        let Self::LibraryVersion(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<Read> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<Read, Self::Error> {
        let Self::Read(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<Write> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<Write, Self::Error> {
        let Self::Write(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<SetBitMode> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<SetBitMode, Self::Error> {
        let Self::SetBitMode(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}

impl TryInto<SetFlowControl> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<SetFlowControl, Self::Error> {
        let Self::SetFlowControl(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<SetLatencyTimer> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<SetLatencyTimer, Self::Error> {
        let Self::SetLatencyTimer(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<SetTimeouts> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<SetTimeouts, Self::Error> {
        let Self::SetTimeouts(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<SetChars> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<SetChars, Self::Error> {
        let Self::SetCharsResponse(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<SetUSBParameters> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<SetUSBParameters, Self::Error> {
        let Self::SetUSBParameters(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<QueueLength> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<QueueLength, Self::Error> {
        let Self::QueueLength(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<ResetDevice> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<ResetDevice, Self::Error> {
        let Self::ResetDevice(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<Open> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<Open, Self::Error> {
        let Self::Open(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<CreateDeviceInfoList> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<CreateDeviceInfoList, Self::Error> {
        let Self::CreateDeviceInfoList(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}
impl TryInto<GetDetails> for ResponseBody {
    type Error = Self;
    fn try_into(self) -> Result<GetDetails, Self::Error> {
        let Self::GetDetails(value) = self else {
            return Err(self)
        };
        Ok(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RPCResponse {
    pub body: ResponseBody,
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
            Self::FT_INSUFFICIENT_RESOURCES => {
                ftd_api::ftd2xx::FT_INSUFFICIENT_RESOURCES.try_into().unwrap()
            }
            Self::FT_INVALID_PARAMETER => ftd_api::ftd2xx::FT_INVALID_PARAMETER.try_into().unwrap(),
            Self::FT_INVALID_BAUD_RATE => ftd_api::ftd2xx::FT_INVALID_BAUD_RATE.try_into().unwrap(),
            Self::FT_DEVICE_NOT_OPENED_FOR_ERASE => ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED_FOR_ERASE
                .try_into()
                .unwrap(),
            Self::FT_DEVICE_NOT_OPENED_FOR_WRITE => ftd_api::ftd2xx::FT_DEVICE_NOT_OPENED_FOR_WRITE
                .try_into()
                .unwrap(),
            Self::FT_FAILED_TO_WRITE_DEVICE => {
                ftd_api::ftd2xx::FT_FAILED_TO_WRITE_DEVICE.try_into().unwrap()
            }
            Self::FT_EEPROM_READ_FAILED => ftd_api::ftd2xx::FT_EEPROM_READ_FAILED.try_into().unwrap(),
            Self::FT_EEPROM_WRITE_FAILED => {
                ftd_api::ftd2xx::FT_EEPROM_WRITE_FAILED.try_into().unwrap()
            }
            Self::FT_EEPROM_ERASE_FAILED => {
                ftd_api::ftd2xx::FT_EEPROM_ERASE_FAILED.try_into().unwrap()
            }
            Self::FT_EEPROM_NOT_PRESENT => ftd_api::ftd2xx::FT_EEPROM_NOT_PRESENT.try_into().unwrap(),
            Self::FT_EEPROM_NOT_PROGRAMMED => {
                ftd_api::ftd2xx::FT_EEPROM_NOT_PROGRAMMED.try_into().unwrap()
            }
            Self::FT_INVALID_ARGS => ftd_api::ftd2xx::FT_INVALID_ARGS.try_into().unwrap(),
            Self::FT_NOT_SUPPORTED => ftd_api::ftd2xx::FT_NOT_SUPPORTED.try_into().unwrap(),
            Self::FT_OTHER_ERROR => ftd_api::ftd2xx::FT_OTHER_ERROR.try_into().unwrap(),
            Self::FT_DEVICE_LIST_NOT_READY => {
                ftd_api::ftd2xx::FT_DEVICE_LIST_NOT_READY.try_into().unwrap()
            }
        }
    }
}
