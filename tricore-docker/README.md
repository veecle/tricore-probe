# Running DAS tools on linux

DAS is only available for windows - unless you simulate a windows environment
in a docker container, provide a patched version of the FTD2XX library and let
it communicate with an FTDI server on your host over OS pipes.

**This code needs a cleanup** - we recommend you get familiar with the windows setup first
before you enter this flimsy, undocumented region of prototyping.

# I am brave, let me!
You conciously or accidentaly decided to keep going, so how to get started?
Note: This setup is most likely not ideal, but the first working result after a 
long path of failed attempts.

## Prepare for building the artifacts
You need to build two artifacts for windows, specifically for the `x86_64-pc-windows-msvc`
target. This trivially cannot be done on linux, the desired option would be to 
use the [`cross`](https://github.com/cross-rs/cross) project but I was unable to get it running. You thus want to build
a docker container using the `xwin` project (to provide required header files) 
that allows you to build rust applications with the given target.

```bash
docker build .. -f xwin.Dockerfile --tag veecle/xwin
```

Note that this uses the parenting folder as the build context to prime the cargo
registry caches inside the container.

## Build artifacts for the simulated windows environment
You need to build two artifacts for the simulated windows environment. The commands
are already prepared in `build_artifacts.sh`

## Build the simulated windows environment
This folder should now contain a folder `artifacts` with two files: `ftd2xx.dll`
and `win-daemon.exe`. You can now build the virtual environment with

```bash
docker build . --tag veecle/flash-tricore
```

**Note:** This repository ships a Memtool and DAS installation in a wine 
environment. To use this setup, make sure you checked the terms and conditions
of these programs by e.g. downloading and installing them yourself, and then accept
them for this setup by setting the required build argument with `--build-arg=AGREE_INFINEON_TERMS=1`
when building the docker image.

## Install the correct version of tricore-probe
Install tricore-probe with default features disabled and the `docker` feature enabled
to use the docker container as a backend instead of the native windows implementation.

Ét voila! If everything worked, tricore-probe now runs on your linux machine.