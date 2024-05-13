use crate::backtrace::Stacktrace;
use anyhow::{bail, Context};
use rust_mcd::connection::Scan;
use rust_mcd::reset::ResetClass;
use rust_mcd::system::System;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use crate::chip_interface::DeviceSelection;
use crate::das;
use crate::defmt::{decode_rtt, HaltReason};
use crate::elf::elf_to_hex;
use crate::flash::AurixFlasherUpload;

pub struct ChipCommunication {
    device: Option<DeviceSelection>,
    scan_result: Option<Scan>,
}

impl ChipCommunication {
    pub(crate) fn list_devices(&mut self) -> anyhow::Result<Vec<DeviceSelection>> {
        let connection = self.attempt_connection()?;
        anyhow::Ok(
            connection
                .servers()
                .enumerate()
                .map(|(udas_port, info)| DeviceSelection { udas_port, info })
                .collect(),
        )
    }

    pub(crate) fn connect(&mut self, device: Option<&DeviceSelection>) -> anyhow::Result<()> {
        if let Some(device) = device {
            log::debug!("Connecting to device {device:?}");
        } else {
            log::debug!("Connecting to any available device");
        }

        self.device = device.copied();

        anyhow::Ok(())
    }

    pub(crate) fn new() -> anyhow::Result<Self> {
        log::debug!("Spawning DAS console.");
        std::thread::spawn(|| das::run_console().expect("Background process crashed."));
        // We need to wait a bit so that DAS is booted up correctly and sees
        // all connected chips.
        std::thread::sleep(Duration::from_millis(800));

        rust_mcd::library::init();
        anyhow::Ok(Self {
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

        anyhow::Ok(())
    }

    /// Behaves like [Chip::flash_hex], but the binary is specified as a path to an elf
    /// file instead of provided as Intel hex in memory.
    pub fn flash_elf(&mut self, elf_file: &Path) -> anyhow::Result<()> {
        log::info!("Converting elf {} to hex file", elf_file.display());
        let elf_data = fs::read(elf_file).context("Cannot load elf file")?;
        let ihex = elf_to_hex(&elf_data).context("Cannot convert elf to hex file")?;
        log::info!("Flashing hex file");
        self.flash_hex(ihex)
    }

    pub(crate) fn read_rtt<W: Write>(
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
        anyhow::Ok(halt_reason)
    }

    pub fn reset(&mut self) -> anyhow::Result<()> {
        rust_mcd::library::init();
        let system = self.get_system()?;

        let core = system.get_core(0)?;

        let system_reset = ResetClass::construct_reset_class(&core, 0);
        // Do we also need to reset the other cores?
        core.reset(system_reset, true)?;
        core.run()?;
        drop(system);
        anyhow::Ok(())
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

        anyhow::Ok(self.device.as_ref().unwrap())
    }

    fn attempt_connection(&mut self) -> anyhow::Result<&Scan> {
        if self.scan_result.is_none() {
            self.scan_result = Some(Scan::new()?);
        }

        anyhow::Ok(self.scan_result.as_ref().unwrap())
    }

    fn get_system(&mut self) -> anyhow::Result<System> {
        self.get_selected_device()?.info.connect()
    }
}
