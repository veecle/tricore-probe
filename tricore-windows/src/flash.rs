use std::path::PathBuf;
use std::process::{Child, Command};

use anyhow::Context;
use tempfile::TempDir;

/// Models an upload of a binary with Memtool.
pub struct MemtoolUpload {
    spawned: Child,
    _temporary_files: TempDir,
}

impl MemtoolUpload {
    /// Uploads a binary to the default device in Memtool.
    ///
    /// It generates a configuration file and uses Memtool's batch functionality to
    /// instruct the program to flash all available sections to the device.
    ///
    /// For the created operation to succeed successfully, a DAS instance must
    /// be already spawned, the device to be flashed is selected based on the given
    /// UDAS port.
    ///
    /// Note that the binary must not contain unflashable sections.
    pub fn start(ihex: String, halt_memtool: bool, udas_port: usize) -> anyhow::Result<Self> {
        let temporary_files =
            TempDir::new().context("Cannot create temporary directory for memtool input")?;

        let input_hex_path = temporary_files.path().join("input.hex");

        std::fs::write(&input_hex_path, ihex)
            .context("Cannot write create temporary input hex file")?;

        let aurix_flasher_path = PathBuf::from("C:\\AurixFlasher\\AURIXFlasher.exe");
        let mut process = Command::new(aurix_flasher_path);

        let process = process
            .arg("-hex")
            .arg(input_hex_path.display().to_string());
        let spawned = process
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .with_context(|| "Could not start AurixFlasher to flash device")?;
        log::info!("Spawned AurixFlasher to flash hex file");

        Ok(MemtoolUpload {
            spawned,
            _temporary_files: temporary_files,
        })
    }

    /// Waits on the upload process to finish.
    ///
    /// This can take a second, but if the tool fails execution it will hang here.
    /// This can happen when the flash layout is broken or when another debugger
    /// is already attached. The problem can only really be debugged with the GUI
    /// or solved by implementing reading the logs from Memtool.
    pub fn wait(&mut self) {
        let output = self
            .spawned
            .wait()
            .expect("AurixFlasher did not exit with success");
        assert!(output.success());
        log::info!("Infineon Memtool terminated successfully");
    }
}
