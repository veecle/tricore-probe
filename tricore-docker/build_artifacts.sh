# exit when any command fails
set -e

# Build artifacts required for emulation through wine in linux
docker run \
    --rm \
    --user $(id -u):$(id -g) \
    --mount type=bind,source="$(pwd)/..",target=/build \
    --workdir /build \
    --env RUST_LOG=trace \
    --env DAS_HOME="C:\\DAS64" \
    --env AURIX_FLASHER_PATH="C:\\Infineon\\AURIXFlasherSoftwareTool\\AURIXFlasher.exe" \
    veecle/xwin \
    cargo build --manifest-path tricore-docker/win-daemon/Cargo.toml -Z build-std --target x86_64-pc-windows-msvc --target-dir tricore-docker/docker-build-daemon

# Copy artifacts so we don't have to pass the whole build folder in the build context
rm -rf artifacts
mkdir artifacts
cp docker-build-daemon/x86_64-pc-windows-msvc/debug/win-daemon.exe artifacts/
