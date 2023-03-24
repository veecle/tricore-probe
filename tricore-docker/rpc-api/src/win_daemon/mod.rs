use std::fmt::Debug;

pub mod log;

use serde::{Deserialize, Serialize};
use tricore_common::backtrace::Stacktrace;

#[derive(Deserialize, Serialize, Debug)]
pub enum Commands {
    WriteHex(WriteHex),
    Reset,
    DefmtData { address: u64 },
}

#[derive(Deserialize, Serialize)]
pub struct WriteHex {
    pub elf_data: String,
    pub halt_memtool: bool,
}

impl Debug for WriteHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WriteHex")
            .field("elf_data_size", &self.elf_data.len())
            .field("halt_memtool", &self.halt_memtool)
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
}


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
