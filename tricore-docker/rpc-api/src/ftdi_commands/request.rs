use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct ResetDevice {
    pub handle: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Open {
    pub number: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetDetails {
    pub device_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetUSBParameters {
    pub handle: u32,
    pub transfer_size_in: u32,
    pub transfer_size_out: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Close {
    pub handle: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueLength {
    pub handle: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetChars {
    pub event_character: u8,
    pub event_character_enable: u8,
    pub error_character: u8,
    pub error_character_enabled: u8,
    pub handle: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDeviceInfoList;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetTimeouts {
    pub read_ms: u32,
    pub write_ms: u32,
    pub handle: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetLatencyTimer {
    pub timer_ms: u8,
    pub handle: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SetFlowControl {
    pub flow_control: u16,
    pub on: u8,
    pub off: u8,
    pub handle: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetBitMode {
    pub mask: u8,
    pub mode: u8,
    pub handle: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Write {
    pub data: Vec<u8>,
    pub handle: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Read {
    pub max_data_len: u32,
    pub handle: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DriverVersion {
    pub handle: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryVersion;

#[derive(Serialize, Deserialize, Debug)]
pub enum RPCRequest {
    Open(Open),
    CreateDeviceInfoList(CreateDeviceInfoList),
    GetDetails(GetDetails),
    ResetDevice(ResetDevice),
    QueueLength(QueueLength),
    USBParamters(SetUSBParameters),
    SetChars(SetChars),
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

impl From<Close> for RPCRequest {
    fn from(value: Close) -> Self {
        RPCRequest::Close(value)
    }
}
impl From<DriverVersion> for RPCRequest {
    fn from(value: DriverVersion) -> Self {
        RPCRequest::DriverVersion(value)
    }
}
impl From<LibraryVersion> for RPCRequest {
    fn from(value: LibraryVersion) -> Self {
        RPCRequest::LibraryVersion(value)
    }
}
impl From<Read> for RPCRequest {
    fn from(value: Read) -> Self {
        RPCRequest::Read(value)
    }
}
impl From<Write> for RPCRequest {
    fn from(value: Write) -> Self {
        RPCRequest::Write(value)
    }
}
impl From<SetBitMode> for RPCRequest {
    fn from(value: SetBitMode) -> Self {
        RPCRequest::SetBitMode(value)
    }
}
impl From<SetFlowControl> for RPCRequest {
    fn from(value: SetFlowControl) -> Self {
        RPCRequest::SetFlowControl(value)
    }
}
impl From<SetLatencyTimer> for RPCRequest {
    fn from(value: SetLatencyTimer) -> Self {
        RPCRequest::SetLatencyTimer(value)
    }
}
impl From<SetTimeouts> for RPCRequest {
    fn from(value: SetTimeouts) -> Self {
        RPCRequest::SetTimeouts(value)
    }
}
impl From<SetChars> for RPCRequest {
    fn from(value: SetChars) -> Self {
        RPCRequest::SetChars(value)
    }
}
impl From<SetUSBParameters> for RPCRequest {
    fn from(value: SetUSBParameters) -> Self {
        RPCRequest::USBParamters(value)
    }
}
impl From<Open> for RPCRequest {
    fn from(value: Open) -> Self {
        RPCRequest::Open(value)
    }
}
impl From<QueueLength> for RPCRequest {
    fn from(value: QueueLength) -> Self {
        RPCRequest::QueueLength(value)
    }
}
impl From<ResetDevice> for RPCRequest {
    fn from(value: ResetDevice) -> Self {
        RPCRequest::ResetDevice(value)
    }
}
impl From<CreateDeviceInfoList> for RPCRequest {
    fn from(value: CreateDeviceInfoList) -> Self {
        RPCRequest::CreateDeviceInfoList(value)
    }
}
impl From<GetDetails> for RPCRequest {
    fn from(value: GetDetails) -> Self {
        RPCRequest::GetDetails(value)
    }
}