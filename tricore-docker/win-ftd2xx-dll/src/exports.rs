#![allow(non_snake_case)]
#![allow(unsupported_calling_conventions)]
#![allow(non_camel_case_types)]
use ftd_api::ftd2xx::{
    BOOL, DWORD, FTTIMEOUTS, FT_DEVICE, FT_DEVICE_LIST_INFO_NODE, FT_HANDLE, FT_STATUS, HANDLE,
    LPCTSTR, LPDWORD, LPFTCOMSTAT, LPFTDCB, LPLONG, LPOVERLAPPED, LPSECURITY_ATTRIBUTES, LPVOID,
    LPWORD, PCHAR, PFT_PROGRAM_DATA, PUCHAR, PULONG, PVOID, UCHAR, ULONG, USHORT, WORD,
};
type void = ();
use crate::implementations::{
    close, create_device_info_list, get_device_info_detail, get_driver_version, get_queue_status,
    library_version, open, read, reset_device, set_bit_mode, set_chars, set_flow_control,
    set_latency_timer, set_timeouts, set_usb_parameters, write,
};

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
