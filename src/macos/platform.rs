use crate::*;

use core_foundation::{
    base::CFRelease,
    data::{CFDataGetBytePtr, CFDataGetLength, CFDataRef},
    string::CFStringRef,
};
use core_foundation::{
    base::{kCFAllocatorDefault, mach_port_t, TCFTypeRef},
    dictionary::CFDictionaryGetValueIfPresent,
};
use core_foundation_sys::base::{CFAllocatorRef, CFTypeRef};
use core_foundation_sys::dictionary::{CFDictionaryRef, CFMutableDictionaryRef};

use mach::kern_return::kern_return_t;

use std::os::raw::{c_char, c_int, c_uint, c_void};

/*
use io_kit_sys::{
    ret::kIOReturnSuccess,
    types::{io_service_t, IOOptionBits},
    IOMasterPort, IOObjectRelease, IORegistryEntryCreateCFProperties,
    IORegistryEntryCreateCFProperty, IOServiceGetMatchingService, IOServiceMatching, CFSTR,
};
*/
use mach::*;
use std::{convert::TryFrom, ffi::CString, io::Error, io::ErrorKind, ptr::null_mut};
///*
pub type io_registry_entry_t = io_object_t;
pub type io_service_t = io_object_t;
pub type io_object_t = mach_port_t;
pub type IOOptionBits = c_uint;
pub const KERN_SUCCESS: kern_return_t = 0;
// IOReturn
pub type IOReturn = kern_return_t;
// OK
pub const kIOReturnSuccess: IOReturn = KERN_SUCCESS as c_int;

extern "C" {
    pub static kIOMasterPortDefault: mach_port_t;

    fn __CFStringMakeConstantString(cStr: *const c_char) -> CFStringRef;

    pub fn IOMasterPort(bootstrapPort: mach_port_t, masterPort: *mut mach_port_t) -> kern_return_t;

    pub fn IORegistryEntryCreateCFProperties(
        entry: io_registry_entry_t,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> kern_return_t;

    pub fn IORegistryEntryCreateCFProperty(
        entry: io_registry_entry_t,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;

    pub fn IOServiceGetMatchingService(
        masterPort: mach_port_t,
        matching: CFDictionaryRef,
    ) -> io_service_t;

    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;

    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;
}

pub fn CFSTR(cStr: *const c_char) -> CFStringRef {
    unsafe { __CFStringMakeConstantString(cStr) }
}
//*/
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
                "SMBIOS entry point is unreachable",
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
        let mut properties: CFMutableDictionaryRef = null_mut();
        let properties_ptr: *mut CFMutableDictionaryRef = &mut properties;

        println!("here");

        if kIOReturnSuccess
            != IORegistryEntryCreateCFProperties(
                service.service_handle,
                properties_ptr,
                kCFAllocatorDefault,
                option_bits,
            )
        {
            return Err(Error::new(
                ErrorKind::NotFound,
                "No data in AppleSMBIOS IOService",
            ));
        }

        println!("here2");

        let mut data_ref: CFDataRef = null_mut();
        let data_ref_ptr: *mut CFDataRef = &mut data_ref;

        if CFDictionaryGetValueIfPresent(
            properties,
            smbios_table_name.as_ptr() as *const c_void,
            data_ref_ptr as *mut *const c_void,
        ) != 0
        {
            return Err(Error::new(
                ErrorKind::NotFound,
                "SMBIOS property data is unreachable",
            ));
        }

        println!("here3");

        let data_ptr = CFDataGetBytePtr(data_ref);
        let data_length = CFDataGetLength(data_ref);
        let mut table = Vec::with_capacity(data_length as usize);

        std::ptr::copy(data_ptr, table.as_mut_ptr(), data_length as usize);
        table.set_len(data_length as usize);

        if !data_ref.is_null() {
            CFRelease(data_ref.as_void_ptr());
        }

        if !properties.is_null() {
            CFRelease(properties.as_void_ptr());
        }

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

    Ok(SMBiosData::from_vec_and_version(table, Some(version)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_load_unit_test() {
        match table_load_from_device() {
            Ok(table) => {
                println!("table_data: {:?}", table);

                for smbios_structure in table.into_iter() {
                    println!("{:#?}", smbios_structure.defined_struct());
                }
            }
            Err(err) => panic!("failure: {:?}", err),
        }
    }
}
