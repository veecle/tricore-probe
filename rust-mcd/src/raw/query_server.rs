use crate::mcd_bindings::{mcd_server_info_st, mcd_server_st, DynamicMCDxDAS};

use super::McdReturnError;

impl DynamicMCDxDAS {
    /// Behaves like [DynamicMCDxDAS::mcd_qry_servers_f], with num_servers set to 0.
    pub fn query_server_count(&self, host: &std::ffi::CStr) -> Result<u32, McdReturnError> {
        let mut num_open_servers = 0u32;
        log::debug!("Before MCF mcd_qry_servers_f");
        McdReturnError::from_library_call(unsafe {
            self.mcd_qry_servers_f(
                host.as_ptr(),
                1,
                0,
                &mut num_open_servers,
                core::ptr::null_mut(),
            )
        })?;
        log::debug!("After MCF mcd_qry_servers_f");
        Ok(num_open_servers)
    }

    /// Behaves like [DynamicMCDxDAS::mcd_qry_servers_f].
    pub fn query_server_infos(
        &self,
        host: &std::ffi::CStr,
        server_count: u32,
    ) -> Result<Vec<mcd_server_info_st>, McdReturnError> {
        let mut result_length = server_count;

        let mut result = vec![mcd_server_info_st::default(); server_count as usize];

        McdReturnError::from_library_call(unsafe {
            self.mcd_qry_servers_f(host.as_ptr(), 1, 0, &mut result_length, result.as_mut_ptr())
        })?;

        assert_eq!(result.len(), server_count as usize);

        Ok(result)
    }

    /// Behaves like [DynamicMCDxDAS::mcd_open_server_f].
    pub fn open_server(
        &self,
        config: &std::ffi::CStr,
    ) -> Result<*mut mcd_server_st, McdReturnError> {
        let system_key: i8 = 0;
        let mut server_info = core::ptr::null_mut::<mcd_server_st>();
        McdReturnError::from_library_call(unsafe {
            self.mcd_open_server_f(
                &system_key as *const i8,
                config.as_ptr(),
                &mut server_info as _,
            )
        })?;

        Ok(server_info)
    }
}
