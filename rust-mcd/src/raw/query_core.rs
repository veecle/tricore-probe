use anyhow::{Context, Result};

use crate::{
    error::get_error,
    mcd_bindings::{mcd_core_con_info_st, DynamicMCDxDAS, MCD_ERR_NONE},
};

impl DynamicMCDxDAS {
    /// See [Self::mcd_qry_cores_f]
    pub fn query_core_info(
        &self,
        connection_info: &mcd_core_con_info_st,
        start_index: u32,
        core_query_count: u32,
    ) -> Result<Vec<mcd_core_con_info_st>> {
        assert!(
            core_query_count > 0,
            "Can only query non-zero number of cores"
        );

        let mut core_info = vec![mcd_core_con_info_st::default(); core_query_count as usize];
        let mut num_cores = core_query_count;

        let result = unsafe {
            self.mcd_qry_cores_f(
                connection_info as *const mcd_core_con_info_st,
                start_index,
                &mut num_cores,
                core_info.as_mut_ptr(),
            )
        };

        if result != (MCD_ERR_NONE as u32) {
            return Err(get_error(None).unwrap())
                .with_context(|| "Cannot query cores of the system");
        } else {
            Ok(core_info)
        }
    }

    /// See [Self::mcd_qry_cores_f], with `num_devices` set to 0
    pub fn query_core_count(&self, connection_info: &mcd_core_con_info_st) -> Result<u32> {
        let mut num_cores = 0;

        let result = unsafe {
            self.mcd_qry_cores_f(
                connection_info as *const mcd_core_con_info_st,
                0,
                &mut num_cores,
                core::ptr::null_mut(),
            )
        };

        if result != (MCD_ERR_NONE as u32) {
            return Err(get_error(None).unwrap())
                .with_context(|| "Cannot query cores of the system");
        } else {
            Ok(num_cores)
        }
    }
}
