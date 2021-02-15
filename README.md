# smbios-lib
An SMBIOS Library created in Rust that reads (decodes) raw BIOS data

## Table of contents
* [General info](#general-info)
* [Dependencies](#dependencies)
* [Example](#example)

## General info
This project reads raw SMBIOS data from either a device or file.

### Supports
* [DMTF System Management BIOS (SMBIOS) Reference
Specification 3.4.0](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf)
* Unix family
* Windows family

### Planned Support
* MacOS

### Project Status
In early development.

The current development stage is to finalize the API design.
	
## Dependencies
* libc version: 0.2 (On Windows family only)
	
## Example
Example code that retrieves the System UUID from a device:

```rust
#[test]
fn retrieve_system_uuid() {
    match table_load_from_device() {
        Ok(data) => match data.find_first::<SMBiosSystemInformation>() {
            Some(system_information) => println!(
                "System Information UUID == {:?}",
                system_information.uuid().unwrap()
            ),
            None => println!("No System Information (Type 1) structure found"),
        },
        Err(err) => println!("failure: {:?}", err),
    }
}
```

Output:
```
running 1 test
System Information UUID == Uuid(4EE6523F-D56A-F3EA-8E2A-891CF96286EA)
test windows_retrieve_system_uuid ... ok
````