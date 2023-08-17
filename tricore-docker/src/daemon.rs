use std::sync::Arc;

use crate::builder::DockerBuilder;

use super::{
    builder::{DockerInstance, Spawned},
    logger,
    pipe::{DuplexPipeConnection, Pipe},
};

pub struct VirtualizedDaemon {
    _running_instance: DockerInstance<Spawned>,
    _rpc_channel_commands: Arc<DuplexPipeConnection>,
    _rpc_channel_ftdi: Arc<DuplexPipeConnection>,
}

impl VirtualizedDaemon {
    pub fn spawn(
        with_gui: Option<String>,
        rpc_channel_commands: Arc<DuplexPipeConnection>,
        rpc_channel_ftdi: Arc<DuplexPipeConnection>,
    ) -> anyhow::Result<Self> {
        let log_file = Pipe::new();
        let ftd2xx_log_file = Pipe::new();

        let mut builder = DockerBuilder::new();

        if let Some(with_gui) = with_gui.as_ref() {
            builder = builder.connect_display(with_gui)
        }

        let docker = builder
            .image_name("veecle/flash-tricore")
            .named("tricore-probe")
            .add_pipe_as_argument("ftd2xx-log-file", &ftd2xx_log_file)
            .add_pipe_as_argument("log-file", &log_file)
            .add_pipe_as_argument("in-from-driver", rpc_channel_ftdi.from())
            .add_pipe_as_argument("out-to-driver", rpc_channel_ftdi.to())
            .add_pipe_as_argument("in-commands", rpc_channel_commands.to())
            .add_pipe_as_argument("out-commands", rpc_channel_commands.from())
            .build();

        logger::spawn_piped(log_file, "docker");
        logger::spawn_piped(ftd2xx_log_file, "ftd2xx");

        let running_instance = docker.spawn().unwrap();

        Ok(VirtualizedDaemon {
            _running_instance: running_instance,
            _rpc_channel_commands: rpc_channel_commands,
            _rpc_channel_ftdi: rpc_channel_ftdi,
        })
    }
}
