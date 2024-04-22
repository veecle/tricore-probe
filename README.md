# Tricore-Probe: Run rust effortlessly on Tricore chips

`tricore-probe` is an effort to deploy and debug rust programs with little effort
on Tricore chips. It uses publicly available Infineon tools to interface with the
chips debug controller. As its name suggests, it is inspired by [`probe-run`](https://crates.io/crates/probe-run) and depends 
on the [`defmt`](https://defmt.ferrous-systems.com/) framework to integrate seemlessly just as `probe-run` does.

This program can be configured as a [runner](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner). 
Check [`main.rs`](src/main.rs) for additional configuration options.

# Requirements

This program has various dependencies that must be installed for this program to 
work properly:
1. [Infineon DAS tool](https://www.infineon.com/cms/en/product/promopages/das/#!?fileId=db3a30431ed1d7b2011f469ac40e56af)
2. [Infineon Memtool 2021.08](https://www.infineon.com/cms/en/tools/aurix-tools/free-tools/infineon/): 

When installing this program make sure the environment variable `MEMTOOL_PATH` is set
to the memtool executable (`<your-path>\Infineon\Memtool 2021\IMTMemtool.exe`). If the environment variable is not set 
a default path is assumed that should work if the install path was not changed 
during installation.

3. [`defmt-print` CLI utility](https://crates.io/crates/defmt-print)
4. `objcopy` CLI utility (obtain e.g. as part of the [MinGW-w64](https://www.mingw-w64.org/) project)
5. `addr2line` CLI utility (obtain e.g. as part of the [MinGW-w64](https://www.mingw-w64.org/) project)
6. Rust nightly toolchain
7. [LLVM](https://github.com/llvm/llvm-project/releases) (also set `LIBCLANG_PATH` to `<your-path>\LLVM\lib`)

# Known flaws

This application is still in development and has some known drawbacks. If you 
find something that is not listed here, feel free to open an issue or leave us a
message.

1. For flashing to work seemlessly the chip you are trying to flash should be 
configured as the default settings in the `Memtool` application.
2. Memtool and DAS are only available on windows. This is a big problem since we at Veecle
require a working linux version. We thus created a dockerized setup that allows 
us to use these tools in linux with little overhead. Check [tricore-docker/README.md](tricore-docker/README.md)
for more information.


## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.