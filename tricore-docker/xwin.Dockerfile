# Build this docker file with the parenting folder as the build context!
# We'll just use the official Rust image
FROM docker.io/library/rust:1.74.0-slim

ENV KEYRINGS /usr/local/share/keyrings

RUN set -eux; \
    mkdir -p $KEYRINGS; \
    apt-get update && apt-get install -y gpg curl; \
    # clang/lld/llvm
    curl --fail https://apt.llvm.org/llvm-snapshot.gpg.key | gpg --dearmor > $KEYRINGS/llvm.gpg; \
    echo "deb [signed-by=$KEYRINGS/llvm.gpg] http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-13 main" > /etc/apt/sources.list.d/llvm.list;

RUN set -eux; \
    dpkg --add-architecture i386; \
    # Skipping all of the "recommended" cruft reduces total images size by ~300MiB
    apt-get update && apt-get install --no-install-recommends -y \
        clang-13 \
        # llvm-ar
        llvm-13 \
        lld-13 \
        # Unpack xwin
        tar; \
    # ensure that clang/clang++ are callable directly
    ln -s clang-13 /usr/bin/clang && ln -s clang /usr/bin/clang++ && ln -s lld-13 /usr/bin/ld.lld; \
    # We also need to setup symlinks ourselves for the MSVC shims because they aren't in the debian packages
    ln -s clang-13 /usr/bin/clang-cl && ln -s llvm-ar-13 /usr/bin/llvm-lib && ln -s lld-link-13 /usr/bin/lld-link; \
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

# Retrieve the std lib for the target
RUN rustup toolchain install --force-non-host stable-x86_64-pc-windows-msvc

RUN set -eux; \
    xwin_version="0.2.10"; \
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

RUN cargo install defmt-print --target x86_64-pc-windows-msvc --locked
RUN cargo install addr2line --target x86_64-pc-windows-msvc --features bin --git https://github.com/gimli-rs/addr2line --locked

# Built the binaries once to prime the cargo registry cache
WORKDIR /src
COPY . /src
RUN cargo build -p tricore-probe --target x86_64-pc-windows-msvc --features in_docker



RUN rm -r *