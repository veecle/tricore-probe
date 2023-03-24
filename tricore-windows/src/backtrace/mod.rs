//! This module defines structures to create a backtrace using the [rust_mcd] library

use anyhow::{Context, Ok};
use rust_mcd::core::Core;
use tricore_common::backtrace::{csa::UpperContext, Stacktrace};

use crate::backtrace::pcxi::PCXIExt;

mod csa;
mod pcxi;

/// Extension trait to obtain a stacktrace
pub trait StacktraceExt: Sized {
    /// Read the stacktrace for the given core
    /// 
    /// This function is available on the [Core] type.
    fn read_current(&self) -> anyhow::Result<Stacktrace>;
}

impl<'a> StacktraceExt for &'a Core<'a> {
    fn read_current(&self) -> anyhow::Result<Stacktrace> {
        let groups = self.register_groups()?;
        let group = groups.get_group(0)?;

        let register = |name: &str| {
            Ok(group
                .register(name)
                .ok_or_else(|| {
                    anyhow::Error::msg(format!("Could not find {name} register for core"))
                })?
                .read()
                .with_context(|| format!("Cannot read {name} register"))?)
        };

        let current_upper = UpperContext {
            pcxi: register("PCXI")?.into(),
            psw: register("PSW")?,
            a10: register("A10")?,
            a11: register("A11")?,
            d8: register("D8")?,
            d9: register("D9")?,
            d10: register("D10")?,
            d11: register("D11")?,
            a12: register("A12")?,
            a13: register("A13")?,
            a14: register("A14")?,
            a15: register("A15")?,
            d12: register("D12")?,
            d13: register("D13")?,
            d14: register("D14")?,
            d15: register("D15")?,
        };

        let current_pc = register("PC")?;

        let stack_frames = current_upper.pcxi.walk_context(self).collect();

        Ok(Stacktrace {
            stack_frames,
            current_pc,
            current_upper,
        })
    }
}
