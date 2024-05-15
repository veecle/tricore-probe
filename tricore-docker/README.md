# Running DAS tools on linux

DAS is only available for windows - unless you simulate a Windows environment
in a docker container, provide a patched version of the FTD2XX library and let
it communicate with an FTDI server on your host over OS pipes.

**This code needs a cleanup** - we recommend you get familiar with the windows setup first
before you enter this flimsy, undocumented region of prototyping.

# I am brave, let me!
You consciously or accidentally decided to keep going, so how to get started?
Note: This setup is most likely not ideal, but the first working result after a 
long path of failed attempts.

## Requirements
`libudev` needs to be installed for linux systems.

Apt:
```bash
sudo apt-get install libudev-dev
```
Dnf:
```bash
sudo dnf install systemd-devel
```


## Prepare for building the artifacts
You need to build one artifact for windows, specifically for the `x86_64-pc-windows-msvc`
target. This trivially cannot be done on linux, the desired option would be to 
use the [`cross`](https://github.com/cross-rs/cross) project but I was unable to get it running. You thus want to build
a docker container using the `xwin` project (to provide required header files) 
that allows you to build rust applications with the given target.

```bash
docker build .. -f xwin.Dockerfile --tag veecle/xwin
```

Note that this uses the parenting folder as the build context to prime the cargo
registry caches inside the container.

## Build artifact for the simulated Windows environment
You need to build one artifact for the simulated Windows environment. The commands
are already prepared in `build_artifacts.sh`

## Build the simulated Windows environment
This folder should now contain a folder `artifacts` with one file: `win-daemon.exe`.
The simulated Windows environment requires the installers for the DAS server and AurixFlasher tool.
Specifically, [`aurixflashersoftwaretool_1.0.10_Windows_x64.msi`](https://softwaretools.infineon.com/tools/com.ifx.tb.tool.aurixflashersoftwaretool) and [`DAS_V8_0_5_SETUP.exe`](https://www.infineon.com/cms/en/product/promopages/das/) need to be places into the `tricore-docker` directory.


You can now build the virtual environment with

```bash
docker build . --tag veecle/flash-tricore
```

**Note:** The `veecle/flash-tricore` container will contain an AurixFlasher and DAS installation in a wine 
environment. To use this setup, make sure you checked the terms and conditions
of these programs by e.g. downloading and installing them yourself, and then accept
them for this setup by setting the required build argument with `--build-arg=AGREE_INFINEON_TERMS=1`
when building the docker image.

## Install the correct version of tricore-probe
Install tricore-probe with default features disabled and the `docker` feature enabled
to use the docker container as a backend instead of the native windows implementation.

Ét voila! If everything worked, tricore-probe now runs on your linux machine.