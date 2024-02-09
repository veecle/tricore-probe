//! Handles decoding of defmt byte streams, see [DefmtDecoder]
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Child, Command, Stdio},
};

use anyhow::Context;

use elf::{endian::AnyEndian, ElfBytes};

/// A structure that is able to decode a byte stream as defmt data
///
/// This is implemented by spawning `defmt-print` and piping the output to the
/// parents stdout. Note that this object implements [Write], so input data is
/// written into this object through this trait.
pub struct DefmtDecoder {
    spawned_decoder: Child,
    rtt_symbol_address: u64,
}

impl DefmtDecoder {
    /// Spawn a new process with the `defmt-print` utility
    ///
    /// This function will fail if the user did not install the program, e.g. via
    /// `cargo install defmt-print`.
    pub fn spawn(elf_file: &Path) -> anyhow::Result<DefmtDecoder> {
        let elf_data = fs::read(elf_file).unwrap();
        let elf = ElfBytes::<'_, AnyEndian>::minimal_parse(&elf_data).unwrap();

        let (symbols, strings) = elf
            .symbol_table()
            .with_context(|| "Could not parse symbol table from elf file")?
            .with_context(|| "Elf file does not have symbol table")?;

        let rtt_symbol_address = symbols
            .iter()
            .find_map(|symbol| {
                let Ok(symbol_name) = strings.get(symbol.st_name as usize) else {
                    return None;
                };

                if symbol_name != "_SEGGER_RTT" {
                    return None;
                }

                Some(symbol.st_value)
            })
            .ok_or_else(|| anyhow::Error::msg("Elf file does not have _SEGGER_RTT symbol"))?;

        let mut defmt_print_process = Command::new("defmt-print");
        let spawned_decoder = defmt_print_process
            .stdin(Stdio::piped())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .env("DEFMT_LOG", "trace")
            .env("RUST_LOG", "trace")
            .arg("--verbose")
            .arg("-e")
            .arg(format!("{}", elf_file.display()))
            .spawn()
            .with_context(|| "Cannot spawn 'defmt-print' to decode log frames. Did you run 'cargo install defmt-print'?")?;

        Ok(DefmtDecoder {
            spawned_decoder,
            rtt_symbol_address,
        })
    }

    /// Obtain the address of the RTT control block of the underlying binary
    ///
    /// TODO this method does not really belong to this class
    pub fn rtt_control_block_address(&self) -> u64 {
        self.rtt_symbol_address
    }
}

impl Write for DefmtDecoder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.spawned_decoder.stdin.as_mut().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.spawned_decoder.stdin.as_mut().unwrap().flush()
    }
}
