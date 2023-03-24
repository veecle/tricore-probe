use anyhow::{Context, Ok};
use rust_mcd::core::Core;
use tricore_common::backtrace::{csa::UpperContext, BackTrace};

use crate::backtrace::pcxi::PCXIExt;

mod csa;
mod pcxi;

pub trait BackTraceExt: Sized {
    fn read_current(core: &Core) -> anyhow::Result<Self>;
}

impl BackTraceExt for BackTrace {
    fn read_current(core: &Core) -> anyhow::Result<Self> {
        let groups = core.register_groups()?;
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

        let stack_frames = current_upper.pcxi.walk_context(core).collect();

        Ok(BackTrace {
            stack_frames,
            current_pc,
            current_upper,
        })
    }
}
