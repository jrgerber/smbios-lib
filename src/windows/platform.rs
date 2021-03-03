use std::{
    convert::TryInto,
    io::{Error, ErrorKind},
};

use crate::SMBiosData;

use super::WinSMBiosData;

mod ffi {
    // https://doc.rust-lang.org/nomicon/ffi.html
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

/// Calls the Windows kernel32 function [GetSystemFirmwareTable](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable)
pub fn load_windows_smbios_data() -> Result<WinSMBiosData, Error> {
    match raw_smbios_from_device() {
        Ok(raw) => WinSMBiosData::new(raw),
        Err(e) => Err(e)
    }
}

/// Loads SMBIOS table data ([SMBiosData]) from the device
pub fn table_load_from_device() -> Result<SMBiosData, Error> {
    Ok(load_windows_smbios_data()?.smbios_data)
}

/// Returns smbios raw data
pub fn raw_smbios_from_device() -> Result<Vec<u8>, Error> {
    use std::ptr;

    unsafe {
        const MEMORY_ERROR_MESSAGE: &'static str = "Memory error";
        const RAW_SMBIOS_SIGNATURE: u32 = 1381190978u32; // 'RSMB' ASCII bytes == 1381190978
        let max_i32: u32 = i32::MAX.try_into().unwrap();
        let firmware_table_buffer_ptr: *mut u8 = ptr::null_mut();

        let buffer_size =
            ffi::GetSystemFirmwareTable(RAW_SMBIOS_SIGNATURE, 0, firmware_table_buffer_ptr, 0);

        // 0 is win32 exception, > i32::MAX is memory exception
        if buffer_size == 0 || buffer_size > max_i32 {
            return Err(Error::new(ErrorKind::Other, MEMORY_ERROR_MESSAGE));
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
            Err(Error::last_os_error())
        }
        // > i32::MAX is memory exception
        else if buffer_size > max_i32 {
            Err(Error::new(ErrorKind::Other, MEMORY_ERROR_MESSAGE))
        } else {
            firmware_table_buffer.set_len(buffer_size as usize);
            Ok(firmware_table_buffer)
        }
    }
}