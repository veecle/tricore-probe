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
    cargo build -p tricore-probe -Z build-std --target x86_64-pc-windows-msvc --target-dir tricore-docker/docker-build-exe --features in_docker


docker run \
    --rm \
    --user $(id -u):$(id -g) \
    --mount type=bind,source="$(pwd)",target=/build \
    --workdir /build \
    --env RUST_LOG=trace \
    --env DAS_HOME="C:\\DAS64" \
    --env AURIX_FLASHER_PATH="C:\\Infineon\\AURIXFlasherSoftwareTool\\AURIXFlasher.exe" \
    veecle/xwin \
    cargo +nightly-2023-09-20 install defmt-print -Z build-std --target x86_64-pc-windows-msvc --root docker-build-exe

docker run \
    --rm \
    --user $(id -u):$(id -g) \
    --mount type=bind,source="$(pwd)",target=/build \
    --workdir /build \
    --env RUST_LOG=trace \
    --env DAS_HOME="C:\\DAS64" \
    --env AURIX_FLASHER_PATH="C:\\Infineon\\AURIXFlasherSoftwareTool\\AURIXFlasher.exe" \
    veecle/xwin \
    cargo +nightly-2023-09-20 install addr2line -Z build-std --target x86_64-pc-windows-msvc --root docker-build-exe --features bin  --git https://github.com/gimli-rs/addr2line


# Copy artifacts so we don't have to pass the whole build folder in the build context
rm -rf artifacts
mkdir artifacts
cp docker-build-exe/x86_64-pc-windows-msvc/debug/tricore-probe.exe artifacts/
cp docker-build-exe/bin/defmt-print.exe artifacts/
cp docker-build-exe/bin/addr2line.exe artifacts/