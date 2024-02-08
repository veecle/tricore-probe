use std::io::Write;

pub mod backtrace;

pub trait Device {
    fn hardware_description(&self) -> &str;
}

/// Implementors provide an interface to a chip, allowing to perform basic
/// operations on it.
///
/// Implementors use the Infineon Memtool for flashing.
pub trait Chip: Sized {
    /// Required configuration for connecting to a chip
    type Config: clap::Args;

    type Device: Device;

    /// Connect to a chip given the required configuration
    ///
    /// If no device is specified, this function will fail if not exactly one device is connected
    fn new(config: Self::Config) -> anyhow::Result<Self>;

    /// Scan for available devices
    fn list_devices(&mut self) -> anyhow::Result<Vec<Self::Device>>;

    /// Connect to a chip given the required configuration
    ///
    /// If no device is specified, this function will fail if not exactly one device is connected
    fn connect(&mut self, device: Option<&Self::Device>) -> anyhow::Result<()>;

    /// Flash the chip with the binary specified in Intel hex format
    ///
    /// Implementors should stop the memtool interface after loading the binary
    /// to allow for user interaction.
    fn flash_hex(&mut self, ihex: String, halt_memtool: bool) -> anyhow::Result<()>;

    /// Reset the chip and pass the data found in the first channel of the specified
    /// RTT control block to the given decoder
    ///
    /// The function will return when the device halts, which happens when a
    /// breakpoint is hit, e.g. `asm!("debug")`
    fn read_rtt<W: Write>(
        &mut self,
        rtt_control_block_address: u64,
        decoder: W,
    ) -> anyhow::Result<backtrace::Stacktrace>;
}
