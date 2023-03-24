use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn run_console() {
    log::trace!("Starting dashpas");

    let das_home =
        PathBuf::from(std::env::var("DAS_HOME").expect("DAS_HOME not defined, is DAS installed?"));

    let mut process = Command::new(das_home.join("dashpas/das_dashpas.exe"));

    let _started_dashpas = process.spawn().unwrap();

    log::trace!("Starting UDAS_Console");
    let mut process = Command::new(das_home.join("servers/udas/UDAS_Console.exe"));

    let process = process.stderr(Stdio::inherit()).stdout(Stdio::null());

    let mut started = process.spawn().unwrap();
    log::info!("DAS server started");
    let result = started.wait().expect("UDAS Console aborted");
    assert!(result.success());
}
