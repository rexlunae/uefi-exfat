# uefi-exfat

An exFAT driver for UEFI written in Rust.

## Overview

This project implements a UEFI driver for the exFAT filesystem, written entirely in Rust. It provides the core structures and operations needed to read exFAT filesystems in a UEFI environment.

## Features

- **exFAT Filesystem Support**: Implements core exFAT structures including:
  - Boot sector and parameter block parsing
  - File Allocation Table (FAT) handling
  - Directory entry structures (File, Stream Extension, File Name)
  - File metadata and attributes
  
- **UEFI Integration**: Designed for UEFI environments with:
  - Global allocator support
  - No standard library (`no_std`)
  - UEFI-compatible panic handler
  - EFI calling convention

- **Type-Safe API**: Provides safe Rust abstractions for:
  - Volume operations
  - File operations (read, seek)
  - Directory operations
  - Cluster-to-LBA conversion

## Building

### Prerequisites

- Rust nightly toolchain
- `rust-src` component

```bash
# Install nightly toolchain
rustup toolchain install nightly

# Add rust-src component
rustup component add rust-src --toolchain nightly
```

### Build for UEFI

Build the driver for the UEFI target:

```bash
cargo +nightly build --target x86_64-unknown-uefi -Z build-std=core,compiler_builtins,alloc -Z build-std-features=compiler-builtins-mem
```

The output will be in `target/x86_64-unknown-uefi/debug/libuefi_exfat.a`.

For a release build:

```bash
cargo +nightly build --release --target x86_64-unknown-uefi -Z build-std=core,compiler_builtins,alloc -Z build-std-features=compiler-builtins-mem
```

### Testing

Run the test suite on the host platform:

```bash
cargo test --lib --target x86_64-unknown-linux-gnu
```

## Project Structure

```
uefi-exfat/
├── src/
│   ├── lib.rs         # Main library entry point and UEFI entry function
│   ├── exfat.rs       # exFAT filesystem structures and parsing
│   └── protocol.rs    # UEFI protocol implementation (Volume, File, Directory)
├── Cargo.toml         # Project configuration
├── .cargo/
│   └── config.toml    # Build configuration for UEFI target
└── README.md          # This file
```

## exFAT Structures

The driver implements the following exFAT structures:

- **BootSector**: Main boot sector with filesystem parameters
- **DirectoryEntry**: Generic directory entry (32 bytes)
- **FileEntry**: File directory entry with metadata
- **StreamExtensionEntry**: Stream extension with cluster and size info
- **FileNameEntry**: File name in UTF-16
- **FatEntry**: File Allocation Table entries

## API Example

```rust
// Create a volume from a boot sector
let volume = ExFatVolume::new(boot_sector, block_device)?;

// Get cluster location
let lba = volume.cluster_to_lba(cluster_number);

// Read a cluster
volume.read_cluster(cluster, &mut buffer)?;

// File operations
let mut file = ExFatFile::new(name, attributes, first_cluster, size, volume_handle);
file.read(&mut buffer)?;
file.seek(position)?;
```

## Implementation Status

This is a foundational implementation with the following status:

✅ **Completed:**
- Core exFAT data structures
- Boot sector parsing and validation
- FAT entry parsing
- Basic volume operations
- File and directory handle structures
- Test suite for core functionality

🚧 **In Progress / Future Work:**
- Block device I/O integration
- Full directory traversal
- File read operations with actual data
- UEFI Simple File System Protocol implementation
- Write support
- Long file name handling

## Technical Details

### Memory Management

The driver uses UEFI's allocator through the `uefi::allocator::Allocator` type, which provides heap allocation in UEFI environments.

### No Standard Library

This is a `no_std` crate that works in UEFI environments without the Rust standard library. It only depends on:
- `core`: Rust core library
- `alloc`: Allocation support
- `uefi`: UEFI support crate

### Calling Convention

The entry point uses the EFI calling convention (`extern "efiapi"`):

```rust
pub extern "efiapi" fn efi_main(image_handle: Handle, system_table: *const ()) -> Status
```

## Contributing

Contributions are welcome! Areas for improvement include:
- Block device I/O implementation
- Directory iteration
- File reading with actual cluster chain traversal
- UEFI protocol registration
- Additional tests

## License

This project is dual-licensed under MIT OR Apache-2.0.

## References

- [Microsoft exFAT Specification](https://docs.microsoft.com/en-us/windows/win32/fileio/exfat-specification)
- [UEFI Specification](https://uefi.org/specifications)
- [uefi-rs](https://github.com/rust-osdev/uefi-rs) - Rust UEFI support library

