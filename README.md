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
In early development and open for public review and comments.

The current development stage goal is to define a final API design.
	
## Dependencies
* libc version: 0.2 (On Windows family only)
	
## Example
Example code that retrieves the System UUID on a Windows family device:

```rust
#[cfg(target_family = "windows")]
#[test]
fn windows_retrieve_system_uuid() {
    // Load table data from the Windows device
    match get_raw_smbios_data() {
        Ok(raw_data) => {
            let mut iterator = raw_data.smbios_table_data.into_iter();

            // Search the table data for the first System Information (Type 1) structure
            match iterator.find(|current_struct| {
                current_struct.header.struct_type() == SMBiosSystemInformation::STRUCT_TYPE
            }) {
                Some(base_struct) =>
                // Down cast the structure to an SMBIOSSystemInformation structure
                {
                    match base_struct.struct_type_name() {
                        DefinedStruct::SystemInformation(system_information) => {
                            println!(
                                "System Information UUID == {:?}",
                                system_information.uuid().unwrap()
                            )
                        }
                        _ => panic!("Downcasting library design failure"),
                    }
                }
                None => println!("No System Information (Type 1) structure found"),
            }
        }
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

> Note: The library design for this common user scenario is overly complex and requires refactoring.