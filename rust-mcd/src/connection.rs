use std::{ffi::CStr, fmt::Debug};

use crate::{error::McdError, mcd_bindings::mcd_server_info_st, system::System, MCD_LIB};

pub struct Connection {
    servers: Vec<mcd_server_info_st>,
}

impl Connection {
    /// Scan for available servers
    pub fn scan() -> anyhow::Result<Self> {
        let host = CStr::from_bytes_with_nul(b"localhost\0").unwrap();
        let server_count = MCD_LIB.query_server_count(host)?;
        let servers = MCD_LIB
            .query_server_infos(host, server_count)
            .add_mcd_error_info(None)?;

        let connection = Connection { servers };

        log::trace!("Scanned for servers, found {connection:?}");

        Ok(connection)
    }

    /// List all servers available in this connection
    pub fn servers(&self) -> impl Iterator<Item = ServerInfo> + '_ {
        self.servers.iter().map(ServerInfo::from)
    }

    /// Number of servers available
    pub fn count(&self) -> usize {
        self.servers.len()
    }
}

/// Information a server
#[derive(Clone, Copy)]
pub struct ServerInfo {
    pub(crate) inner: mcd_server_info_st,
}

impl ServerInfo {
    /// Connect to this server exposing the system in it
    pub fn connect(&self) -> anyhow::Result<System> {
        System::connect(self)
    }
}

impl<'a> From<&'a mcd_server_info_st> for ServerInfo {
    fn from(value: &'a mcd_server_info_st) -> Self {
        ServerInfo { inner: *value }
    }
}

impl ServerInfo {
    /// Descriptor of the hardware in use by the server
    pub fn acc_hw(&self) -> &str {
        // SAFETY
        // i8 and u8 have the same memory layout
        let acc_hw = unsafe { std::mem::transmute::<&[i8], &[u8]>(&self.inner.acc_hw) };
        CStr::from_bytes_with_nul(acc_hw).unwrap().to_str().unwrap()
    }

    /// Description of the server itself
    pub fn server(&self) -> &str {
        // SAFETY
        // i8 and u8 have the same memory layout
        let server = unsafe { std::mem::transmute::<&[i8], &[u8]>(&self.inner.server) };
        CStr::from_bytes_with_nul(server).unwrap().to_str().unwrap()
    }

    /// TODO what is the semantics of this?
    pub fn system_instance(&self) -> &str {
        // SAFETY
        // i8 and u8 have the same memory layout
        let system_instance =
            unsafe { std::mem::transmute::<&[i8], &[u8]>(&self.inner.system_instance) };
        CStr::from_bytes_with_nul(system_instance)
            .unwrap()
            .to_str()
            .unwrap()
    }
}

impl Debug for ServerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerInfo")
            .field("acc_hw", &self.acc_hw())
            .field("server", &self.server())
            .field("system_instance", &self.system_instance())
            .finish()
    }
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: Vec<_> = self
            .servers
            .iter()
            .map(ServerInfo::from)
            .collect();
        f.debug_struct("Connection")
            .field("servers", &result)
            .finish()
    }
}
