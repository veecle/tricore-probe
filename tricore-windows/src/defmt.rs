use anyhow::{bail, Context};
use byteorder::ReadBytesExt;
use rust_mcd::{
    breakpoint::TriggerType,
    core::{Core, CoreState},
    reset::ResetClass,
};
use std::{io::Write, time::Duration};
use tricore_common::backtrace::BackTrace;

use crate::backtrace::BackTraceExt;

pub fn run_defmt<W: Write>(
    core: &mut Core<'_>,
    rtt_block_address: u64,
    mut data_target: W,
) -> anyhow::Result<HaltReason> {
    let rtt_block = RttControlBlock::new(rtt_block_address);
    let system_reset = ResetClass::construct_reset_class(core, 0);
    core.reset(system_reset, true)?;
    const TIMEOUT: Duration = Duration::from_secs(10);
    log::info!(
        "Trying to detect segger rtt block at {:#X} with timeout {:?}",
        rtt_block_address,
        TIMEOUT
    );
    let initial_trigger =
        core.create_breakpoint(TriggerType::RW, rtt_block.device_write_index_addr(), 4)?;

    core.download_triggers();
    core.run()?;

    loop {
        let state = core.query_state()?;
        if state.state != CoreState::Running {
            log::trace!("Write pointer modified, checking validity of structure");
            break;
        }
    }

    let data = core.read_bytes(rtt_block.id_addr(), 16)?;
    if &data[..16] == b"SEGGER RTT\0\0\0\0\0\0" {
        log::info!("Detected RTT control block");
    } else {
        bail!("Timeout occurred while waiting for the rtt control block to appear");
    }

    core.write(rtt_block.flags_addr(), u32::to_le_bytes(2).to_vec())?;
    initial_trigger.remove()?;
    core.run()?;

    let mut local_read_index = 0;
    let mut buffer = None;

    loop {
        // log::trace!("The core halted");
        let device_write_index = core
            .read_bytes(rtt_block.device_write_index_addr(), 4)
            .with_context(|| "Error while obtaining the device write index")?
            .as_slice()
            .read_u32::<byteorder::LE>()?;
        // core.run();
        if device_write_index != local_read_index {
            // log::trace!("new write index @ {}, old @ {}", device_write_index, local_read_index);
            let (buffer_address, buffer_size) = if let Some((address, size)) = buffer.as_ref() {
                (*address, *size)
            } else {
                let data = core
                    .read_bytes(rtt_block.buffer_addr_and_size(), 8)
                    .with_context(|| {
                        "Error while obtain buffer specification (address and size)"
                    })?;
                let mut data = data.as_slice();
                let address = data.read_u32::<byteorder::LE>()?;
                let size = data.read_u32::<byteorder::LE>()?;

                // log::debug!("Found buffer at {:#X} with size {}", address, size);

                buffer = Some((address, size));
                (address, size)
            };
            if device_write_index > buffer_size {
                bail!("The RTT write index on the device is {:#X} which exceeds the given size of {:#X}", device_write_index, buffer_size);
            }
            let new_data = if device_write_index < local_read_index {
                // log::warn!("wrapping, device is at {:?}, we are at {:?}", device_write_index, local_read_index);
                // log::trace!("Read from {:#X}, len {:?}", buffer_address + local_read_index, buffer_size - local_read_index);
                let mut chunk_at_end = core
                    .read_bytes(
                        (buffer_address + local_read_index) as u64,
                        (buffer_size - local_read_index) as usize,
                    )
                    .with_context(|| "Error while reading buffer data")?;
                if device_write_index != 0 {
                    // log::trace!("Read from {:#X}, len {:?}", buffer_address, device_write_index);
                    chunk_at_end.extend(
                        core.read_bytes(buffer_address as u64, device_write_index as usize)
                            .with_context(|| "Error while reading buffer data")?,
                    );
                }
                chunk_at_end
            } else {
                let read_address = buffer_address + local_read_index;
                let read_length = device_write_index - local_read_index;
                // log::trace!("Read from {:#X}, len {:?}", read_address, read_length);
                core.read_bytes(read_address as u64, read_length as usize)
                    .with_context(|| "Error while reading buffer data")?
            };
            local_read_index = device_write_index;
            // log::trace!("DEFMT: {:?}", &new_data);
            core.write(
                rtt_block.host_read_index_addr(),
                u32::to_le_bytes(local_read_index).into(),
            )?;
            data_target.write_all(&new_data)?;
            data_target.flush()?;
        } else {
            let core_state = core.query_state()?;
            if core_state.state != CoreState::Running {
                log::trace!("Device halted, attempting to acquire backtrace");
                let backtrace = BackTrace::read_current(core)
                    .with_context(|| "Cannot read backtrace from device")?;
                return Ok(HaltReason::DebugHit(backtrace));
            }
        }
    }
}

#[derive(Debug)]
pub enum HaltReason {
    DebugHit(BackTrace),
}

/// Helper structure to facilitate reading at the correct offsets within
/// the control block
///
/// The offsets were inferred as of https://github.com/knurling-rs/defmt/blob/59c14b924815a7185fd0079a74b936dba90c867c/firmware/defmt-rtt/src/lib.rs#L124
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
