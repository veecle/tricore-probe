use std::{
    fs::File,
    io::Write,
    process::{Child, Command},
};

use anyhow::Context;
use tempfile::TempDir;

pub struct FlashUpload {
    spawned: Child,
    _temporary_files: TempDir,
}

impl FlashUpload {
    pub fn start(ihex: String, halt_memtool: bool) -> anyhow::Result<Self> {
        let temporary_files = TempDir::new().unwrap();

        let mut input_hex = File::create(temporary_files.path().join("input.hex"))?;

        input_hex.write_all(ihex.as_bytes()).unwrap();
        input_hex.flush().unwrap();

        let mtb = if !halt_memtool {
            format!("connect\nopen_file {}\nselect_all_sections\nadd_selected_sections\nprogram\ndisconnect\nexit", temporary_files.path().join("input.hex").display())
        } else {
            format!(
                "connect\nopen_file {}\n",
                temporary_files.path().join("input.hex").display()
            )
        };

        let mut batch_file = File::create(temporary_files.path().join("batch.mtb"))?;
        batch_file.write_all(mtb.as_bytes()).unwrap();
        batch_file.flush().unwrap();

        let mut process = Command::new(env!("MEMTOOL_PATH"));
        let process = process.arg("batch.mtb").current_dir(temporary_files.path());
        let spawned = process
            .spawn()
            .with_context(|| "Could not start memtool to flash device")?;
        log::info!("Spawned Infineon Memtool to flash hex file");

        Ok(FlashUpload {
            spawned,
            _temporary_files: temporary_files,
        })
    }

    pub fn wait(&mut self) {
        // Waiting on the process can take a second, but if the tool fails execution it will hang here.
        // This can happen when the flash layout is broken or when another debugger attached.
        // The problem can only really be debugged with the GUI or by implementing reading the logs from
        // the tool
        let output = self
            .spawned
            .wait()
            .expect("Memtool did not exit with success");
        assert!(output.success());
        log::info!("Infineon Memtool terminated successfully");
    }
}
