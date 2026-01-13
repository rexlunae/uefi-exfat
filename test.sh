#!/bin/bash
# Test script for uefi-exfat

set -e

echo "Running tests for uefi-exfat..."

# Run tests on host platform
cargo test --lib --target x86_64-unknown-linux-gnu

echo "All tests passed!"
