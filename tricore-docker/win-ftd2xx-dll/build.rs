use std::env;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let out_dir = PathBuf::from_str(&env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();
    let ordinals = out_dir.join("ordinals.def");
    println!(
        "cargo:rustc-cdylib-link-arg=/DEF:{}",
        ordinals.as_path().display()
    );
}
