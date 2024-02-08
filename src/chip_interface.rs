use std::{fmt::Debug, fs, io::Write, path::Path};

use crate::elf::elf_to_hex;

pub use imp::Config;
use tricore_common::{backtrace::Stacktrace, Chip};

cfg_if::cfg_if! {
    if #[cfg(all(feature = "docker", feature = "windows"))] {
        compile_error!("Features 'docker' and 'windows' are mutually exclusive");
    } else if #[cfg(feature = "docker")] {
        use tricore_docker as imp;
    } else if #[cfg(feature = "windows")] {
        use tricore_windows as imp;
    } else {
        compile_error!("One of the features 'docker' or 'windows' must be enabled");
    }
}

pub type ChipInterface = ChipInterfaceImpl<imp::ChipInterface>;

pub struct ChipInterfaceImpl<C: Chip> {
    implementation: C,
}

impl<C: Chip> ChipInterfaceImpl<C>
where
    C::Device: Debug,
{
    /// Initiate a new connection to a chip
    pub fn new(interface_configuration: C::Config) -> anyhow::Result<Self> {
        Ok(ChipInterfaceImpl {
            implementation: C::new(interface_configuration)?,
        })
    }

    /// Like [Chip::list_devices]
    pub fn list_devices(&mut self) -> anyhow::Result<Vec<C::Device>> {
        self.implementation.list_devices()
    }

    /// Like [Chip::connect]
    pub fn connect(&mut self, device: Option<&C::Device>) -> anyhow::Result<()> {
        if let Some(device) = device {
            log::debug!("Connecting to device {device:?}");
        } else {
            log::debug!("Connecting to any available device");
        }

        self.implementation.connect(device)
    }

    /// Like [Chip::flash_hex], but the binary is specified as a path to an elf
    /// file instead of provided as Intel hex in memory.
    pub fn flash_elf(&mut self, elf_file: &Path, halt_memtool: bool) -> anyhow::Result<()> {
        log::info!("Converting elf {} to hex file", elf_file.display());
        let elf_data = fs::read(elf_file).unwrap();
        let ihex = elf_to_hex(&elf_data)?;
        log::info!("Flashing hex file");
        self.implementation.flash_hex(ihex, halt_memtool)
    }

    /// Like [Chip::read_rtt]
    pub fn read_rtt<W: Write>(
        &mut self,
        rtt_control_block_address: u64,
        decoder: W,
    ) -> anyhow::Result<Stacktrace> {
        self.implementation
            .read_rtt(rtt_control_block_address, decoder)
    }
}
