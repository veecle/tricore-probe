#![allow(non_snake_case)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]

pub use ftd_api::ftd2xx::{
    FT_Close, FT_CreateDeviceInfoList, FT_GetDeviceInfoDetail, FT_GetDriverVersion,
    FT_GetLibraryVersion, FT_GetQueueStatus, FT_ListDevices, FT_Open, FT_Read, FT_ResetDevice,
    FT_SetBitMode, FT_SetChars, FT_SetFlowControl, FT_SetLatencyTimer, FT_SetTimeouts,
    FT_SetUSBParameters, FT_Write,
};

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        use ftd_api::ftd2xx::FT_STATUS;

        pub fn FT_SetVIDPID(vendor: u32, product: u32) -> FT_STATUS {
            #[link(name = "ftd2xx", kind = "static")]
            extern "C" {
                #[link_name = "FT_SetVIDPID"]
                fn _FT_SetVIDPID(vendor: u32, product: u32) -> FT_STATUS;
            }
            unsafe { _FT_SetVIDPID(vendor, product) }
        }
    }
}
