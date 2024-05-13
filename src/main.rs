#![doc = include_str!("../README.md")]
#![cfg_attr(target_os = "linux", allow(dead_code))]

use crate::chip_communication::DeviceSelection;
use anyhow::{bail, Context};
use clap::Parser;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::path::PathBuf;
use std::str::FromStr;

pub mod backtrace;
mod chip_communication;
pub mod das;
pub mod defmt;
pub mod elf;
pub mod flash;

/// Simple program to flash and interface with tricore chips.
#[derive(Parser, Debug)]
struct Args {
    /// Set when flashing should be skipped.
    #[arg(long, default_value_t = false)]
    no_flash: bool,

    /// Set this flag to print a list of available devices and exit.
    #[arg(long, default_value_t = false)]
    list_devices: bool,

    /// Target device to be used.
    ///
    /// When not specified, the tool will pick an available device if exactly one
    /// device is available.
    #[arg(short, long)]
    device: Option<String>,

    /// Path to the binary.
    #[arg(value_parser = existing_path)]
    elf: PathBuf,

    /// Configures the log level.
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

    #[cfg(target_os = "linux")]
    {
        use crate::elf::elf_to_hex;
        use std::collections::HashSet;
        use std::fs;
        use std::process::Command;
        use std::process::Stdio;
        use tempfile::TempDir;

        let mut docker_command = Command::new("docker");
        let command = docker_command
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            // "-t" is required for color output.
            .args(["run", "--init", "--rm", "-t"]);

        let mut tricore_args = Vec::new();
        if args.no_flash {
            tricore_args.push("--no-flash".to_owned());
        }
        if args.list_devices {
            tricore_args.push("--list-devices".to_owned());
        }

        if let Some(device) = &args.device {
            tricore_args.push("--device".to_owned());
            tricore_args.push(device.clone());
        }

        match args.log_level {
            LogLevel::Warn => tricore_args.push("--log-level=warn".to_owned()),
            LogLevel::Info => tricore_args.push("--log-level=info".to_owned()),
            LogLevel::Debug => tricore_args.push("--log-level=debug".to_owned()),
            LogLevel::Trace => tricore_args.push("--log-level=trace".to_owned()),
        };

        let temp_dir = if let Ok(absolute_path) = args.elf.canonicalize() {
            let elf_content = fs::read(&absolute_path)
                .context("Cannot load elf file")
                .unwrap();
            let ihex_content = elf_to_hex(&elf_content)
                .context("Cannot convert elf to hex file")
                .unwrap();
            let temporary_directory =
                TempDir::new().context("Failed to set up temporary directory")?;
            let ihex_path = temporary_directory.path().join("output.hex");
            fs::write(&ihex_path, ihex_content)
                .context("Cannot create temporary hex file")
                .unwrap();

            let elf_path_mount = format!(
                "{}:{}",
                ihex_path.as_path().to_str().unwrap(),
                "/root/.wine/drive_c/output.hex"
            );
            println!("-v {}", elf_path_mount);
            command.arg("-v").arg(elf_path_mount);

            let filename = absolute_path.file_name().unwrap().to_str().unwrap();
            let file_path_in_docker_in_wine = format!("/root/.wine/drive_c/{}", filename);
            let elf_path_mount = format!(
                "{}:{}",
                absolute_path.as_path().to_str().unwrap(),
                file_path_in_docker_in_wine
            );
            println!("-v {}", elf_path_mount);
            command.arg("-v").arg(elf_path_mount);
            tricore_args.push(format!("C:\\{}", filename));
            temporary_directory
        } else {
            bail!("File not working for some reason.");
        };

        let mut daemon_command = "RUST_LOG=trace xvfb-run wine64 tricore-probe.exe".to_owned();
        for arg in tricore_args {
            daemon_command.push(' ');
            daemon_command.push_str(&arg);
        }
        log::debug!("Running command in docker: {}", daemon_command);

        let mut enumerator = udev::Enumerator::new().unwrap();
        enumerator.match_subsystem("usb").unwrap();
        enumerator
            .match_property("ID_VENDOR_FROM_DATABASE", "Infineon Technologies")
            .unwrap();
        let mut devices: HashSet<String> = HashSet::new();
        for device in enumerator.scan_devices().unwrap() {
            if let Some(dev_node) = device.devnode() {
                devices.insert(dev_node.to_str().unwrap().to_owned());
            }
        }

        if devices.is_empty() {
            bail!("No devices found.");
        }

        let mut dev_path_param = Vec::new();
        for dev_path in devices {
            println!("--device {}:{}", dev_path, dev_path);
            dev_path_param.push("--device".to_string());
            dev_path_param.push(format!("{}:{}", dev_path, dev_path));
        }
        command.args(dev_path_param);

        command
            .arg("veecle/flash-tricore")
            .args(["bash", "-c"])
            .arg(daemon_command);
        command.status().expect("Failed to run docker command");
        // keep temp dir alive
        drop(temp_dir);
    }
    #[cfg(not(target_os = "linux"))]
    {
        use crate::chip_communication::ChipCommunication;
        use colored::Colorize;
        use defmt::DefmtDecoder;

        let mut command_server = ChipCommunication::new()?;

        if args.list_devices {
            let scanned_devices = command_server.list_devices()?;
            crate::pretty_print_devices(&scanned_devices);
            return Ok(());
        }

        if let Some(device) = args.device {
            let scanned_devices = command_server.list_devices().with_context(|| {
                anyhow::anyhow!("Cannot scan devices to search for device \"{device}\"")
            })?;
            let matched_devices: Vec<_> = scanned_devices
                .iter()
                .filter(|element| element.info.acc_hw().contains(&device))
                .collect();

            match matched_devices.len() {
                0 => {
                    println!("Cannot find device matching filter \"{device}\"");
                    crate::pretty_print_devices(&scanned_devices);
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
                .flash_elf(args.elf.as_path())
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
    }
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

fn pretty_print_devices(devices: &Vec<DeviceSelection>) {
    if devices.is_empty() {
        println!("No devices available");
        return;
    }
    println!("Found {} devices:", devices.len());
    for (index, scanned_device) in devices.iter().enumerate() {
        println!("Device {index}: {:?}", scanned_device.info.acc_hw())
    }
}
