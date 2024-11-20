# Build this docker file with the parenting folder as the build context!

# Use the official rust image with xwin (https://github.com/Jake-Shadle/xwin) as base.
#
FROM docker.io/library/rust:1.74.0-slim-bookworm AS builder-base

ENV KEYRINGS /usr/local/share/keyrings

RUN set -eux; \
    mkdir -p $KEYRINGS; \
    apt-get update && apt-get install -y gpg curl; \
    # clang/lld/llvm
    curl --fail https://apt.llvm.org/llvm-snapshot.gpg.key | gpg --dearmor > $KEYRINGS/llvm.gpg; \
    echo "deb [signed-by=$KEYRINGS/llvm.gpg] http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-18 main" > /etc/apt/sources.list.d/llvm.list;

RUN set -eux; \
    # Skipping all of the recommended reduces total images size by ~300MiB
    apt-get update && apt-get install --no-install-recommends -y \
        clang-18 \
        # llvm-ar
        llvm-18 \
        lld-18 \
        # Unpack xwin
        tar; \
    # ensure that clang/clang++ are callable directly
    ln -s clang-18 /usr/bin/clang && ln -s clang /usr/bin/clang++ && ln -s lld-18 /usr/bin/ld.lld; \
    # We also need to setup symlinks ourselves for the MSVC shims because they aren't in the debian packages
    ln -s clang-18 /usr/bin/clang-cl && ln -s llvm-ar-18 /usr/bin/llvm-lib && ln -s lld-link-18 /usr/bin/lld-link; \
    # Verify the symlinks are correct
    clang++ -v; \
    ld.lld -v; \
    # Doesn't have an actual -v/--version flag, but it still exits with 0
    llvm-lib -v; \
    clang-cl -v; \
    lld-link --version; \
    # Use clang instead of gcc when compiling and linking binaries targeting the host (eg proc macros, build files)
    update-alternatives --install /usr/bin/cc cc /usr/bin/clang 100; \
    update-alternatives --install /usr/bin/c++ c++ /usr/bin/clang++ 100; \
    update-alternatives --install /usr/bin/ld ld /usr/bin/ld.lld 100; \
    apt-get remove -y --auto-remove; \
    rm -rf /var/lib/apt/lists/*;

RUN set -eux; \
    xwin_version="0.5.2"; \
    xwin_prefix="xwin-$xwin_version-x86_64-unknown-linux-musl"; \
    # Install xwin to cargo/bin via github release. Note you could also just use `cargo install xwin`.
    curl --fail -L https://github.com/Jake-Shadle/xwin/releases/download/$xwin_version/$xwin_prefix.tar.gz | tar -xzv -C /usr/local/cargo/bin --strip-components=1 $xwin_prefix/xwin; \
    # Splat the CRT and SDK files to /xwin/crt and /xwin/sdk respectively
    xwin --accept-license splat --output /xwin; \
    # Remove unneeded files to reduce image size
    rm -rf .xwin-cache /usr/local/cargo/bin/xwin;


# Note that we're using the full target triple for each variable instead of the
# simple CC/CXX/AR shorthands to avoid issues when compiling any C/C++ code for
# build dependencies that need to compile and execute in the host environment
ENV CC_x86_64_pc_windows_msvc="clang-cl" \
    CXX_x86_64_pc_windows_msvc="clang-cl" \
    AR_x86_64_pc_windows_msvc="llvm-lib" \
    # Note that we only disable unused-command-line-argument here since clang-cl
    # doesn't implement all of the options supported by cl, but the ones it doesn't
    # are _generally_ not interesting.
    CL_FLAGS="-Wno-unused-command-line-argument -fuse-ld=lld-link -I/xwin/crt/include -I/xwin/sdk/include/ucrt -I/xwin/sdk/include/um -I/xwin/sdk/include/shared" \
    # Let cargo know what linker to invoke if you haven't already specified it
    # in a .cargo/config.toml file
    CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="lld-link" \
    CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS="-Lnative=/xwin/crt/lib/x86_64 -Lnative=/xwin/sdk/lib/um/x86_64 -Lnative=/xwin/sdk/lib/ucrt/x86_64"

# These are separate since docker/podman won't transform environment variables defined in the same ENV block
ENV CFLAGS_x86_64_pc_windows_msvc="$CL_FLAGS" \
    CXXFLAGS_x86_64_pc_windows_msvc="$CL_FLAGS"

RUN rustup target add x86_64-pc-windows-msvc

# Use cargo-chef to enable caching of dependencies.
RUN cargo install cargo-chef --locked

WORKDIR /cargo-bin

# Install defmt-print and addr2line for Windows.
# The executables are later copied to the runner image.
RUN cargo install defmt-print --target x86_64-pc-windows-msvc --locked --root /cargo-bin
# Addr2line binary has no crates.io release yet.
# Until it has, we use git.
RUN cargo install addr2line --target x86_64-pc-windows-msvc --features bin --git https://github.com/gimli-rs/addr2line --rev ade443f2ba6ce26a5dfddde1210f41aa46d96b92 --locked --root /cargo-bin

WORKDIR /build


FROM builder-base AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM builder-base AS builder
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-pc-windows-msvc --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release -p tricore-probe --target x86_64-pc-windows-msvc --features in_docker


FROM ubuntu:jammy AS runner

# Silences interactive installers.
ENV DEBIAN_FRONTEND=noninteractive

# Set timezone.
ENV TZ=Europe/Berlin

RUN dpkg --add-architecture i386 && apt update && apt install -y wget
RUN mkdir -pm755 /etc/apt/keyrings
RUN wget -O /etc/apt/keyrings/winehq-archive.key https://dl.winehq.org/wine-builds/winehq.key
RUN wget -NP /etc/apt/sources.list.d/ https://dl.winehq.org/wine-builds/ubuntu/dists/jammy/winehq-jammy.sources
RUN apt update
# Virtual X server to make applications not crash without display.
RUN apt install -y xvfb
RUN apt install -y binutils
# Make etc.
RUN apt install -y build-essential
# Wine.
RUN apt install -y winehq-stable
# Mutli-architecture libraries required to build wineftd2xx.
RUN apt install -y gcc-multilib
# Compiler for wineftd2xx.
RUN apt install -y wine-stable-dev
# msiextract tool used to extract the AurixFlasher installer.
RUN apt install -y msitools

ARG AGREE_INFINEON_TERMS=0
RUN if [ "${AGREE_INFINEON_TERMS}" -ne "1" ]; then echo "This installation ships an AurixFlasher and DAS installation. Agree with these conditions by setting environment variable AGREE_INFINEON_TERMS to 1" && exit 1; fi

WORKDIR "/root"
# Copy DAS setup and install script.
COPY ["tricore-docker/DAS_V8_0_5_SETUP.exe", "."]
COPY ["tricore-docker/DAS_8_0_5_installer_script.qs", "."]

# Install DAS, this will accept the DAS license agreement, notice and terms of use.
RUN wine DAS_V8_0_5_SETUP.exe --script DAS_8_0_5_installer_script.qs -d --al

# Copy Aurix flasher setup.
COPY ["tricore-docker/aurixflashersoftwaretool_1.0.10_Windows_x64.msi", "."]
# Extract Aurix flasher.
RUN msiextract -C /root/.wine/dosdevices/c:/Infineon/AURIXFlasherSoftwareTool aurixflashersoftwaretool_1.0.10_Windows_x64.msi
# Install wine mono required for AurixFlasher.
RUN wget -O ./wine-mono-9.1.0-x86.msi https://dl.winehq.org/wine/wine-mono/9.1.0/wine-mono-9.1.0-x86.msi
RUN msiexec /i wine-mono-9.1.0-x86.msi

# Build FTD2XX DLL shim.
COPY ["tricore-docker/wineftd2xx", "./wineftd2xx"]
WORKDIR "./wineftd2xx"
RUN make
RUN make install

# The DAS installer places TAS server executables in both directories, so we place the DLL in both directories.
RUN cp "/root/wineftd2xx/ftd2xx.dll.so" "/root/.wine/dosdevices/c:/DAS64/servers/ftd2xx.dll"
RUN cp "/root/wineftd2xx/ftd2xx.dll.so" "/root/.wine/dosdevices/c:/windows/system32/ftd2xx.dll"

WORKDIR "/root/.wine/dosdevices/c:/"
COPY --from=builder /build/target/x86_64-pc-windows-msvc/release/tricore-probe.exe .
COPY --from=builder-base /cargo-bin/bin/defmt-print.exe .
COPY --from=builder-base /cargo-bin/bin/addr2line.exe .

# Wine debug channels https://wiki.winehq.org/Debug_Channels
# Turn on "+relay" for only one application: "win-daemon.exe:+relay"
# FTD2XX dll logging: "trace+ftd2xx"
# Disables all Wine debug logging.
ENV WINEDEBUG="-all"

# DAS home directory.
ENV DAS_HOME="C:\\DAS64"

# AurixFlasher path.
ENV AURIX_FLASHER_PATH="C:\\Infineon\\AURIXFlasherSoftwareTool\\AURIXFlasher.exe"

CMD ["bash"]


