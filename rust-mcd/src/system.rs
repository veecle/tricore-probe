//! Abstracts over a system connected to a server
//!
//! TODO This also abstracts over connecting to a server. This concept should be
//! split into a separate module/structure.

use std::ptr::{self, NonNull};

use anyhow::{bail, Context};

use crate::{
    error::get_error,
    library::scan_open_servers,
    mcd_bindings::{mcd_core_con_info_st, mcd_server_st},
    MCD_LIB,
};

use super::core::Core;

/// A single system (e.g. chip)
pub struct System {
    core_connection: mcd_core_con_info_st,
}

impl System {
    /// Connect to a system
    ///
    /// This currently only supports a single server with a single system,
    /// so this implementation will fail if multiple DAS servers are available or
    /// might unexpected results if multiple systems are connected to
    /// that server at the same time.
    ///
    /// The implementation was mainly inferred from the MCD demo project.
    pub fn connect() -> anyhow::Result<System> {
        let open_servers = scan_open_servers()?;

        if open_servers != 1 {
            bail!(
                "Library only supports exactly one server, found {}",
                open_servers
            );
        }

        let mut server_info = core::ptr::null_mut::<mcd_server_st>();

        // The length of the array were taken from the MCD demo, but theoretically
        // the arrays represent a null-terminated string, so they could be shrinked
        // to size 1
        let system_key = [0; 64];
        let config = [0; 128];

        log::trace!("Connecting to server");
        let result = unsafe {
            MCD_LIB.mcd_open_server_f(system_key.as_ptr(), config.as_ptr(), &mut server_info)
        };
        if result != 0 {
            return Err(get_error(None).unwrap()).with_context(|| "Cannot connect to server");
        }

        log::trace!("Scanning for systems attached to the server");

        // TODO at least check whether there are more than one system available for
        // the given server
        let mut num_systems = 1;
        let mut system_info = [mcd_core_con_info_st::default(); 1];

        let result =
            unsafe { MCD_LIB.mcd_qry_systems_f(0, &mut num_systems, system_info.as_mut_ptr()) };
        if result != 0 {
            return Err(get_error(None).unwrap()).with_context(|| "Initial query to system failed");
        }

        log::trace!("Scanning for cores in the given system");

        let mut core_info = [mcd_core_con_info_st::default(); 1];
        let mut num_core = 1;
        let result = unsafe {
            MCD_LIB.mcd_qry_cores_f(
                system_info.as_ptr(),
                0,
                &mut num_core,
                core_info.as_mut_ptr(),
            )
        };

        if result != 0 {
            return Err(get_error(None).unwrap())
                .with_context(|| "Cannot connect to the first core of the system");
        }

        Ok(System {
            core_connection: core_info[0],
        })
    }

    /// Open a connection to the first core of this system
    pub fn get_first_core(&self) -> Result<Core, anyhow::Error> {
        let mut reference = ptr::null_mut();

        // I observed that in certain circumstances opening a core can fail (mcd_open_core_f
        // returns 2), so we retry as suggested by the MCD library documentation
        const TRIES: usize = 5;

        for nth_try in 0..TRIES {
            let result = unsafe { MCD_LIB.mcd_open_core_f(&self.core_connection, &mut reference) };
            match result {
                0 => {
                    return Ok(Core::new(
                        unsafe { NonNull::new(reference).unwrap().as_ref() },
                        &self.core_connection,
                    ))
                }
                2 => log::trace!("Retrying to open core, try number {nth_try}"),
                _ => {
                    return Err(get_error(None).unwrap())
                        .with_context(|| "MCD library reported an error")
                }
            }
        }
        bail!("Could not open core after {TRIES} tries");
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe { MCD_LIB.mcd_exit_f() }
    }
}
