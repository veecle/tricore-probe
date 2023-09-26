//! Module to interface with a DAS instance

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

/// Spawns a DAS instance
pub fn run_console() {
    let das_home =
        PathBuf::from(std::env::var("DAS_HOME").expect("DAS_HOME not defined, is DAS installed?"));

    #[cfg(not(feature = "dasv8"))]
    {
        log::trace!("Starting dashpas");
        let mut process = Command::new(das_home.join("dashpas/das_dashpas.exe"));
        let _started_dashpas = process.spawn().unwrap();
    }

    log::trace!("Starting UDAS_Console");

    let mut udas_console = Command::new(das_home.join("servers/udas/UDAS_Console.exe"));
    let udas_console = udas_console.stderr(Stdio::inherit()).stdout(Stdio::null());
    let mut udas_console = udas_console.spawn().unwrap();

    log::info!("DAS server started");
    let result = udas_console.wait().expect("UDAS Console aborted");
    assert!(result.success());
}
