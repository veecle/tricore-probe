//! Handles decoding of defmt byte streams, see [DefmtDecoder].
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Child, Command, Stdio},
};
use std::time::Duration;

use anyhow::{bail, Context};
use byteorder::ReadBytesExt;

use elf::{endian::AnyEndian, ElfBytes};
use rust_mcd::breakpoint::TriggerType;
use rust_mcd::core::{Core, CoreState};
use rust_mcd::error::EventError;
use rust_mcd::reset::ResetClass;
use crate::backtrace::{Stacktrace, StacktraceExt};

/// This structure handles decoding a byte stream as defmt data.
///
/// This is implemented by spawning `defmt-print` and piping the output to the
/// parents stdout. Note that this object implements [Write], so input data is
/// written into this object through this trait.
pub struct DefmtDecoder {
    spawned_decoder: Child,
    rtt_symbol_address: u64,
}

impl DefmtDecoder {
    /// Starts a new decoding process.
    ///
    /// Output will be written to the current standard output.
    ///
    /// This function will fail if the user did not install the program, e.g. via
    /// `cargo install defmt-print`.
    pub fn spawn(elf_file: &Path) -> anyhow::Result<DefmtDecoder> {
        let elf_data = fs::read(elf_file).context("Cannot read elf file")?;
        let elf =
            ElfBytes::<'_, AnyEndian>::minimal_parse(&elf_data).context("Cannot parse elf file")?;

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

    /// Returns the address of the RTT control block used by the underlying binary.
    ///
    /// TODO this method does not really belong to this class.
    pub fn rtt_control_block_address(&self) -> u64 {
        self.rtt_symbol_address
    }
}

impl Write for DefmtDecoder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.spawned_decoder
            .stdin
            .as_mut()
            .expect("Process must be set up with stdin")
            .write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.spawned_decoder
            .stdin
            .as_mut()
            .expect("Process must be set up with stdin")
            .flush()
    }
}


/// Decode the rtt data from the first channel of the specified rtt block and
/// write it to the supplied data sink.
///
/// A main core must be provided through which the RTT data is read from the chip.
///
/// The function will return when the device halts, e.g. when any core (including the
/// secondary ones) hits a breakpoint. The backtrace returned is obtained by
/// traversing the CSA link list.
pub fn decode_rtt<W: Write>(
    core: &mut Core<'_>,
    secondary_cores: &mut [Core<'_>],
    rtt_block_address: u64,
    mut data_sink: W,
) -> anyhow::Result<HaltReason> {
    let rtt_block = RttControlBlock::new(rtt_block_address);
    // Construct reset class 0 which is assumed to be the simplest reset
    // Can we assume that this reset class exists and it fits the usecase here?
    let system_reset = ResetClass::construct_reset_class(core, 0);

    // Do we also need to reset the other cores?
    core.reset(system_reset, true)?;

    for secondary_core in secondary_cores.iter_mut() {
        secondary_core.reset(system_reset, true)?;
        secondary_core.download_triggers();
    }

    log::info!(
        "Trying to detect segger rtt block at {:#X}",
        rtt_block_address
    );

    // We create a breakpoint that puts the chip into debug mode when the write index
    // is changed and then wait for the chip to hit the breakpoint.
    let breakpoint_on_write_change =
        core.create_breakpoint(TriggerType::RW, rtt_block.device_write_index_addr(), 4)?;
    core.download_triggers();
    core.run()?;

    loop {
        let state = core.query_state()?;
        if state.state != CoreState::Running {
            log::trace!("Breakpoint hit, checking validity of structure");
            break;
        }
    }

    // Best effort to make sure that the address is correct: We check the first
    // bytes of the rtt control block, they must contain the given data
    let data = core.read_bytes(rtt_block.id_addr(), 16)?;
    if &data[..16] == b"SEGGER RTT\0\0\0\0\0\0" {
        log::info!("Detected RTT control block");
    } else {
        bail!("The device halted, but the rtt control is malformatted");
    }

    core.write(rtt_block.flags_addr(), u32::to_le_bytes(2).to_vec())?; // Set the flag that the host is connected

    // Read the buffer parameters at startup, they should not change at runtime
    let ring_buffer = {
        let data = core
            .read_bytes(rtt_block.buffer_addr_and_size(), 8)
            .with_context(|| "Cannot obtain buffer specification (address and size)")?;

        let mut data = data.as_slice();
        let address = data.read_u32::<byteorder::LE>()?;
        let size = data.read_u32::<byteorder::LE>()?;

        log::trace!("Found buffer at {:#X} with size {}", address, size);

        BufferSetup { address, size }
    };

    // Remove the breakpoint, we do busy looping to acquire the rtt data
    breakpoint_on_write_change.remove()?;

    core.run()?;

    let mut local_read_index = 0;

    loop {
        read_from_core(
            core,
            &mut data_sink,
            &rtt_block,
            &mut local_read_index,
            &ring_buffer,
        )?;

        /// Checks if the core is still running, if it is not we assume a
        /// breakpoint was hit.
        ///
        /// This function is a bit of a hack to work around lifetime issues
        /// when borrowing the cores in multiple iterations of the loop.
        fn should_exit_for_core(
            core: &mut Core,
            accept_reset_event: bool,
        ) -> Option<anyhow::Result<HaltReason>> {
            let core_state = core.query_state_gracefully(|e| {
                accept_reset_event && e.event_error_code() == EventError::Reset
            });

            let core_state = match core_state {
                Ok(core_state) => core_state,
                Err(error) => return Some(Err(error).context("Failed to query core state")),
            };

            if core_state.state != CoreState::Running {
                log::trace!("Device halted, attempting to acquire backtrace");
                return Some(
                    core.read_current()
                        .with_context(|| "Cannot read backtrace from device")
                        .map(HaltReason::DebugHit),
                );
            }

            None
        }

        const RTT_WAIT_DURATION: Duration = Duration::from_millis(300);

        if let Some(exit_reason) = should_exit_for_core(core, false) {
            if exit_reason.is_ok() {
                log::info!(
                    "Main core halted, collecting RTT data for {}ms",
                    RTT_WAIT_DURATION.as_millis()
                );
                std::thread::sleep(RTT_WAIT_DURATION);
                read_from_core(
                    core,
                    &mut data_sink,
                    &rtt_block,
                    &mut local_read_index,
                    &ring_buffer,
                )?;
            }
            return exit_reason.context("Cannot query state of the main core");
        }

        for (secondary_index, core) in secondary_cores.iter_mut().enumerate() {
            if let Some(exit_reason) = should_exit_for_core(core, true) {
                if exit_reason.is_ok() {
                    // FIXME: The core index we give here might be misleading, we can probably obtain
                    // that information from the core itself
                    log::info!(
                        "Secondary core {} halted, collecting RTT data for {}ms",
                        secondary_index + 1,
                        RTT_WAIT_DURATION.as_millis()
                    );
                    std::thread::sleep(RTT_WAIT_DURATION);
                    read_from_core(
                        core,
                        &mut data_sink,
                        &rtt_block,
                        &mut local_read_index,
                        &ring_buffer,
                    )?;
                }
                return exit_reason.with_context(|| {
                    format!("Cannot query state of core {}", secondary_index + 1)
                });
            }
        }
    }
}

/// Represents the reason why decoding rtt data failed.
#[derive(Debug)]
pub enum HaltReason {
    DebugHit(Stacktrace),
}

/// Models addresses within an rtt control block.
///
/// The model is based on https://github.com/knurling-rs/defmt/blob/59c14b924815a7185fd0079a74b936dba90c867c/firmware/defmt-rtt/src/lib.rs#L124.
struct RttControlBlock {
    base_address: u64,
}

impl RttControlBlock {
    fn new(address: u64) -> Self {
        RttControlBlock {
            base_address: address,
        }
    }

    fn id_addr(&self) -> u64 {
        self.base_address
    }

    fn device_write_index_addr(&self) -> u64 {
        self.base_address + 36
    }

    fn buffer_addr_and_size(&self) -> u64 {
        self.base_address + 28
    }

    fn host_read_index_addr(&self) -> u64 {
        self.base_address + 40
    }

    fn flags_addr(&self) -> u64 {
        self.base_address + 44
    }
}

/// Models the buffer in use at runtime.
struct BufferSetup {
    /// Defines where the ring buffer starts.
    address: u32,
    /// Defines the size of the ring buffer.
    size: u32,
}

fn read_from_core<W: Write>(
    core: &mut Core,
    data_sink: &mut W,
    rtt_block: &RttControlBlock,
    local_read_index: &mut u32,
    ring_buffer: &BufferSetup,
) -> anyhow::Result<()> {
    let device_write_index = core
        .read_bytes(rtt_block.device_write_index_addr(), 4)
        .with_context(|| "Error while obtaining the device write index")?
        .as_slice()
        .read_u32::<byteorder::LE>()?;
    if device_write_index == *local_read_index {
        return Ok(());
    }
    if device_write_index > ring_buffer.size {
        bail!(
            "The RTT write index on the device is {:#X} which exceeds the given size of {:#X}",
            device_write_index,
            ring_buffer.size
        );
    }
    let new_data = if device_write_index < *local_read_index {
        // The write wrapped, we might need to do two reads
        let mut chunk_at_end = core
            .read_bytes(
                (ring_buffer.address + *local_read_index) as u64,
                (ring_buffer.size - *local_read_index) as usize,
            )
            .with_context(|| "Error while reading buffer data")?;
        if device_write_index != 0 {
            chunk_at_end.extend(
                core.read_bytes(ring_buffer.address as u64, device_write_index as usize)
                    .with_context(|| "Error while reading buffer data")?,
            );
        }
        chunk_at_end
    } else {
        let read_address = ring_buffer.address + *local_read_index;
        let read_length = device_write_index - *local_read_index;
        core.read_bytes(read_address as u64, read_length as usize)
            .with_context(|| "Error while reading buffer data")?
    };
    log::trace!("Read {} bytes from the device", new_data.len());
    *local_read_index = device_write_index;

    core.write(
        rtt_block.host_read_index_addr(),
        u32::to_le_bytes(*local_read_index).into(),
    )?;
    data_sink.write_all(&new_data)?;
    data_sink.flush()?;

    Ok(())
}
