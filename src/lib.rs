//! SMBIOS Library
//!
//! Implements the DMTF [System Management BIOS (SMBIOS) Reference Specification 3.4.0](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf).
//!
//! This library focuses on the tasks involved with reading and interpreting
//! BIOS data.

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]

mod core;
mod file_io;
mod macos;
mod structs;
mod unix;
mod windows;

pub use structs::*;

pub use crate::core::*;
pub use file_io::*;

#[cfg(target_family = "windows")]
pub use windows::{load_windows_smbios_data, raw_smbios_from_device, table_load_from_device};

pub use windows::WinSMBiosData;

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
pub use unix::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use macos::*;
