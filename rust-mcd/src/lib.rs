#![doc = include_str!("../README.md")]

use std::path::PathBuf;

use anyhow::Context;

pub mod breakpoint;
pub mod core;
pub mod error;
pub mod library;
pub mod memory;
pub mod registers;
pub mod reset;
pub mod system;

mod raw;
mod transaction;

mod mcd_bindings {
    #![allow(unused)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(clippy::type_complexity)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

lazy_static::lazy_static! {
    static ref MCD_LIB: crate::mcd_bindings::DynamicMCDxDAS = {
        load_library().unwrap()
    };
}

fn load_library() -> anyhow::Result<crate::mcd_bindings::DynamicMCDxDAS> {
    // The environment variable DAS_HOME is defined by the standard DAS installation
    let das_home = PathBuf::from(
        std::env::var("DAS_HOME").with_context(|| "Unable to determine path to mcdxdas.dll")?,
    );
    let mcd_das_dll_path = das_home.join("bin/mcdxdas.dll");
    unsafe { crate::mcd_bindings::DynamicMCDxDAS::new(mcd_das_dll_path) }
        .with_context(|| "Unable to load mcdxdas.dll")
}
