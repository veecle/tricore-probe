/// Filter to query for servers, check [crate::mcd_bindings::DynamicMCDxDAS::mcd_open_server_f].
///
/// TODO: Various fields are missing, add when needed.
#[derive(Default, Debug)]
pub struct ServerConfig {
    /// Restricts this server to connect to devices via a specific access
    /// hardware as determined by the string.
    pub acc_hw: Option<String>,
}

impl ServerConfig {
    /// Returns the configuration as a string compatible to the MCD library.
    pub fn as_config_string(&self) -> std::ffi::CString {
        let mut composed_string = String::new();

        if let Some(value) = self.acc_hw.as_ref() {
            composed_string += &format!("McdAccHw=\"{value}\"\n");
        }

        std::ffi::CString::new(composed_string).unwrap()
    }
}
