#![doc = include_str!("../README.md")]
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{bail, Context};
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
use tricore_common::Device;

/// Simple program to flash and interface with tricore chips
#[derive(Parser, Debug)]
struct Args {
    /// Whether flashing should be skipped
    #[arg(long, default_value_t = false)]
    no_flash: bool,

    /// Set this flag to print a list of available devices and exit
    #[arg(long, default_value_t = false)]
    list_devices: bool,

    #[arg(short, long)]
    device: Option<String>,

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

    let mut command_server = ChipInterface::new(args.backend)?;

    if args.list_devices {
        let scanned_devices = command_server.list_devices()?;
        pretty_print_devices(&scanned_devices);
        return Ok(());
    }

    if let Some(device) = args.device {
        let scanned_devices = command_server.list_devices().with_context(|| {
            anyhow::anyhow!("Cannot scan devices to search for device \"{device}\"")
        })?;
        let matched_devices: Vec<_> = scanned_devices
            .iter()
            .filter(|element| element.hardware_description().contains(&device))
            .collect();

        match matched_devices.len() {
            0 => {
                println!("Cannot find device matching filter \"{device}\"");
                pretty_print_devices(&scanned_devices);
                bail!("Cannot connect to device")
            }
            1 => command_server.connect(Some(matched_devices[0]))?,
            _ => bail!("Multiple devices found: {matched_devices:?}"),
        }
    } else {
        command_server.connect(None)?;
    }

    if !args.no_flash {
        command_server
            .flash_elf(args.elf.as_path(), args.halt_memtool)
            .context("Cannot flash elf file")?;
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

fn pretty_print_devices<D: tricore_common::Device>(devices: &Vec<D>) {
    if devices.is_empty() {
        println!("No devices available");
        return;
    }
    println!("Found {} devices:", devices.len());
    for (index, scanned_device) in devices.iter().enumerate() {
        println!(
            "Device {index}: {:?}",
            scanned_device.hardware_description()
        )
    }
}
