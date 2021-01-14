use super::fields::{get_field_byte, get_field_dword};
use super::structs::SMBiosTableData;
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

pub fn get_raw_smbios_data() -> Option<RawSMBiosData> {
    use std::convert::TryInto;
    use std::ptr;

    unsafe {
        const RAW_SMBIOS_SIGNATURE: u32 = 1381190978u32; // 'RSMB' ASCII bytes == 1381190978
        let max_i32: u32 = i32::MAX.try_into().unwrap();
        let firmware_table_buffer_ptr: *mut u8 = ptr::null_mut();

        let buffer_size =
            ffi::GetSystemFirmwareTable(RAW_SMBIOS_SIGNATURE, 0, firmware_table_buffer_ptr, 0);

        // 0 is win32 exception, > i32::MAX is memory exception
        if buffer_size == 0 || buffer_size > max_i32 {
            return None;
        }

        let mut firmware_table_buffer = Vec::with_capacity(buffer_size as usize);
        let firmware_table_buffer_ptr = firmware_table_buffer.as_mut_ptr();

        let buffer_size = ffi::GetSystemFirmwareTable(
            RAW_SMBIOS_SIGNATURE,
            0,
            firmware_table_buffer_ptr,
            buffer_size,
        );
        // 0 is win32 exception, > i32::MAX is memory exception
        if buffer_size == 0 || buffer_size > max_i32 {
            None
        } else {
            firmware_table_buffer.set_len(buffer_size as usize);
            Some(RawSMBiosData::new(firmware_table_buffer))
        }
    }
}

pub struct RawSMBiosData {
    raw_smbios_data: Vec<u8>,
}

impl RawSMBiosData {
    pub fn new(raw_smbios_data: Vec<u8>) -> Self {
        RawSMBiosData { raw_smbios_data }
    }

    pub fn raw_smbios_data(&self) -> &[u8] {
        self.raw_smbios_data.as_slice()
    }

    pub fn used20_calling_method(&self) -> Option<u8> {
        get_field_byte(0, self.raw_smbios_data())
    }

    pub fn smbios_major_version(&self) -> Option<u8> {
        get_field_byte(1, self.raw_smbios_data())
    }

    pub fn smbios_minor_version(&self) -> Option<u8> {
        get_field_byte(2, self.raw_smbios_data())
    }

    pub fn dmi_revision(&self) -> Option<u8> {
        get_field_byte(3, self.raw_smbios_data())
    }

    pub fn length(&self) -> Option<u32> {
        get_field_dword(4, self.raw_smbios_data())
    }

    // https://depth-first.com/articles/2020/06/22/returning-rust-iterators/
    pub fn smbios_table_data(&self) -> Option<SMBiosTableData> {
        match self.raw_smbios_data.get(8..) {
            Some(val) => Some(SMBiosTableData::new(val)),
            None => None,
        }
    }
}

impl fmt::Debug for RawSMBiosData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<RawSMBiosData>())
            .field("used20_calling_method", &self.used20_calling_method())
            .field("smbios_major_version", &self.smbios_major_version())
            .field("smbios_minor_version", &self.smbios_minor_version())
            .field("dmi_revision", &self.dmi_revision())
            .field("length", &self.length())
            .finish()
    }
}
