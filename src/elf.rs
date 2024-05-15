//! Hosts utilities to work with elf files.

use std::process::{Command, Stdio};

use anyhow::Context;
use tempfile::TempDir;

/// Interprets the given data as a hex file and returns it in Intel hex format.
///
/// This function relies on the gnu utility 'objcopy' to be installed on the system.
pub fn elf_to_hex(data: &[u8]) -> anyhow::Result<String> {
    if cfg!(feature = "in_docker") {
        return std::fs::read_to_string("C:\\output.hex")
            .context("Docker: Cannot read resulting hex file");
    }

    let temporary_directory = TempDir::new().context("Failed to set up temporary directory")?;
    let input_path = temporary_directory.path().join("input.elf");

    std::fs::write(&input_path, data)
        .context("Cannot create temporary elf input file for objcopy")?;

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

    let hex_file =
        std::fs::read_to_string(output_file.as_path()).context("Cannot read resulting hex file")?;

    // Keep the explicit drop here, otherwise the OS might decide to drop the directory
    // before objcopy exits
    drop(temporary_directory);
    Ok(hex_file)
}
