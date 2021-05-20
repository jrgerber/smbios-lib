# smbios-lib
An SMBIOS Library created in Rust that reads and parses raw BIOS data

[![crates.io](https://img.shields.io/crates/v/smbios-lib.svg)](https://crates.io/crates/smbios-lib)
[![smbioslib_ci](https://github.com/jrgerber/smbios-lib/actions/workflows/smbios_ci.yml/badge.svg)](https://github.com/jrgerber/smbios-lib/actions/workflows/smbios_ci.yml)
![LOC](https://tokei.rs/b1/github/jrgerber/smbios-lib?category=code)

## Table of contents
* [General info](#general-info)
* [Dependencies](#dependencies)
* [Security](#security)
* [Examples](#examples)

## General info
This project reads raw [SMBIOS](https://en.wikipedia.org/wiki/BIOS) data from either a device or file and provides the data as an API.

For an example project using this library take a look at [dmidecode-rs](https://github.com/jrgerber/dmidecode-rs).

### Supports
* [DMTF System Management BIOS (SMBIOS) Reference
Specification 3.4.0](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf)
* Linux
* MacOS
* Windows family

> SMBIOS 3.4.0 contains 47 defined structure types, all of which are covered by this library (types 0-44, 126, and 127).  Support via extensibility exists for types 128-255 (reserved for OEMs).  Extensibility also applies in the case when this library has not been updated for the latest specification version or a pre-released specification and a new type is introduced.

### Project Status
In early development.

The current development stage is to finalize the API design.
	
## Dependencies
* Windows
    * libc = "^0.2"
* MacOS
    * libc = "^0.2"
    * mach = "^0.3"
    * core-foundation = "~0.6"
    * core-foundation-sys = "~0.6"
    * io-kit-sys = "^0.1.0"
	
## Security
This library design follows a strict security mantra: *"Never trust the input"*.

SMBIOS has been around for decades and has undergone many versions and revisions.  Many OEM vendors have interpreted and implemented the specifications over the years. Known cases of incorrect firmware implementations exist.  This presents a veritable labrynth of logic for both the known and the unknown. Rather than creating such a complex state machine, we take advantage of Rust's [Option<>](https://doc.rust-lang.org/std/option/) trait and assert that the act of retrieval for any and all information may fail.  The burden of proof thus shifts from the library to the library consumer who is required to implement the failing condition arm.

## Examples
### Retrieve a Field of a Single Instance Structure
Some structures are required and are a single instance. (e.g. [SMBiosSystemInformation](src/structs/types/system_information.rs))

```rust
#[test]
/// Retrieves the System UUID from a device.
/// UUID is found in the System Information (type 1) structure
fn retrieve_system_uuid() {
    match table_load_from_device() {
        Ok(data) => match data.find_map(|sys_info: SMBiosSystemInformation| sys_info.uuid()) {
            Some(uuid) => println!("System Information UUID == {:?}", uuid),
            None => println!("No System Information (Type 1) structure found with a UUID field"),
        },
        Err(err) => println!("failure: {:?}", err),
    }
}
```

Output:
```
running 1 test
System Information UUID == Uuid(4EE6523F-D56A-F3EA-8E2A-891CF96286EA)
test retrieve_system_uuid ... ok
```

### Retrieve All Instances of a Structure - collect()
Some structures are allowed to have more than one instance. (e.g. [SMBiosMemoryDevice](src/structs/types/memory_device.rs))

```rust
#[test]
/// Prints information for all memory devices within a device.
fn print_all_memory_devices() {
    match table_load_from_device() {
        Ok(data) => {
            for memory_device in data.collect::<SMBiosMemoryDevice>() {
                println!("{:#?}", memory_device);
            }
        }
        Err(err) => println!("failure: {:?}", err),
    }
}
```

Output:
```
running 1 test
smbioslib::structs::types::memory_device::SMBiosMemoryDevice {
    header: smbioslib::core::header::Header {
        struct_type: 17,
        length: 40,
        handle: smbioslib::structs::structure::Handle {
            handle: 8,
        },
    },
    physical_memory_array_handle: Some(
        smbioslib::structs::structure::Handle {
            handle: 1,
        },
    ),
[...elided...]
```

### Retrieve a Structure Given a Handle - find_by_handle()
Some structures point to other structures via handles. (e.g. [SMBiosMemoryDevice](src/structs/types/memory_device.rs) points to [SMBiosPhysicalMemoryArray](src/structs/types/physical_memory_array.rs))

```rust
/// Finds an associated struct by handle
#[test]
fn struct_struct_association() {
    match table_load_from_device() {
        Ok(data) => match data.first::<SMBiosMemoryDevice>() {
            Some(first_memory_device) => {
                let handle = first_memory_device.physical_memory_array_handle().unwrap();
                match data.find_by_handle(&handle) {
                    Some(undefined_struct) => {
                        let physical_memory_array = undefined_struct.defined_struct();
                        println!("{:#?}", physical_memory_array)
                    }
                    None => println!("No Physical Memory Array (Type 16) structure found"),
                }
            }
            None => println!("No Memory Device (Type 17) structure found"),
        },
        Err(err) => println!("failure: {:?}", err),
    }
}
```

Output:
```
running 1 test
PhysicalMemoryArray(
    smbioslib::structs::types::physical_memory_array::SMBiosPhysicalMemoryArray {
        header: smbioslib::core::header::Header {
            struct_type: 16,
            length: 23,
            handle: smbioslib::structs::structure::Handle {
                handle: 1,
            },
        },
        location: Some(
            smbioslib::structs::types::physical_memory_array::MemoryArrayLocationData {
                raw: 3,
                value: SystemBoardOrMotherboard,
            },
        ),
[...elided...]
```
