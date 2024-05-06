//! Module to interface with a DAS instance

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Context};

/// Spawns a DAS instance.
pub fn run_console() -> anyhow::Result<()> {
    let das_home = PathBuf::from(
        std::env::var("DAS_HOME").context("DAS_HOME not defined, is DAS installed?")?,
    );

    log::trace!("Starting tas_server_console, or not?");

    let mut udas_console = Command::new(das_home.join("servers/tas_server_console.exe"));
    let udas_console = udas_console.stderr(Stdio::inherit()).stdout(Stdio::inherit());
    let mut udas_console = udas_console
        .spawn()
        .context("Failed to spawn tas_server_console")?;

    log::info!("DAS server started");
    let result = udas_console
        .wait()
        .context("tas_server_console.exe process aborted")?;

    if !result.success() {
        bail!("tas_server_console exited unsuccessfully: {result:?}")
    }

    Ok(())
}
