#![feature(type_alias_impl_trait)]

use std::{io::Write, time::Duration};

use anyhow::{bail, Context, Ok};
use defmt::{decode_rtt, HaltReason};
use flash::AurixFlasherUpload;
use rust_mcd::{
    connection::{Scan, ServerInfo},
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
    scan_result: Option<Scan>,
}

impl Chip for ChipInterface {
    type Config = Config;

    type Device = DeviceSelection;

    fn list_devices(&mut self) -> anyhow::Result<Vec<Self::Device>> {
        log::debug!("Before connection attempt");
        let connection = self.attempt_connection()?;
        log::debug!("After connection attempt");
        Ok(connection
            .servers()
            .enumerate()
            .map(|(udas_port, info)| DeviceSelection { udas_port, info })
            .collect())
    }

    fn connect(&mut self, device: Option<&Self::Device>) -> anyhow::Result<()> {
        self.device = device.copied();

        Ok(())
    }

    fn new(_config: Self::Config) -> anyhow::Result<Self> {

        log::debug!("Spawning DAS console.");
        std::thread::spawn(|| das::run_console().expect("Background process crashed."));
        // We need to wait a bit so that DAS is booted up correctly and sees
        // all connected chips.
        std::thread::sleep(Duration::from_millis(800));

        rust_mcd::library::init();
        Ok(ChipInterface {
            device: None,
            scan_result: None,
        })
    }

    fn flash_hex(&mut self, ihex: String) -> anyhow::Result<()> {
        let device = self
            .get_selected_device()
            .context("Failed to identify target device for AurixFlasher.")?;

        let mut upload = AurixFlasherUpload::start(ihex, device.udas_port)
            .context("Failed to run AurixFlasher.")?;

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

    /// Returns the selected device.
    ///
    /// This function will not fail if no selection has been made, but exactly one
    /// device is available.
    fn get_selected_device(&mut self) -> anyhow::Result<&DeviceSelection> {
        if self.device.is_none() {
            let connection = self.attempt_connection()?;
            let device = {
                let mut servers = connection.servers();

                let Some(first_server) = servers.next() else {
                    bail!("No devices available")
                };

                let None = servers.next() else {
                    bail!(
                        "No device selected, multiple ({}) available",
                        connection.count()
                    );
                };

                DeviceSelection {
                    udas_port: 0,
                    info: first_server,
                }
            };

            self.device = Some(device);
        }

        Ok(self.device.as_ref().unwrap())
    }

    fn attempt_connection(&mut self) -> anyhow::Result<&Scan> {
        if self.scan_result.is_none() {
            log::debug!("New scan");
            self.scan_result = Some(Scan::new()?);
            log::debug!("New scan done");
        }

        Ok(self.scan_result.as_ref().unwrap())
    }

    fn get_system(&mut self) -> anyhow::Result<System> {
        self.get_selected_device()?.info.connect()
    }
}
