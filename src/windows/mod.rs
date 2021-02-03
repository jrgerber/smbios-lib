//! Windows SMBIOS.
//!
//! Functions and structures for working with SMBIOS on Windows.
//!
//! # Example
//! ```rust
//! #[cfg(target_family = "windows")]
//! fn windows_dump() {
//!     match smbios::get_raw_smbios_data() {
//!         Ok(raw_data) => {
//!             println!("raw_data: {:?}", raw_data);
//!
//!             for parts in &raw_data.smbios_table_data {
//!                 println!("{:?}", parts.struct_type_name());
//!             }
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
