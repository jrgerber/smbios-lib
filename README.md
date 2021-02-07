# smbios-lib
An SMBIOS Library created in Rust that reads (decodes) raw BIOS data

## Table of contents
* [General info](#general-info)
* [Technologies](#technologies)
* [Example](#example)

## General info
This project reads raw SMBIOS data from either a device or file.

### Supports
* [DMTF System Management BIOS (SMBIOS) Reference
Specification 3.4.0](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf)
* Operating Systems: Linux and Windows

### Planned Support
* MacOS

### Project Status
In early development and open for public review and comments.

The current development stage goal is to define a final API design.
	
## Technologies
Project dependencies:
* libc version: 0.2 (On Windows family only)
	
## Example
Example code for retrieving the System UUID on a Windows device:

```
TODO
```