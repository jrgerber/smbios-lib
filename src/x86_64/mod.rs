//! X86_64 no-std SMBIOS.
//!
//! Functions and structures for working with SMBIOS on no-std x86_64 environments.
//!
//! # Example
//! ```rust
//! use x86_64::{PhysAddr, VirtAddr};
//! use smbioslib::{MemoryMapper, table_load_from_device};
//! struct MemoryMapperImpl {}
//!
//! impl MemoryMapper for MemoryMapperImpl {
//!     fn map_block(&mut self, addr: PhysAddr, size: usize) -> VirtAddr {
//!         // map physical block to virtual space
//!         VirtAddr::new(addr.as_u64())
//!     }
//! }
//!
//! fn smbios_dump() {
//!     let mut mapper = MemoryMapperImpl {};
//!     match table_load_from_device(&mut mapper) {
//!         Ok(data) => {
//!             println!("smbios_data: {:#?}", data);
//!         }
//!         Err(err) => panic!("failure: {:?}", err),
//!     }
//! }
//! ```
mod platform;

pub use platform::*;