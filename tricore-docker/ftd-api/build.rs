use std::{env, path::PathBuf};

fn main() {
    let (header_path, clang_args) = match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => {
            let header_file = "../native/ftdi-source-windows-x64/ftd2xx.h";
            // If we build on windows for windows everything is fine
            #[cfg(target_os = "windows")]
            const CLANG_ARGS: [&str; 0] = [];

            // If we build on linux for windows, we need to specify import where
            // to find the header files
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            const CLANG_ARGS: [&str; 4] = [
                "-I/xwin/crt/include",
                "-I/xwin/sdk/include/ucrt",
                "-I/xwin/sdk/include/um",
                "-I/xwin/sdk/include/shared",
            ];
            (header_file, &CLANG_ARGS[..])
        }
        "linux" => {
            let header_file = "../native/ftdi-source-linux-x64/ftd2xx.h";
            (header_file, &[][..])
        }
        system => panic!("Not supported os {system}"),
    };

    println!("cargo:rerun-if-changed={header_path}");

    // println!("debug {:?}", clang_args);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header_path)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_type("FT_STATUS")
        .allowlist_type("LPDWORD")
        .allowlist_type("FT_HANDLE")
        .allowlist_type("PCHAR")
        .allowlist_type("DWORD")
        .allowlist_type("LPVOID")
        .allowlist_type("BOOL")
        .allowlist_type("UCHAR")
        .allowlist_type("USHORT")
        .allowlist_type("LPOVERLAPPED")
        .allowlist_type("WORD")
        .allowlist_type("LPWORD")
        .allowlist_type("PFT_PROGRAM_DATA")
        .allowlist_type("LPSECURITY_ATTRIBUTES")
        .allowlist_type("LPCTSTR")
        .allowlist_type("FT_DEVICE")
        .allowlist_type("LPFTDCB")
        .allowlist_type("FTTIMEOUTS")
        .allowlist_type("PULONG")
        .allowlist_type("PUCHAR")
        .allowlist_type("LPFTCOMSTAT")
        .allowlist_type("LPLONG")
        .allowlist_type("FT_DEVICE_LIST_INFO_NODE")
        .allowlist_var("FT_OK")
        .allowlist_var("FT_OTHER_ERROR")
        .allowlist_var("PVOID")
        .allowlist_function("FT_ListDevices")
        .allowlist_function("FT_Close")
        .allowlist_function("FT_GetDriverVersion")
        .allowlist_function("FT_GetLibraryVersion")
        .allowlist_function("FT_Read")
        .allowlist_function("FT_Write")
        .allowlist_function("FT_SetBitMode")
        .allowlist_function("FT_SetFlowControl")
        .allowlist_function("FT_SetLatencyTimer")
        .allowlist_function("FT_SetTimeouts")
        .allowlist_function("FT_SetChars")
        .allowlist_function("FT_SetUSBParameters")
        .allowlist_function("FT_GetQueueStatus")
        .allowlist_function("FT_ResetDevice")
        .allowlist_function("FT_Open")
        .allowlist_function("FT_CreateDeviceInfoList")
        .allowlist_function("FT_GetDeviceInfoDetail")
        .clang_args(clang_args)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
