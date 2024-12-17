# Tricore-Probe: Run rust effortlessly on Tricore chips

`tricore-probe` is an effort to deploy and debug rust programs with little effort
on Tricore chips. It uses publicly available Infineon tools to interface with the
chips debug controller. As its name suggests, it is inspired by [`probe-run`](https://crates.io/crates/probe-run) and depends 
on the [`defmt`](https://defmt.ferrous-systems.com/) framework to integrate seamlessly just as `probe-run` does.

### Platform support
Currently only Windows and Linux are supported.

# Installation

## Windows

### Requirements

1. [Infineon DAS tool version 8.1.4](https://www.infineon.com/cms/en/product/promopages/das/)
    Please make sure the `DAS_HOME` environment variable points to the DAS tool installation directory.
2. [Infineon AURIX™ Flasher Software Tool 3.0.0](https://softwaretools.infineon.com/tools/com.ifx.tb.tool.aurixflashersoftwaretool)
   Please make sure the `AURIX_FLASHER_PATH` environment variable points to the AurixFlasher executable (`<your-path>\AURIXFlasher.exe`).
3. [`defmt-print` CLI utility](https://crates.io/crates/defmt-print): `cargo install defmt-print`
4. `objcopy` CLI utility (obtain e.g. as part of the [MinGW-w64](https://www.mingw-w64.org/) project)
5. `addr2line` CLI utility (obtain e.g. as part of the [MinGW-w64](https://www.mingw-w64.org/) project)
6. Rust toolchain
7. [bindgen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) 

### Installation
Install `tricore-probe`:
```shell
cargo install tricore-probe --git https://github.com/veecle/tricore-probe --version 0.2.0
```

## Linux

The Linux setup is not officially supported by Infineon and thus might not work as expected.
Please report any bugs or issues you encounter with the Linux setup only to this repository, not to Infineon.

### Requirements

1. Place the [Infineon DAS tool version 8.1.4](https://www.infineon.com/cms/en/product/promopages/das/) installer (`DAS_V8_1_4_SETUP.exe`) in [tricore-docker](tricore-docker).
2. Place the [Infineon AURIX™ Flasher Software Tool 3.0.0](https://softwaretools.infineon.com/tools/com.ifx.tb.tool.aurixflashersoftwaretool) installer (`AURIXFlasherSoftwareTool-setup_3.0.0_20241030-1737.exe`) in [tricore-docker](tricore-docker).
3. `objcopy` CLI utility
4. Rust toolchain
5. `libudev` library (`libudev-dev` on Ubuntu, `systemd-libs` on Fedora)

### Installation
Clone the repository and place the DAS tool installer and AurixFlasher installer into the [tricore-docker](tricore-docker) directory.
Build the docker container running the DAS tool, AurixFlasher and other utilities.

**Note:**
The `veecle/flash-tricore` container will contain an AurixFlasher and DAS installation in a Wine environment.
To use this setup, make sure you checked the terms and conditions of these programs and accept them by setting the required build argument with `--build-arg=AGREE_INFINEON_TERMS=1` when building the docker image.

```shell
docker build . --tag veecle/flash-tricore --build-arg=AGREE_INFINEON_TERMS=1 -f tricore-docker/Dockerfile
```

Install `tricore-probe`:
```shell
cargo install tricore-probe --git https://github.com/veecle/tricore-probe --version 0.2.1
```

### Attribution

The Linux setup is based on a modified version of the [`wineftd2xx` project](https://github.com/brentr/wineftd2xx) by [`brentr`](https://github.com/brentr).

## Quickstart

```
> tricore-probe --list-devices
Found 1 devices:
Device 0: "DAS JDS AURIX LITE KIT V2.0 (TC375) LK7KFCF1"
```

In case of a simple example based on the [Bluewind blinky](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples/tree/main/blinky) the output will look something like this:

```
> tricore-probe blinky.elf
DEBUG power on reset
└─ bw_r_drivers_tc37x::ssw::tc0::init_clock @ C:\Users\andra\.cargo\registry\src\github.com-d2f9efa20490c5c8\bw-r-drivers-tc37x-0.2.0\src\ssw\tc0.rs:12
INFO  LED2 toggle
└─ blinky::main @ src\main.rs:46
```

For more sample code refer to the Bluewind [bare-metal examples](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples) and to the Veecle [PXROS examples](https://github.com/veecle/veecle-pxros/tree/main/examples).

For applications not running on all the available cores, you can specify the number of active cores in the application with a CLI flag in order to prevent abrupt exit from the `rtt` session (by default, all of the cores available to the MCU are used):
```
> tricore-probe --cores <n> app.elf 
```
Note that this parameter only works for applications running on contiguous cores. For example, on a tri-core processor, with an application only starting `core0` and `core2`, this session will stop anyway:
```
> tricore-probe --cores 2 app.elf 
```

## Cargo runner
This program can be configured as a [runner](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner).
Check [`main.rs`](src/main.rs) or run `tricore-probe --help` for additional configuration options.

A simple runner config for a TC375 lite kit could look like this:

```toml
[target.tc162-htc-none]
runner = "tricore-probe"
# The default dwarf version of the HighTec compiler is version 2 in v0.2.0
# and version 3 in v1.0.0, both versions being incompatible with defmt location information.
rustflags = ["-Z", "dwarf-version=4"]

[build]
target = "tc162-htc-none"
```

Then you can call `cargo run` in your project root which will run the built .elf on your board via `tricore-probe`.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
