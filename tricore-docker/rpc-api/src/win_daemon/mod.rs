use std::fmt::Debug;

pub mod log;

use serde::{Deserialize, Serialize};
use tricore_common::backtrace::Stacktrace;

#[derive(Deserialize, Serialize, Debug)]
pub enum Commands {
    WriteHex(WriteHex),
    Reset,
    ListDevices,
    DefmtData { address: u64 },
    Connect(Option<DeviceInfo>),
}

#[derive(Deserialize, Serialize)]
pub struct WriteHex {
    pub elf_data: String,
}

impl Debug for WriteHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WriteHex")
            .field("elf_data_size", &self.elf_data.len())
            .finish()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Response {
    Ok,
    Error,
    Log(String),
    DefmtData(Vec<u8>),
    StackFrame(Stacktrace),
    Devices(Vec<DeviceInfo>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeviceInfo {
    pub port: usize,
    pub name: String,
}

impl tricore_common::Device for DeviceInfo {
    fn hardware_description(&self) -> &str {
        &self.name
    }
}

impl DeviceInfo {
    pub fn new(port: usize, name: String) -> Self {
        DeviceInfo { port, name }
    }
}

#[derive(Debug)]
pub enum Error {
    Internal,
}

impl Response {
    pub fn as_result(&self) -> Result<(), Error> {
        match self {
            Self::Ok => Ok(()),
            Self::Error => Err(Error::Internal),
            _ => panic!("Invalid cast"),
        }
    }
}
