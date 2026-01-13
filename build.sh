#!/bin/bash
# Build script for uefi-exfat

set -e

echo "Building uefi-exfat for UEFI target..."

# Check if nightly toolchain is installed
if ! rustup toolchain list | grep -q nightly; then
    echo "Installing nightly toolchain..."
    rustup toolchain install nightly
fi

# Check if rust-src is installed
if ! rustup component list --toolchain nightly | grep -q "rust-src.*installed"; then
    echo "Installing rust-src component..."
    rustup component add rust-src --toolchain nightly
fi

# Build for UEFI target
BUILD_TYPE="${1:-debug}"

if [ "$BUILD_TYPE" = "release" ]; then
    echo "Building release version..."
    cargo +nightly build --release \
        --target x86_64-unknown-uefi \
        -Z build-std=core,compiler_builtins,alloc \
        -Z build-std-features=compiler-builtins-mem
    echo "Build complete: target/x86_64-unknown-uefi/release/libuefi_exfat.a"
else
    echo "Building debug version..."
    cargo +nightly build \
        --target x86_64-unknown-uefi \
        -Z build-std=core,compiler_builtins,alloc \
        -Z build-std-features=compiler-builtins-mem
    echo "Build complete: target/x86_64-unknown-uefi/debug/libuefi_exfat.a"
fi

echo "Done!"
