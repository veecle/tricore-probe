# Tricore-Probe: Run rust effortlessly on Tricore chips

`tricore-probe` is an effort to deploy and debug rust programs with little effort
on Tricore chips. It uses publicly available Infineon tools to interface with the
chips debug controller. As its name suggests, it is inspired by [`probe-run`](https://crates.io/crates/probe-run) and depends 
on the [`defmt`](https://defmt.ferrous-systems.com/) framework to integrate seemlessly just as `probe-run` does.

This program can be configured as a [runner](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner). 
Check [`main.rs`](src/main.rs) for additional configuration options.

A simple runner config for a TC375 lite kit could look like this:

```toml
[target.tc162-htc-none]
runner = "tricore-probe"
# Default dwarf version of v0.2.0 HighTec compiler is version 2 which is
# incompatible with defmt location information
rustflags = ["-Z", "dwarf-version=4"]

[build]
target = "tc162-htc-none"
```

Then you can call `cargo run` in your project root which will run the built .elf on your board via `tricore-probe`.

# Requirements

This program has various dependencies that must be installed for this program to 
work properly:
1. [Infineon DAS tool version 8.0.5](https://www.infineon.com/cms/en/product/promopages/das/)
2. [Infineon AURIX™ Flasher Software Tool](https://softwaretools.infineon.com/tools/com.ifx.tb.tool.aurixflashersoftwaretool): 

When installing this program make sure the environment variable `AURIX_FLASHER_PATH` is set
to the AurixFlasher executable (`<your-path>\AURIXFlasher.exe`). If the environment variable is not set 
a default path is assumed.

3. [`defmt-print` CLI utility](https://crates.io/crates/defmt-print)
4. `objcopy` CLI utility (obtain e.g. as part of the [MinGW-w64](https://www.mingw-w64.org/) project)
5. `addr2line` CLI utility (obtain e.g. as part of the [MinGW-w64](https://www.mingw-w64.org/) project)
6. Rust toolchain
7. [LLVM](https://github.com/llvm/llvm-project/releases) (also set `LIBCLANG_PATH` to `<your-path>\LLVM\lib`)

# Quickstart

```
> git clone https://github.com/veecle/tricore-probe.git
> cd tricore-probe
> cargo build
> .\target\debug\tricore-probe.exe <your-executable>.elf --list-devices
Found 1 devices:
Device 0: "DAS JDS AURIX LITE KIT V2.0 (TC375) LK7KFCF1"
```

In case of a simple example based on the [Bluewind blinky](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples/tree/main/blinky) the output will look something like this:

```
> .\target\debug\tricore-probe.exe blinky.elf
DEBUG power on reset
└─ bw_r_drivers_tc37x::ssw::tc0::init_clock @ C:\Users\andra\.cargo\registry\src\github.com-d2f9efa20490c5c8\bw-r-drivers-tc37x-0.2.0\src\ssw\tc0.rs:12
INFO  LED2 toggle
└─ blinky::main @ src\main.rs:46
```

For more sample code refer to the Bluewind [bare-metal examples](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples) and to the Veecle [PXROS examples](https://github.com/veecle/veecle-pxros/tree/main/examples).

# Known flaws

This application is still in development and has some known drawbacks. If you 
find something that is not listed here, feel free to open an issue or leave us a
message.

1. AurixFlasher and DAS are only available on windows. This is a big problem since we at Veecle
require a working linux version. We thus created a dockerized setup that allows 
us to use these tools in linux with little overhead. Check [tricore-docker/README.md](tricore-docker/README.md)
for more information.


## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.