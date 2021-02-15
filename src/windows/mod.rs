//! Windows SMBIOS.
//!
//! Functions and structures for working with SMBIOS on Windows.
//!
//! # Example
//! ```rust
//! #[cfg(target_family = "windows")]
//! #[test]
//! fn windows_dump() {
//!     match load_windows_smbios_data() {
//!         Ok(windows_data) => {
//!             println!("windows_data: {:#?}", windows_data);
//!         }
//!         Err(err) => panic!("failure: {:?}", err),
//!     }
//! }
//! ```

mod win_struct;

#[cfg(target_family = "windows")]
mod platform;

pub use win_struct::*;

#[cfg(target_family = "windows")]
pub use platform::*;

pub use std::convert::TryInto;
pub use std::{fmt, fs, io};
