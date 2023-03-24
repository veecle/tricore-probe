#![feature(type_alias_impl_trait)]

use std::io::Write;

use defmt::{run_defmt, HaltReason};
use flash::FlashUpload;
use rust_mcd::{reset::ResetClass, system::System};
use server::run_console;
use tricore_common::{backtrace::BackTrace, Chip};

mod backtrace;
pub mod defmt;
pub mod flash;
pub mod server;

#[derive(clap::Args, Debug)]
pub struct Config;

#[derive(Default)]
pub struct ChipInterface {}

impl Chip for ChipInterface {
    type Config = Config;

    fn new(_config: Self::Config) -> anyhow::Result<Self> {
        std::thread::spawn(run_console);
        Ok(ChipInterface {})
    }

    fn flash_hex(&self, ihex: String, halt_memtool: bool) -> anyhow::Result<()> {
        let mut upload = FlashUpload::start(ihex, halt_memtool)?;
        upload.wait();

        Ok(())
    }

    fn read_rtt<W: Write>(
        &self,
        rtt_control_block_address: u64,
        decoder: W,
    ) -> anyhow::Result<BackTrace> {
        rust_mcd::library::init();
        let system = System::connect()?;
        let mut core = system.get_first_core()?;
        let HaltReason::DebugHit(halt_reason) =
            run_defmt(&mut core, rtt_control_block_address, decoder)?;
        Ok(halt_reason)
    }
}

impl ChipInterface {
    pub fn reset(&self) -> anyhow::Result<()> {
        rust_mcd::library::init();
        let system = System::connect()?;
        let core = system.get_first_core()?;
        let system_reset = ResetClass::construct_reset_class(&core, 0);
        core.reset(system_reset, true)?;
        core.run()?;
        drop(system);
        Ok(())
    }
}
