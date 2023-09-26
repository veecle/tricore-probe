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
use env_logger::{Builder, Target};
use log::LevelFilter;

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

    /// Sets the log level
    #[arg(short, long, value_enum, required = false, default_value_t = LogLevel::Warn)]
    log_level: LogLevel,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let log_filter = match args.log_level {
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    };

    Builder::from_default_env()
        .filter_level(log_filter)
        .target(Target::Stdout)
        .init();

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

#[derive(clap::ValueEnum, Clone, Debug)]
enum LogLevel {
    Warn,
    Info,
    Debug,
    Trace,
}

fn existing_path(input_path: &str) -> anyhow::Result<PathBuf> {
    PathBuf::from_str(input_path).with_context(|| "Value is not a correct path")
}
