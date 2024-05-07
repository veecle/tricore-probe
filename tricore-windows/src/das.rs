//! Module to interface with a DAS instance

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Context};
use log::Level::Trace;
use log::log_enabled;

/// Spawns a DAS instance.
pub fn run_console() -> anyhow::Result<()> {
    let das_home = PathBuf::from(
        std::env::var("DAS_HOME").context("DAS_HOME not defined, is DAS installed and environment variable set?")?,
    );

    log::trace!("Starting tas_server_console.");

    let mut udas_console = Command::new(das_home.join("servers/tas_server_console.exe"));
    let udas_console =  if log_enabled!(Trace) {
        // Setting the argument to 8 enables the tas_server_console.
        udas_console.arg("8").stderr(Stdio::inherit()).stdout(Stdio::inherit())
    } else  {
        // Setting the argument to 0 disables all logging.
        udas_console.arg("0").stderr(Stdio::null()).stdout(Stdio::null())
    };

    let mut udas_console = udas_console
        .spawn()
        .context("Failed to spawn tas_server_console")?;

    log::info!("DAS server started.");
    let result = udas_console
        .wait()
        .context("tas_server_console.exe process aborted")?;

    if !result.success() {
        // If a DAS/TAS server is already running, duplicates will terminate after 5 seconds.
        log::warn!("tas_server_console exited unsuccessfully: {result:?} \
        This is normal and can be ignored if a TAS server is already running on the system.");
    }

    Ok(())
}
