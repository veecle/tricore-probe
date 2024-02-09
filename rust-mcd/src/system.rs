//! Abstracts over a system connected to a server
//!
//! TODO This also abstracts over connecting to a server. This concept should be
//! split into a separate module/structure.

use anyhow::{bail, Context};

use crate::{
    config::ServerConfig, connection::ServerInfo, error::McdError,
    mcd_bindings::mcd_core_con_info_st, raw::McdReturnError, MCD_LIB,
};

use super::core::Core;

/// A single system (e.g. chip)
pub struct System {
    core_connection: Vec<mcd_core_con_info_st>,
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
    pub(crate) fn connect(server_information: &ServerInfo) -> anyhow::Result<System> {
        log::trace!("Connecting to {server_information:?}");

        let configuration = ServerConfig {
            acc_hw: Some(server_information.acc_hw().to_owned()),
        };

        let configuration = configuration.as_config_string();

        // Apparently we just need to open the server to make the connection available in the MCD internal database
        let _ = MCD_LIB
            .open_server(configuration.as_c_str())
            .add_mcd_error_info(None)?;

        log::trace!("Scanning for systems attached to the server");

        let num_systems = MCD_LIB.query_system_count()?;
        let system_info = MCD_LIB
            .query_systems(num_systems)
            .add_mcd_error_info(None)?;

        if system_info.len() != 1 {
            // Not really sure what to do here since the semantic of a "system" is not really clear
            bail!("FIXME: Multiple systems found, cannot select the correct one");
        }

        let system_info = system_info[0];

        log::trace!("Scanning for cores in the given system");

        let core_count = MCD_LIB
            .query_core_count(&system_info)
            .add_mcd_error_info(None)
            .context("Cannot obtain core count")?;

        let core_info = MCD_LIB
            .query_core_info(&system_info, 0, core_count)
            .add_mcd_error_info(None)
            .with_context(|| format!("Cannot obtain information for {core_count} core(s)"))?;

        Ok(System {
            core_connection: core_info,
        })
    }

    /// Open a connection to a core of this system
    ///
    /// # Panic
    /// This method will panic if the index exceeds the number of cores available
    pub fn get_core(&self, core_index: usize) -> Result<Core, anyhow::Error> {
        // I observed that in certain circumstances opening a core can fail (mcd_open_core_f
        // returns 2), so we retry as suggested by the MCD library documentation
        const TRIES: usize = 5;

        for _ in 0..TRIES {
            match MCD_LIB.open_core(&self.core_connection[core_index]) {
                Ok(result) => return Ok(Core::new(result, &self.core_connection[core_index])),
                Err(McdReturnError::TryAgain) => continue,
                Err(other) => return Err(other).add_mcd_error_info(None),
            };
        }
        bail!("Could not open core after {TRIES} tries");
    }

    /// The number of cores connected to this system
    pub fn core_count(&self) -> usize {
        self.core_connection.len()
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe { MCD_LIB.mcd_exit_f() }
    }
}
