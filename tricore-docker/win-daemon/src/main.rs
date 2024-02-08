use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use anyhow::Context;
use rpc_api::win_daemon::{log::PipeLogger, Commands, DeviceInfo, Response};

use clap::Parser;
use tricore_common::Chip;
use tricore_windows::{ChipInterface, Config};

/// Program that manages the udas server and manages its connection with the Infineon
/// memtool and the internal MCD connection
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// A path to a file where to read data from the FTDI driver
    #[arg(long, value_parser = existing_path)]
    in_from_driver: PathBuf,

    /// A path to a file where to write data to the FTDI driver
    #[arg(long, value_parser = existing_path)]
    out_to_driver: PathBuf,

    /// A path to a file where to read commands from
    #[arg(long, value_parser = existing_path)]
    in_commands: PathBuf,

    /// A path to a file where to write commands to
    #[arg(long, value_parser = existing_path)]
    out_commands: PathBuf,

    /// A path to a file where to write log messages to
    #[arg(long, value_parser = existing_path)]
    log_file: Option<PathBuf>,

    /// A path to a file where to write log messages to (for the custom ftd2xx dll)
    #[arg(long, value_parser = existing_path)]
    ftd2xx_log_file: Option<PathBuf>,

    /// Windows configuration
    #[command(flatten)]
    backend: Config,
}

fn existing_path(input_path: &str) -> Result<PathBuf, anyhow::Error> {
    let path = PathBuf::from_str(input_path).with_context(|| "Value is not a correct path")?;

    Ok(path)
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if let Some(log_file) = args.log_file {
        let logger = PipeLogger::new(log_file.as_path());
        log::set_boxed_logger(Box::new(logger)).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        env_logger::init();
    }

    let to_driver = args.out_to_driver;
    let from_driver = args.in_from_driver;

    std::env::set_var("FTD2XX_PIPE_FROM_DRIVER", &from_driver);
    std::env::set_var("FTD2XX_PIPE_TO_DRIVER", &to_driver);

    if let Some(path) = args.ftd2xx_log_file {
        std::env::set_var("FTD2XX_LOGS", &path);
    }

    let mut interface = ChipInterface::new(args.backend)?;

    let mut scanned_devices = None;

    const WAIT_TIME: Duration = Duration::from_secs(2);
    log::info!("Waiting {:?} for UDAS to start", WAIT_TIME);
    std::thread::sleep(WAIT_TIME);

    let mut command_connection =
        CommandServer::new(args.out_commands.as_path(), args.in_commands.as_path());

    while let Ok(command) = command_connection.next_command() {
        match command {
            Commands::WriteHex(hex) => {
                interface.flash_hex(hex.elf_data, hex.halt_memtool)?;
                command_connection.send_answer(Response::Ok);
            }
            Commands::Reset => {
                log::debug!("Resetting core");
                interface.reset()?;
                command_connection.send_answer(Response::Ok);
            }
            Commands::DefmtData { address } => {
                log::debug!("Initializing defmt data transmission");
                command_connection.send_answer(Response::Ok);
                let f = interface.read_rtt(address, command_connection.defmt_sink())?;
                log::trace!("Device hit debug");
                command_connection.send_answer(Response::StackFrame(f));
            }
            Commands::ListDevices => {
                log::debug!("Retrieving list of devices");
                scanned_devices = Some(interface.list_devices()?);
                command_connection.send_answer(Response::Devices(
                    scanned_devices
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|d| DeviceInfo::new(d.udas_port, d.info.acc_hw().to_owned()))
                        .collect(),
                ))
            }
            Commands::Connect(device_info) => {
                if let Some(device_info) = device_info {
                    log::debug!("Connecting to {device_info:?}");
                    let devices = scanned_devices
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Must scan first to connect"))?;

                    let device_to_connect = devices
                        .iter()
                        .find(|device| {
                            device.udas_port == device_info.port
                                && device.info.acc_hw() == device_info.name.as_str()
                        })
                        .ok_or_else(|| anyhow::anyhow!("Could not find device {device_info:?}"))?;
                    interface.connect(Some(device_to_connect))?;
                    command_connection.send_answer(Response::Ok);
                } else {
                    log::trace!("Connecting, no specific device specified");
                    interface.connect(None)?;
                    command_connection.send_answer(Response::Ok);
                }
            }
        }
    }
    log::trace!("Docker application finished, goodbye!");
    Ok(())
}

struct CommandServer<'a> {
    out: File,
    input: &'a Path,
}

impl<'a> CommandServer<'a> {
    fn new(output: &Path, input: &'a Path) -> Self {
        let mut response_channel = File::options();
        let out = response_channel
            .write(true)
            .open(output)
            .expect("Could not open response channel");
        CommandServer { out, input }
    }

    fn defmt_sink<'b>(&'b mut self) -> DefmtSink<'a, 'b> {
        DefmtSink { server: self }
    }

    fn next_command(&self) -> Result<Commands, ()> {
        let mut command_receive = File::options();
        let command_receive = command_receive.read(true);
        let command_receive = command_receive
            .open(self.input)
            .expect("Could not open receive channel");
        ciborium::de::from_reader(&command_receive).map_err(|_| ())
    }

    fn send_answer(&self, response: Response) {
        ciborium::ser::into_writer(&response, &self.out).unwrap();
        (&mut &self.out).flush().unwrap();
    }
}

struct DefmtSink<'a, 'b> {
    server: &'b mut CommandServer<'a>,
}

impl<'a, 'b> Write for DefmtSink<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.server.send_answer(Response::DefmtData(buf.to_vec()));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
