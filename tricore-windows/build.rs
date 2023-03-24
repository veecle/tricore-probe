macro_rules! set_env_if_not_defined {
    ($env_name: expr, $default: expr) => {
        if std::env::var($env_name).is_ok() {
            println!("cargo:rerun-if-env-changed={}", $env_name);
        } else {
            println!(
                "cargo:warning={} env variable not set, assuming default path {}",
                $env_name, $default
            );
            println!("cargo:rustc-env={}={}", $env_name, $default);
        };
    };
}

fn main() {
    set_env_if_not_defined!(
        "MEMTOOL_PATH",
        "C:\\Program Files\\Infineon\\Memtool 2021\\IMTMemtool.exe"
    );
}
