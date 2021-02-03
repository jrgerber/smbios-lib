use std::convert::TryInto;

use super::WinSMBiosData;

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
pub type WinSMBiosDataResult = Result<WinSMBiosData, DataError>;

/// Calls the Windows kernel32 function [GetSystemFirmwareTable](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable)
#[cfg(target_family = "windows")]
pub fn get_raw_smbios_data() -> WinSMBiosDataResult {
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

            WinSMBiosData::new(firmware_table_buffer)
        }
    }
}
