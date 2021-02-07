//! SMBIOS Library
//!
//! Implements the DMTF [System Management BIOS (SMBIOS) Reference Specification 3.4.0](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf).
//!
//! This library focuses on the tasks involved with reading and interpreting
//! BIOS data.

#![warn(missing_docs)]

mod core;
mod read;
mod structs;
mod unix;
mod windows;

pub use structs::*;

pub use crate::core::*;
pub use read::*;

#[cfg(target_family = "windows")]
pub use windows::get_raw_smbios_data;

pub use windows::WinSMBiosData;

#[cfg(target_family = "unix")]
pub use unix::*;
