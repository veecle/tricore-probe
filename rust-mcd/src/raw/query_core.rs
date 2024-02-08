use std::ptr::NonNull;

use crate::mcd_bindings::{mcd_core_con_info_st, mcd_core_st, DynamicMCDxDAS};

use super::McdReturnError;

impl DynamicMCDxDAS {
    /// See [Self::mcd_qry_cores_f]
    pub fn query_core_info(
        &self,
        connection_info: &mcd_core_con_info_st,
        start_index: u32,
        core_query_count: u32,
    ) -> Result<Vec<mcd_core_con_info_st>, McdReturnError> {
        assert!(
            core_query_count > 0,
            "Can only query non-zero number of cores"
        );

        let mut core_info = vec![mcd_core_con_info_st::default(); core_query_count as usize];
        let mut num_cores = core_query_count;

        mcd_call!(unsafe {
            self.mcd_qry_cores_f(
                connection_info as *const mcd_core_con_info_st,
                start_index,
                &mut num_cores,
                core_info.as_mut_ptr(),
            )
        })?;

        assert_eq!(core_info.len(), num_cores as usize);

        Ok(core_info)
    }

    /// See [Self::mcd_qry_cores_f], with `num_devices` set to 0
    pub fn query_core_count(
        &self,
        connection_info: &mcd_core_con_info_st,
    ) -> Result<u32, McdReturnError> {
        let mut num_cores = 0;

        mcd_call!(unsafe {
            self.mcd_qry_cores_f(
                connection_info as *const mcd_core_con_info_st,
                0,
                &mut num_cores,
                core::ptr::null_mut(),
            )
        })?;

        Ok(num_cores)
    }

    /// See [Self::mcd_open_core_f]
    pub fn open_core(
        &self,
        core_connection: &mcd_core_con_info_st,
    ) -> Result<NonNull<mcd_core_st>, McdReturnError> {
        let mut core_reference = core::ptr::null_mut();

        mcd_call!(unsafe { self.mcd_open_core_f(core_connection, &mut core_reference) })?;

        Ok(NonNull::new(core_reference).unwrap())
    }
}
