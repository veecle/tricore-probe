//! This crate exports patched FTDI functions that communicate over RPC
//! rather than directly with the FTDI device.
//!
//! This is exported as a DLL and used to patch how the TAS/DAS server
//! communicate with the FTDI device.
#![allow(unsupported_calling_conventions)]
#![allow(clippy::too_many_arguments)]
#![allow(non_snake_case)]
#![allow(unsupported_calling_conventions)]
#![allow(non_camel_case_types)]

use libftd2xx_ffi::*;
type void = ();

mod implementations;
use implementations::*;

dll_export::patch! {"../header.h"
    FT_CreateDeviceInfoList custom(create_device_info_list),
    FT_GetDeviceInfoDetail custom(get_device_info_detail),
    FT_Open custom(open),
    FT_ResetDevice custom(reset_device),
    FT_GetQueueStatus custom(get_queue_status),
    FT_SetUSBParameters custom(set_usb_parameters),
    FT_SetChars custom(set_chars),
    FT_SetTimeouts custom(set_timeouts),
    FT_SetLatencyTimer custom(set_latency_timer),
    FT_SetFlowControl custom(set_flow_control),
    FT_SetBitMode custom(set_bit_mode),
    FT_Write custom(write),
    FT_Read custom(read),
    FT_GetLibraryVersion custom(library_version),
    FT_GetDriverVersion custom(get_driver_version),
    FT_Close custom(close),
}
