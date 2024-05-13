use rust_mcd::connection::ServerInfo;
use std::fmt::Debug;

#[derive(clap::Args, Debug)]
pub struct Config;

#[derive(Debug, Clone, Copy)]
pub struct DeviceSelection {
    pub udas_port: usize,
    pub info: ServerInfo,
}

impl DeviceSelection {
    pub(crate) fn hardware_description(&self) -> &str {
        self.info.acc_hw()
    }
}
