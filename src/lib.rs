//! SMBIOS Library
//!
//! Implements the DMTF [System Management BIOS (SMBIOS) Reference Specification 3.5.0](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.5.0.pdf).
//!
//! This library focuses on the tasks involved with reading and interpreting
//! BIOS data.

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]
#![cfg_attr(feature = "no_std", no_std)]

extern crate alloc;

mod core;
#[cfg(not(feature = "no_std"))]
mod file_io;
#[cfg(not(feature = "no_std"))]
mod macos;
mod structs;
#[cfg(not(feature = "no_std"))]
mod unix;
#[cfg(not(feature = "no_std"))]
mod windows;
#[cfg(all(feature = "x86_64", feature = "no_std"))]
mod x86_64;

pub use structs::*;

pub use crate::core::*;
#[cfg(not(feature = "no_std"))]
pub use file_io::*;

#[cfg(all(target_family = "windows", not(feature = "no_std")))]
pub use windows::{load_windows_smbios_data, raw_smbios_from_device, table_load_from_device};

#[cfg(not(feature = "no_std"))]
pub use windows::WinSMBiosData;

#[cfg(all(any(target_os = "linux", target_os = "android", target_os = "freebsd"), not(feature = "no_std")))]
pub use unix::*;

#[cfg(all(any(target_os = "macos", target_os = "ios"), not(feature = "no_std")))]
pub use macos::*;

#[cfg(all(feature = "x86_64", feature = "no_std"))]
pub use crate::x86_64::*;