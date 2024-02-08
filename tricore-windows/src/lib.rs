#![feature(type_alias_impl_trait)]

use std::{io::Write, time::Duration};

use anyhow::{bail, Ok};
use defmt::{decode_rtt, HaltReason};
use flash::MemtoolUpload;
use rust_mcd::{
    connection::{Connection, ServerInfo},
    reset::ResetClass,
    system::System,
};
use tricore_common::{backtrace::Stacktrace, Chip};

mod backtrace;
pub mod das;
pub mod defmt;
pub mod flash;

#[derive(clap::Args, Debug)]
pub struct Config;

#[derive(Debug, Clone, Copy)]
pub struct DeviceSelection {
    pub udas_port: usize,
    pub info: ServerInfo,
}

impl tricore_common::Device for DeviceSelection {
    fn hardware_description(&self) -> &str {
        self.info.acc_hw()
    }
}

pub struct ChipInterface {
    device: Option<DeviceSelection>,
    connection: Option<Connection>,
}

impl Chip for ChipInterface {
    type Config = Config;

    type Device = DeviceSelection;

    fn list_devices(&mut self) -> anyhow::Result<Vec<Self::Device>> {
        let connection = self.attempt_connection()?;
        Ok(connection
            .servers()
            .enumerate()
            .map(|(udas_port, info)| DeviceSelection { udas_port, info })
            .collect())
    }

    fn connect(&mut self, device: Option<&Self::Device>) -> anyhow::Result<()> {
        self.device = device.map(|d| d.clone());

        Ok(())
    }

    fn new(_config: Self::Config) -> anyhow::Result<Self> {
        #[cfg(not(feature = "dasv8"))]
        {
            std::thread::spawn(das::run_console);
            // We need to wait a bit so that DAS is booted up correctly and sees
            // all connected chips
            std::thread::sleep(Duration::from_millis(800));
        }
        rust_mcd::library::init();
        Ok(ChipInterface {
            device: None,
            connection: None,
        })
    }

    fn flash_hex(&mut self, ihex: String, halt_memtool: bool) -> anyhow::Result<()> {
        let device = self.get_selected_device()?;
        let mut upload = MemtoolUpload::start(ihex, halt_memtool, device.udas_port)?;
        upload.wait();

        Ok(())
    }

    fn read_rtt<W: Write>(
        &mut self,
        rtt_control_block_address: u64,
        decoder: W,
    ) -> anyhow::Result<Stacktrace> {
        let system = self.get_system()?;
        let core_count = system.core_count();
        let mut core = system.get_core(0)?;
        let secondary_cores: Result<Vec<_>, _> = (1..(core_count))
            .map(|core_index| system.get_core(core_index))
            .collect();
        let mut secondary_cores = secondary_cores?;
        let HaltReason::DebugHit(halt_reason) = decode_rtt(
            &mut core,
            &mut secondary_cores,
            rtt_control_block_address,
            decoder,
        )?;
        Ok(halt_reason)
    }
}

impl ChipInterface {
    pub fn reset(&mut self) -> anyhow::Result<()> {
        rust_mcd::library::init();
        let system = self.get_system()?;

        let core = system.get_core(0)?;

        let system_reset = ResetClass::construct_reset_class(&core, 0);
        // Do we also need to reset the other cores?
        core.reset(system_reset, true)?;
        core.run()?;
        drop(system);
        Ok(())
    }

    /// Choose the best device available
    ///
    /// If no device has been selected previously but only one is available, this device will be choosen
    fn get_selected_device(&mut self) -> anyhow::Result<&DeviceSelection> {
        if self.device.is_none() {
            let connection = self.attempt_connection()?;
            match connection.count() {
                0 => bail!("No devices available"),
                1 => {
                    let device = DeviceSelection {
                        udas_port: 0,
                        info: connection.servers().next().unwrap(),
                    };
                    self.device = Some(device);
                    log::info!("Choosing default device {device:?}");
                }
                available => bail!("No device selected, multiple ({available}) available"),
            }
        }

        Ok(self.device.as_ref().unwrap())
    }

    fn attempt_connection(&mut self) -> anyhow::Result<&Connection> {
        if self.connection.is_none() {
            self.connection = Some(Connection::scan()?);
        }

        Ok(self.connection.as_ref().unwrap())
    }

    fn get_system(&mut self) -> anyhow::Result<System> {
        self.get_selected_device()?.info.connect()
    }
}
