use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("bindings.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() != "windows" {
        // Include the pregenerated file, to ease development on linux
        // Note that this library will currently not run on linux since it requires
        // mcdxdas.dll to be installed
        let pregenerated = include_bytes!("pregenerated.rs");
        let mut bindings = File::create(bindings_file).unwrap();
        bindings.write_all(pregenerated).unwrap();
    } else {
        // If we build on linux, we need to specify import where
        // to find the header files. We assume the xwin project is in use to provide
        // the required header files
        #[cfg(target_os = "linux")]
        const CLANG_ARGS: [&str; 4] = [
            "-I/xwin/crt/include",
            "-I/xwin/sdk/include/ucrt",
            "-I/xwin/sdk/include/um",
            "-I/xwin/sdk/include/shared",
        ];

        // When building on windows, all the header file should be in the path
        // already
        #[cfg(target_os = "windows")]
        const CLANG_ARGS: [&str; 0] = [];

        println!("cargo:rerun-if-changed=mcd_demo_basic_120412/src/mcd_api.h");

        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("mcd_demo_basic_120412/src/mcd_api.h")
            .clang_args(CLANG_ARGS)
            .dynamic_library_name("DynamicMCDxDAS")
            .derive_default(true)
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        bindings
            .write_to_file(bindings_file)
            .expect("Couldn't write bindings!");
    }
}
