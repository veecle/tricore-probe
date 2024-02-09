use std::{
    collections::HashMap,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;
use colored::{Color, Colorize};
use elf::{endian::AnyEndian, ElfBytes};
use tricore_common::backtrace::{csa::SavedContext, Stacktrace};

pub struct BackTraceInfo {
    stack_frames: Vec<StackFrameInfo>,
}

impl BackTraceInfo {
    pub fn log_stdout(&self) {
        for f in self.stack_frames.iter() {
            f.log_stdout();
        }
    }
}

pub trait ParseInfo {
    fn addr2line(&self, elf_file: &Path) -> anyhow::Result<BackTraceInfo>;
}

impl ParseInfo for Stacktrace {
    fn addr2line(&self, elf_file: &Path) -> anyhow::Result<BackTraceInfo> {
        let mut registry = Addr2LineRegistry::new(elf_file);
        let trap_metadata = TrapMetadata::from_elf(elf_file).unwrap_or(TrapMetadata::empty());

        registry.load(
            self.stack_frames
                .iter()
                .map(|ctx| ctx.return_address())
                .chain([self.current_pc].into_iter())
                .chain([self.current_upper.a11].into_iter()),
        )?;

        let mut stack_frames = Vec::new();

        let current_trapinfo = trap_metadata
            .trap_class(self.current_pc)
            .map(|class| TrapInfo {
                class,
                trap_id: self.current_upper.d15 as u8,
            });

        stack_frames.push(StackFrameInfo {
            address: self.current_pc,
            is_trap: current_trapinfo,
            info: registry.get_address_info(self.current_pc)?,
        });

        stack_frames.push(StackFrameInfo {
            address: self.current_upper.a11,
            is_trap: None,
            info: registry.get_address_info(self.current_upper.a11)?,
        });

        for ctx in self.stack_frames.iter() {
            let is_trap = if let SavedContext::Upper(ctx) = ctx {
                trap_metadata.trap_class(ctx.a11).map(|class| TrapInfo {
                    class,
                    trap_id: ctx.d15.try_into().unwrap(),
                })
            } else {
                None
            };

            stack_frames.push(StackFrameInfo {
                address: ctx.return_address(),
                is_trap,
                info: registry.get_address_info(ctx.return_address())?,
            })
        }

        Ok(BackTraceInfo { stack_frames })
    }
}

pub struct StackFrameInfo {
    address: u32,
    is_trap: Option<TrapInfo>,
    info: Addr2LineInfo,
}

#[derive(Debug)]
#[allow(dead_code)]
struct TrapInfo {
    class: u8,
    trap_id: u8,
}

impl StackFrameInfo {
    fn log_stdout(&self) {
        let address = self.address;
        let function = &self.info.function;
        let module = &self.info.module;
        let trap_info = self
            .is_trap
            .as_ref()
            .map(|info| format!("-> detected as trap handler {info:?}"))
            .unwrap_or_else(|| "".into());

        println!(
            "{} -> {} {}\n{}",
            format!("{address:#8X}").white(),
            function.bold().blue(),
            trap_info.bold().on_white().red(),
            format!("└────────── @ {module}").color(Color::TrueColor {
                r: 100,
                g: 100,
                b: 100
            })
        );
    }
}

#[derive(Clone)]
struct Addr2LineInfo {
    function: String,
    module: String,
}

struct Addr2LineRegistry<'a> {
    elf_file: &'a Path,
    registry: HashMap<u32, Addr2LineInfo>,
}

impl<'a> Addr2LineRegistry<'a> {
    fn new(elf_file: &'a Path) -> Self {
        Addr2LineRegistry {
            elf_file,
            registry: HashMap::new(),
        }
    }

    fn get_address_info(&mut self, address: u32) -> anyhow::Result<Addr2LineInfo> {
        let Some(info) = self.registry.get(&address) else {
            self.load([address].into_iter())?;
            return Ok(self.registry.get(&address).unwrap().clone());
        };

        Ok(info.clone())
    }

    fn load<I: Iterator<Item = u32>>(&mut self, addresses: I) -> anyhow::Result<()> {
        let mut defmt_print_process = Command::new("addr2line");
        let spawned_decoder = defmt_print_process
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .arg("-e")
            .arg(format!("{}", self.elf_file.display()))
            .arg("-f")
            .arg("-C");

        let addresses: Vec<u32> = addresses.collect();

        for a in addresses.iter() {
            spawned_decoder.arg(format!("{a:#X}"));
        }

        let addr2line_stdout = spawned_decoder
            .spawn()
            .with_context(|| "Cannot spawn addr2line to decode stack frame")?
            .wait_with_output()
            .with_context(|| "addr2line did not terminate properly")?
            .stdout;
        let string =
            String::from_utf8(addr2line_stdout).with_context(|| "Invalid addr2line output")?;
        let mut items: Vec<&str> = string.split('\n').collect();
        items.truncate(items.len() - 1);

        for (debug, address) in items.chunks_exact(2).zip(addresses.iter()) {
            self.registry.insert(
                *address,
                Addr2LineInfo {
                    function: debug[0].to_owned(),
                    module: debug[1].to_owned(),
                },
            );
        }

        Ok(())
    }
}

/// Captures information about the currently installed trap tables
struct TrapMetadata {
    /// This field will contain the base address of the trap table if one was found
    trap_symbol: Option<u32>,
}

impl TrapMetadata {
    /// Extract information about available trap tables from the given elf file
    ///
    /// If this function call fails, [TrapMetadata::empty] may be used to create
    /// a stub variant of this structure.
    fn from_elf(elf_file: &Path) -> anyhow::Result<Self> {
        let elf_data = std::fs::read(elf_file).unwrap();
        let elf = ElfBytes::<'_, AnyEndian>::minimal_parse(&elf_data).unwrap();

        let (symbols, strings) = elf
            .symbol_table()
            .with_context(|| "Could not parse symbol table from elf file")?
            .with_context(|| "Elf file does not have symbol table")?;

        const VALID_SYMBOLS: &[&str] = &["first_trap_table", "BSP_TRAP_VECTOR_TABLE"];

        let trap_symbol = symbols
            .iter()
            .find_map(|symbol| {
                let Ok(symbol_name) = strings.get(symbol.st_name as usize) else {
                    return None;
                };

                if !VALID_SYMBOLS.contains(&symbol_name) {
                    return None;
                }

                Some(symbol.st_value)
            })
            .with_context(|| {
                format!(
                    "Could not find trap table; searched for symbols {:?}",
                    VALID_SYMBOLS
                )
            })?;

        Ok(TrapMetadata {
            trap_symbol: Some(trap_symbol.try_into().unwrap()),
        })
    }

    /// Creates an instance with no metadata associated to it
    ///
    /// When [`trap_class`][Self::trap_class] is called on such an instance,
    /// it will always return [None].
    pub fn empty() -> Self {
        TrapMetadata { trap_symbol: None }
    }

    /// Based on the metadata in this structure, attempt to identify the trap
    /// class based on the given address
    ///
    /// This address is usually the program counter of the program when it hit the trap.
    fn trap_class(&self, address: u32) -> Option<u8> {
        let offset_from_start = address.checked_sub(self.trap_symbol?)?;

        if offset_from_start > 8 * 32 {
            return None;
        }

        let class = offset_from_start / 32;

        Some(class as u8)
    }
}
