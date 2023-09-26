#![feature(type_alias_impl_trait)]

use std::io::Write;

use defmt::{decode_rtt, HaltReason};
use flash::MemtoolUpload;
use rust_mcd::{reset::ResetClass, system::System};
use tricore_common::{backtrace::Stacktrace, Chip};

mod backtrace;
pub mod das;
pub mod defmt;
pub mod flash;

#[derive(clap::Args, Debug)]
pub struct Config;

#[derive(Default)]
pub struct ChipInterface {}

impl Chip for ChipInterface {
    type Config = Config;

    fn new(_config: Self::Config) -> anyhow::Result<Self> {
        #[cfg(not(feature = "dasv8"))]
        std::thread::spawn(das::run_console);
        rust_mcd::library::init();
        Ok(ChipInterface {})
    }

    fn flash_hex(&self, ihex: String, halt_memtool: bool) -> anyhow::Result<()> {
        let mut upload = MemtoolUpload::start(ihex, halt_memtool)?;
        upload.wait();

        Ok(())
    }

    fn read_rtt<W: Write>(
        &self,
        rtt_control_block_address: u64,
        decoder: W,
    ) -> anyhow::Result<Stacktrace> {
        let system = System::connect()?;
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
    pub fn reset(&self) -> anyhow::Result<()> {
        rust_mcd::library::init();
        let system = System::connect()?;
        let core = system.get_core(0)?;
        let system_reset = ResetClass::construct_reset_class(&core, 0);
        // Do we also need to reset the other cores?
        core.reset(system_reset, true)?;
        core.run()?;
        drop(system);
        Ok(())
    }
}
