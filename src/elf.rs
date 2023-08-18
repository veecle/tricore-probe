//! Utilities to work with elf files

use std::fs::File;
use std::process::{Command, Stdio};

use anyhow::Context;
use std::io::Write;
use tempfile::TempDir;

/// Interpret the given data as a hex file and convert it to the Intel hex format
///
/// This function relies on the gnu utility 'objcopy' to be installed on the system
pub fn elf_to_hex(data: &[u8]) -> anyhow::Result<String> {
    let temporary_directory = TempDir::new().unwrap();
    let input_path = temporary_directory.path().join("input.elf");

    let mut input_file = File::create(input_path.as_path()).unwrap();

    input_file.write_all(data).unwrap();
    input_file.flush().unwrap();
    let output_file = temporary_directory.path().join("output.hex");

    let mut command = Command::new("objcopy");
    let command = command
        .args(["-O", "ihex"])
        .arg(input_path.as_path().display().to_string())
        .arg(output_file.as_path().display().to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let result = command
        .spawn()
        .with_context(|| "Cannot spawn 'objcopy' - is the program installed?")?
        .wait_with_output()
        .with_context(|| "objcopy failed to execute")?;

    if !result.status.success() {
        let message = format!(
            "Running {:?} did not execute successfully, exit code={:?}, stderr={:?}, stdout={:?}",
            command,
            result
                .status
                .code()
                .map(|code| format!("{}", code.clone()))
                .unwrap_or("<undefined>".to_owned()),
            String::from_utf8_lossy(&result.stderr),
            String::from_utf8_lossy(&result.stdout),
        );
        return Err(anyhow::Error::msg(message));
    }

    let hex_file = std::fs::read_to_string(output_file.as_path()).unwrap();

    // Keep the explicit drop here, otherwise the OS might decide to drop the directory
    // before objcopy exits
    drop(temporary_directory);
    Ok(hex_file)
}
