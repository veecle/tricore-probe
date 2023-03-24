use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory

    let cwd = PathBuf::from_str(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();
    let search_path = match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => cwd.join("ftdi-source-windows-x64/Static/amd64"),
        "linux" => cwd.join("ftdi-source-linux-x64/build"),
        system => panic!("Not supported os {system}"),
    };

    println!(
        "cargo:rustc-link-search={}",
        search_path.as_path().display()
    );
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ftd2xx");
}
