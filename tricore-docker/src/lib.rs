use std::{io::Write, sync::Arc};

use anyhow::{bail, Context};
use clap::Args;

use rpc_api::win_daemon::{Commands, DeviceInfo, Response, WriteHex};
use tricore_common::{backtrace::Stacktrace, Chip};

use self::{daemon::VirtualizedDaemon, ftdi::FTDIClient, pipe::DuplexPipeConnection};

pub type Config = DockerConfig;

mod builder;
mod daemon;
mod ftdi;
mod logger;
mod pipe;

#[derive(Args, Debug)]
pub struct DockerConfig {
    #[arg(long)]
    with_gui: Option<String>,
}

pub struct ChipInterface {
    server: Arc<DuplexPipeConnection>,
    _docker: VirtualizedDaemon,
    _ftdi: FTDIClient,
}

impl Chip for ChipInterface {
    type Config = DockerConfig;

    type Device = DeviceInfo;

    fn list_devices(&mut self) -> anyhow::Result<Vec<Self::Device>> {
        let response = self.send_request(Commands::ListDevices)?;

        let Response::Devices(devices) = response else {
            bail!("Got wrong response {response:?} from docker, expected device list")
        };

        Ok(devices)
    }

    fn connect(&mut self, device: Option<&Self::Device>) -> anyhow::Result<()> {
        self.send_request(Commands::Connect(device.map(|d| d.clone())))?
            .as_result()
            .map_err(|e| anyhow::anyhow!("Expected Ok response, got {e:?}"))
    }

    fn new(enable_gui: Config) -> anyhow::Result<Self> {
        let rpc_channel_ftdi = Arc::new(DuplexPipeConnection::new());
        let pipe_for_driver = rpc_channel_ftdi.clone();

        let rpc_channel_commands = Arc::new(DuplexPipeConnection::new());

        log::trace!("Spawning FTDI client for {:?}", pipe_for_driver);
        let ftdi = FTDIClient::spawn(pipe_for_driver);

        log::trace!("Spawning virtualized docker daemon");
        let _docker = VirtualizedDaemon::spawn(
            enable_gui.with_gui,
            rpc_channel_commands.clone(),
            rpc_channel_ftdi,
        )?;

        Ok(ChipInterface {
            server: rpc_channel_commands,
            _ftdi: ftdi,
            _docker,
        })
    }

    fn flash_hex(&mut self, ihex: String, halt_memtool: bool) -> anyhow::Result<()> {
        log::trace!("Sending flash command to daemon");
        let request = Commands::WriteHex(WriteHex {
            elf_data: ihex,
            halt_memtool,
        });

        let response = self.send_request(request)?;

        assert!(response.as_result().is_ok());
        log::trace!("Flash completed");
        Ok(())
    }

    fn read_rtt<W: Write>(
        &mut self,
        rtt_control_block: u64,
        mut decoder: W,
    ) -> anyhow::Result<Stacktrace> {
        let result = self.send_request(Commands::DefmtData {
            address: rtt_control_block,
        })?;
        result
            .as_result()
            .map_err(|_| anyhow::Error::msg("Could not start decoding defmt data"))?;

        loop {
            let response = self.next_response()?;
            match response {
                Response::DefmtData(data) => {
                    decoder.write_all(data.as_slice())?;
                }
                Response::StackFrame(frame) => return Ok(frame),
                Response::Ok => todo!(),
                Response::Error => todo!(),
                Response::Log(_) => todo!(),
                Response::Devices(_) => todo!(),
            }
        }
    }
}

impl ChipInterface {
    fn send_request(&self, request: Commands) -> anyhow::Result<Response> {
        self.send_command(request)?;
        self.next_response()
    }

    fn send_command(&self, request: Commands) -> anyhow::Result<()> {
        log::trace!("Sending request {:?}", request);
        ciborium::ser::into_writer(&request, self.server.to().open()).unwrap();
        Ok(())
    }

    fn next_response(&self) -> anyhow::Result<Response> {
        let r = ciborium::de::from_reader(self.server.from().open())
            .with_context(|| "Failed to obtain response from docker")?;

        Ok(r)
    }
}
