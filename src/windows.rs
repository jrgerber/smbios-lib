//! Windows SMBIOS.
//!
//! Functions and structures for working with SMBIOS on Windows.
//!
//! # Example
//! ```rust
//! fn windows_dump() {
//!     match smbios::windows::get_raw_smbios_data() {
//!         Ok(raw_data) => {
//!             println!("raw_data: {:?}", raw_data);
//!
//!             for parts in raw_data.smbios_table_data() {
//!                 println!("{:?}", parts.struct_type_name());
//!             }
//!         }
//!         Err(err) => panic!("failure: {:?}", err),
//!     }
//! }
//! ```

use super::structs::SMBiosTableData;
use std::convert::TryInto;
use std::fmt;

mod ffi {
    // https://doc.rust-lang.org/nomicon/ffi.html
    extern crate libc;

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetSystemFirmwareTable(
            firmware_table_provider_signature: u32,
            firmware_table_id: u32,
            firmware_table_buffer_ptr: *mut u8,
            buffer_size: u32,
        ) -> u32;
    }
}

/// Raw SMBIOS data errors
#[derive(Debug)]
pub enum DataError {
    /// [GetSystemFirmwareTable](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable)
    /// threw a Win32 exception.
    Win32Exception,
    /// Out of memory exception.
    MemoryException,
    /// The structure is invalid.
    InvalidStructure,
}

/// Result returned when calling get_raw_smbios_data()
pub type RawSMBiosDataResult = Result<RawSMBiosData, DataError>;

/// Calls the Windows kernel32 function [GetSystemFirmwareTable](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable)
pub fn get_raw_smbios_data() -> RawSMBiosDataResult {
    use std::ptr;

    unsafe {
        const RAW_SMBIOS_SIGNATURE: u32 = 1381190978u32; // 'RSMB' ASCII bytes == 1381190978
        let max_i32: u32 = i32::MAX.try_into().unwrap();
        let firmware_table_buffer_ptr: *mut u8 = ptr::null_mut();

        let buffer_size =
            ffi::GetSystemFirmwareTable(RAW_SMBIOS_SIGNATURE, 0, firmware_table_buffer_ptr, 0);

        // 0 is win32 exception, > i32::MAX is memory exception
        if buffer_size == 0 || buffer_size > max_i32 {
            return Err(DataError::MemoryException);
        }

        let mut firmware_table_buffer = Vec::with_capacity(buffer_size as usize);
        let firmware_table_buffer_ptr = firmware_table_buffer.as_mut_ptr();

        let buffer_size = ffi::GetSystemFirmwareTable(
            RAW_SMBIOS_SIGNATURE,
            0,
            firmware_table_buffer_ptr,
            buffer_size,
        );
        // 0 is win32 exception
        if buffer_size == 0 {
            Err(DataError::Win32Exception)
        }
        // > i32::MAX is memory exception
        else if buffer_size > max_i32 {
            Err(DataError::MemoryException)
        } else {
            firmware_table_buffer.set_len(buffer_size as usize);

            RawSMBiosData::new(firmware_table_buffer)
        }
    }
}

/// # Raw SMBIOS Data
///
/// When Windows kernel32 [GetSystemFirmwareTable](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable) function is called for RSMB,
/// the raw SMBIOS table provider ('RSMB') it retrieves the contents of this
/// raw SMBIOS firmware table structure.
pub struct RawSMBiosData {
    raw_smbios_data: Vec<u8>,
}

impl RawSMBiosData {
    /// Offset of the Used20CallingMethod field (0)
    const USED20_CALLING_METHOD_OFFSET: usize = 0usize;

    /// Offset of the SMBIOSMajorVersion field (1)
    const SMBIOS_MAJOR_VERSION_OFFSET: usize = 1usize;

    /// Offset of the SMBIOSMinorVersion field (2)
    const SMBIOS_MINOR_VERSION_OFFSET: usize = 2usize;

    /// Offset of the DMIRevision field (3)
    const DMI_REVISION_OFFSET: usize = 3usize;

    /// Offset of the Length field (4)
    const TABLE_DATA_LENGTH_OFFSET: usize = 4usize;

    /// Offset of the SMBIOSTableData field (8)
    const SMBIOS_TABLE_DATA_OFFSET: usize = 8usize;

    /// Creates an instance of [RawSMBiosData]
    ///
    /// To retrieve this structure on a windows system call get_raw_smbios_data().
    ///
    /// The new() is provided publicly to allow loading data from other sources
    /// such as a file or from memory array as is done with testing.
    pub fn new(raw_smbios_data: Vec<u8>) -> RawSMBiosDataResult {
        if !RawSMBiosData::is_valid_raw_smbios_data(&raw_smbios_data) {
            Err(DataError::InvalidStructure)
        } else {
            Ok(RawSMBiosData {
                raw_smbios_data: {
                    if !RawSMBiosData::is_valid_raw_smbios_data(&raw_smbios_data) {
                        panic!("Invalid structure")
                    }

                    raw_smbios_data
                },
            })
        }
    }

    /// Verify if a block of data is a valid RawSMBiosData structure
    ///
    /// This only checks if the structure itself is valid and not whether the contained
    /// [SMBiosTableData] structure is valid or not.
    pub fn is_valid_raw_smbios_data(raw_data: &Vec<u8>) -> bool {
        let length = raw_data.len();
        if length <= RawSMBiosData::SMBIOS_TABLE_DATA_OFFSET {
            return false;
        }

        // retrieve the table data length field
        let slice = raw_data
            .get(
                RawSMBiosData::TABLE_DATA_LENGTH_OFFSET
                    ..RawSMBiosData::TABLE_DATA_LENGTH_OFFSET + 4,
            )
            .unwrap();
        let table_data_length = u32::from_le_bytes(
            slice
                .try_into()
                .expect("array length does not match type width"),
        ) as usize;

        table_data_length == length - RawSMBiosData::SMBIOS_TABLE_DATA_OFFSET
    }

    /// The raw SMBIOS data this structure is wrapping
    pub fn raw_smbios_data(&self) -> &[u8] {
        self.raw_smbios_data.as_slice()
    }

    /// Used20CallingMethod
    pub fn used20_calling_method(&self) -> u8 {
        self.raw_smbios_data[RawSMBiosData::USED20_CALLING_METHOD_OFFSET]
    }

    /// SMBIOS major version
    pub fn smbios_major_version(&self) -> u8 {
        self.raw_smbios_data[RawSMBiosData::SMBIOS_MAJOR_VERSION_OFFSET]
    }

    /// SMBIOS minor version
    pub fn smbios_minor_version(&self) -> u8 {
        self.raw_smbios_data[RawSMBiosData::SMBIOS_MINOR_VERSION_OFFSET]
    }

    /// DMI revision
    pub fn dmi_revision(&self) -> u8 {
        self.raw_smbios_data[RawSMBiosData::DMI_REVISION_OFFSET]
    }

    /// Length of the smbios table data
    pub fn table_data_length(&self) -> u32 {
        let slice = self
            .raw_smbios_data
            .get(
                RawSMBiosData::TABLE_DATA_LENGTH_OFFSET
                    ..RawSMBiosData::TABLE_DATA_LENGTH_OFFSET + 4,
            )
            .unwrap();
        u32::from_le_bytes(
            slice
                .try_into()
                .expect("array length does not match type width"),
        )
    }

    /// SMBIOS table data [SMBiosTableData]
    pub fn smbios_table_data(&self) -> SMBiosTableData {
        SMBiosTableData::new(
            self.raw_smbios_data
                .get(RawSMBiosData::SMBIOS_TABLE_DATA_OFFSET..)
                .unwrap(),
        )
    }
}

impl fmt::Debug for RawSMBiosData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<RawSMBiosData>())
            .field("used20_calling_method", &self.used20_calling_method())
            .field("smbios_major_version", &self.smbios_major_version())
            .field("smbios_minor_version", &self.smbios_minor_version())
            .field("dmi_revision", &self.dmi_revision())
            .field("table_data_length", &self.table_data_length())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_raw_smbios_data() {
        // Good structure (lengths are correct)
        let struct_data = vec![0x00u8, 0x03, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0xAB];
        assert!(RawSMBiosData::is_valid_raw_smbios_data(&struct_data));

        // Bad structure (too short)
        let struct_data = vec![0x00u8, 0x03, 0x03];
        assert!(!RawSMBiosData::is_valid_raw_smbios_data(&struct_data));

        // Bad structure (bad table data length)
        let struct_data = vec![0x00u8, 0x03, 0x03, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xAB];
        assert!(!RawSMBiosData::is_valid_raw_smbios_data(&struct_data));
    }
}
