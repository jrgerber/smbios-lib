use crate::*;
use core_foundation::{
    base::{kCFAllocatorDefault, mach_port_t, CFRelease, TCFTypeRef},
    data::{CFDataGetBytePtr, CFDataGetLength, CFDataRef},
};
use io_kit_sys::{
    types::{io_service_t, IOOptionBits},
    IOMasterPort, IOObjectRelease, IORegistryEntryCreateCFProperty, IOServiceGetMatchingService,
    IOServiceMatching, CFSTR,
};
use mach::*;
use std::{convert::TryFrom, ffi::CString, io::Error, io::ErrorKind};

struct AppleSMBiosService {
    pub service_handle: io_service_t,
}

impl AppleSMBiosService {
    fn try_init() -> Result<Self, Error> {
        unsafe {
            let service_name = CString::new("AppleSMBIOS").expect("CString::new failed");
            let mut master_port: mach_port_t = port::MACH_PORT_NULL;

            IOMasterPort(port::MACH_PORT_NULL, &mut master_port);

            let service: io_service_t =
                IOServiceGetMatchingService(master_port, IOServiceMatching(service_name.as_ptr()));

            if service == port::MACH_PORT_NULL {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "AppleSMBIOS service is unreachable",
                ));
            }

            Ok(AppleSMBiosService {
                service_handle: service,
            })
        }
    }
}

impl Drop for AppleSMBiosService {
    fn drop(&mut self) {
        unsafe {
            IOObjectRelease(self.service_handle);
        }
    }
}

fn try_load_macos_entry_point() -> Result<SMBiosEntryPoint32, Error> {
    let service = AppleSMBiosService::try_init()?;

    unsafe {
        let smbios_entry_point_name = CString::new("SMBIOS-EPS").expect("CString::new failed");

        let option_bits: IOOptionBits = 0;
        let data_ref = IORegistryEntryCreateCFProperty(
            service.service_handle,
            CFSTR(smbios_entry_point_name.as_ptr()),
            kCFAllocatorDefault,
            option_bits,
        ) as CFDataRef;

        if data_ref.is_null() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "SMBIOS-EPS entry point is unreachable",
            ));
        }

        if !data_ref.is_null() {
            CFRelease(data_ref.as_void_ptr());
        }

        let data_ptr = CFDataGetBytePtr(data_ref);
        let data_length = CFDataGetLength(data_ref);
        let mut entry_point = Vec::with_capacity(data_length as usize);

        std::ptr::copy(data_ptr, entry_point.as_mut_ptr(), data_length as usize);
        entry_point.set_len(data_length as usize);

        SMBiosEntryPoint32::try_from(entry_point)
    }
}

fn try_load_macos_table() -> Result<Vec<u8>, Error> {
    let service = AppleSMBiosService::try_init()?;

    unsafe {
        let smbios_table_name = CString::new("SMBIOS").expect("CString::new failed");

        let option_bits: IOOptionBits = 0;
        let data_ref = IORegistryEntryCreateCFProperty(
            service.service_handle,
            CFSTR(smbios_table_name.as_ptr()),
            kCFAllocatorDefault,
            option_bits,
        ) as CFDataRef;

        if data_ref.is_null() {
            return Err(Error::new(ErrorKind::NotFound, "SMBIOS is unreachable"));
        }

        if !data_ref.is_null() {
            CFRelease(data_ref.as_void_ptr());
        }

        let len = CFDataGetLength(data_ref);

        let mut table: Vec<u8> = Vec::with_capacity(len as usize);
        let table_ptr = table.as_mut_ptr();
        std::ptr::copy(table_ptr, table.as_mut_ptr(), len as usize);
        table.set_len(len as usize);

        Ok(table)
    }
}

/// Loads SMBIOS table data ([SMBiosData]) from the device
pub fn table_load_from_device() -> Result<SMBiosData, Error> {
    let entry_point = try_load_macos_entry_point()?;

    let version = SMBiosVersion {
        major: entry_point.major_version(),
        minor: entry_point.minor_version(),
        revision: 0,
    };

    let table = try_load_macos_table()?;
    dump_table(&table);
    Ok(SMBiosData::from_vec_and_version(table, Some(version)))
}

fn dump_table(data: &Vec<u8>) {
    use std::fs::File;
    use std::io::{BufWriter, Write};

    let f = File::create("macos.dat").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(&data).expect("Unable to write data");
}
