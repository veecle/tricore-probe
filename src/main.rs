#![doc = include_str!("../README.md")]
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use colored::Colorize;

pub mod backtrace;
pub mod chip_interface;
pub mod defmt;
pub mod elf;
use backtrace::ParseInfo;
use chip_interface::ChipInterface;
use defmt::DefmtDecoder;

/// Simple program to flash and interface with tricore chips
#[derive(Parser, Debug)]
struct Args {
    /// Whether flashing should be skipped
    #[arg(long, default_value_t = false)]
    no_flash: bool,

    /// Path to the binary
    #[arg(value_parser = existing_path)]
    elf: PathBuf,

    /// Configuration for the backend
    #[command(flatten)]
    backend: chip_interface::Config,

    /// Stop after setting up the memtool. Memtool will stay open and tricore-probe
    /// will halt until memtool is closed by the user
    #[arg(long, default_value_t = false)]
    halt_memtool: bool,

    /// Increase verbosity for logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::init();

    log::set_max_level(filter_from_verbosity(args.verbose));

    let command_server = ChipInterface::new(args.backend)?;

    if !args.no_flash {
        command_server.flash_elf(args.elf.as_path(), args.halt_memtool)?;
    } else {
        log::warn!("Flashing skipped - this might lead to malformed defmt data!")
    }

    let mut defmt_decoder = DefmtDecoder::spawn(args.elf.as_path())?;

    let backtrace = command_server.read_rtt(
        defmt_decoder.rtt_control_block_address(),
        &mut defmt_decoder,
    )?;

    let backtrace_info = backtrace.addr2line(args.elf.as_path())?;

    println!("{}", "Device halted, backtrace as follows".red());
    backtrace_info.log_stdout();

    Ok(()) as Result<(), anyhow::Error>
}

fn filter_from_verbosity(verbose_flag_count: u8) -> log::LevelFilter {
    match verbose_flag_count {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    }
}

fn existing_path(input_path: &str) -> anyhow::Result<PathBuf> {
    PathBuf::from_str(input_path).with_context(|| "Value is not a correct path")
}
