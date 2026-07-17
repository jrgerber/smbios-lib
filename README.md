# smbios-lib

A Rust library for reading, parsing, and serializing SMBIOS data from the host system or from a file.

[![crates.io](https://img.shields.io/crates/v/smbios-lib.svg)](https://crates.io/crates/smbios-lib)
[![smbioslib_ci](https://github.com/jrgerber/smbios-lib/actions/workflows/smbios_ci.yml/badge.svg)](https://github.com/jrgerber/smbios-lib/actions/workflows/smbios_ci.yml)

## Table of contents
- [Overview](#overview)
- [Installation](#installation)
- [Features](#features)
- [CLI usage](#cli-usage)
- [Library usage](#library-usage)
- [Security](#security)

## Overview
This crate reads raw SMBIOS data from the current platform and exposes it through a typed API. It also supports loading SMBIOS tables from a file and dumping raw data to disk.

The crate now targets the DMTF SMBIOS 3.9.0 specification and is designed to be usable both as a library and as a small CLI tool. For a higher-level example application built on top of this crate, see [dmidecode-rs](https://github.com/jrgerber/dmidecode-rs).

## Installation
Add the crate to your project:

```toml
[dependencies]
smbios-lib = "0.9.3"
```

Or install it with Cargo:

```bash
cargo add smbios-lib
```

## Features
- Implements all 49 defined structure types from the [DMTF SMBIOS 3.9.0 specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.9.0.pdf) (types 0–46, 126, and 127), with extensibility for OEM types 128–255.
- Cross-platform support for Linux, Intel macOS , Windows, FreeBSD.
- On Linux, reads SMBIOS data from `/sys/firmware/dmi/tables` (sysfs); on FreeBSD falls back to `/dev/mem`.
- Exposes typed structure accessors for BIOS, system, baseboard, chassis, processor, memory, and all other standard records.
- Supports iteration, filtering, and handle-based lookups over SMBIOS entries.
- Provides JSON serialization via `serde`/`serde_json`.
- Includes a CLI binary named `smbiosdump`.

### SMBIOS 3.8 / 3.9 highlights
- **Processor Information (Type 4):** new `socket_type` field; `voltage` marked deprecated from 3.8.0; additional `ProcessorFamily` variants (Intel Core 3/5/7/9, Intel Core Ultra 3/5/7/9, Intel Xeon D, and more).
- **System Chassis Information (Type 3):** new `rack_type` and `rack_height` fields; new `SpecifiedInRackHeight` variant for `ChassisHeight`.
- **Memory Device (Type 17):** updated fields per 3.9.0.
- **Management Controller Host Interface (Type 42):** expanded protocol record data.
- **System Slot (Type 9):** additional slot type values.

## CLI usage
The repository includes a binary that can be used directly from the workspace:

```bash
cargo run --bin smbiosdump -- --help
```

Useful examples:

```bash
# Print the full SMBIOS table
cargo run --bin smbiosdump

# Read from a file instead of the host platform
cargo run --bin smbiosdump -- -f /path/to/smbios.bin

# Dump the raw SMBIOS bytes to a file
cargo run --bin smbiosdump -- -o /tmp/smbios.bin

# Query a single SMBIOS string field
cargo run --bin smbiosdump -- -s system-serial-number

# Output the parsed table as JSON
cargo run --bin smbiosdump -- -j
```

## Library usage
The primary entry points are `table_load_from_device`, `load_smbios_data_from_file`, and the `SMBiosData` iterator API.

```rust
use smbioslib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = table_load_from_device()?;

    if let Some(uuid) = data.find_map(|sys_info: SMBiosSystemInformation| sys_info.uuid()) {
        println!("System UUID: {}", uuid);
    }

    Ok(())
}
```

Common patterns include:
- `find_map` to retrieve a single structure instance.
- `collect::<T>()` to gather all entries of a given structure type.
- `find_by_handle(&handle)` to resolve structures that reference one another.
- `filter(...)` and `find(...)` for targeted searches.

## Security
This library follows a strict security stance: never trust the input.

SMBIOS firmware can be inconsistent across vendors and across versions, so the API is designed to return `Option`-based results and let callers handle missing or malformed data explicitly.
