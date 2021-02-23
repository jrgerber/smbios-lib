use crate::*;
use core_foundation::base::{kCFAllocatorDefault, mach_port_t, TCFTypeRef};
use core_foundation::{
    base::{CFRelease},
    data::{CFDataGetLength, CFDataGetBytePtr, CFDataRef},
};
use io_kit_sys::{
    types::{io_service_t, IOOptionBits},
    IOMasterPort, IOObjectRelease, IORegistryEntryCreateCFProperty, IOServiceGetMatchingService,
    IOServiceMatching, CFSTR,
};
use mach::*;
use std::{convert::TryFrom, ffi::CString, io::Error, io::ErrorKind};
//use IOOptionBits::kNilOptions;

/*
#if defined(__APPLE__)
    mach_port_t masterPort;
    io_service_t service = MACH_PORT_NULL;
    CFDataRef dataRef;

    if (!(opt.flags & FLAG_QUIET))
        printf("Getting SMBIOS data from Apple SMBIOS service.\n");

    IOMasterPort(MACH_PORT_NULL, &masterPort);
    service = IOServiceGetMatchingService(masterPort,
        IOServiceMatching("AppleSMBIOS"));
    if (service == MACH_PORT_NULL)
    {
        fprintf(stderr, "AppleSMBIOS service is unreachable, sorry.");
        ret = 1;
        goto exit_free;
    }

    dataRef = (CFDataRef) IORegistryEntryCreateCFProperty(service,
        CFSTR("SMBIOS-EPS"), kCFAllocatorDefault, kNilOptions);

    if (dataRef == NULL)
    {
        fprintf(stderr, "SMBIOS entry point is unreachable, sorry.\n");
        ret = 1;
        goto exit_free;
    }

    if((buf = malloc(0x20)) == NULL)
    {
        perror("malloc");
        ret = 1;
        goto exit_free;
    }

    CFDataGetBytes(dataRef, CFRangeMake(0, 0x20), (UInt8*)buf);

    if (NULL != dataRef)
        CFRelease(dataRef);
    IOObjectRelease(service);

    if (smbios_decode(buf, NULL, FLAG_FROM_API))
    {
        found++;
        goto done;
    }

*/

/*
        mach_port_t masterPort;
        CFMutableDictionaryRef properties = NULL;
        io_service_t service = MACH_PORT_NULL;
        CFDataRef dataRef;

        IOMasterPort(MACH_PORT_NULL, &masterPort);
        service = IOServiceGetMatchingService(masterPort,
            IOServiceMatching("AppleSMBIOS"));
        if (service == MACH_PORT_NULL)
        {
            fprintf(stderr, "AppleSMBIOS service is unreachable, sorry.\n");
            return;
        }

        if (kIOReturnSuccess != IORegistryEntryCreateCFProperties(service,
            &properties, kCFAllocatorDefault, kNilOptions))
        {
            fprintf(stderr, "No data in AppleSMBIOS IOService, sorry.\n");
            return;
        }

        if (!CFDictionaryGetValueIfPresent(properties, CFSTR( "SMBIOS"),
            (const void **)&dataRef))
        {
            fprintf(stderr, "SMBIOS property data is unreachable, sorry.\n");
            return;
        }

        len = CFDataGetLength(dataRef);
        if((buf = malloc(sizeof(u8) * len)) == NULL)
        {
            perror("malloc");
            return;
        }

        CFDataGetBytes(dataRef, CFRangeMake(0, len), (UInt8*)buf);

        if (NULL != dataRef)
            CFRelease(dataRef);

        /*
         * This CFRelease throws 'Segmentation fault: 11' since macOS 10.12, if
         * the compiled binary is not signed with an Apple developer profile.
         */
        if (NULL != properties)
            CFRelease(properties);

        IOObjectRelease(service);
*/

/// Loads SMBIOS table data ([SMBiosData]) from the device
pub fn table_load_from_device() -> Result<SMBiosData, Error> {
    let version: SMBiosVersion;

    unsafe {
        let service_name = CString::new("AppleSMBIOS").expect("CString::new failed");
        let smbios_entry_point_name = CString::new("SMBIOS-EPS").expect("CString::new failed");
        let mut master_port: mach_port_t = port::MACH_PORT_NULL;

        // 1. Get the entry point
        IOMasterPort(port::MACH_PORT_NULL, &mut master_port);

        let service: io_service_t =
            IOServiceGetMatchingService(master_port, IOServiceMatching(service_name.as_ptr()));

        if service == port::MACH_PORT_NULL {
            return Err(Error::new(
                ErrorKind::NotFound,
                "AppleSMBIOS service is unreachable",
            ));
        }

        let option_bits: IOOptionBits = 0;
        let data_ref = IORegistryEntryCreateCFProperty(
            service,
            CFSTR(smbios_entry_point_name.as_ptr()),
            kCFAllocatorDefault,
            option_bits,
        ) as CFDataRef;

        if data_ref.is_null() {
            IOObjectRelease(service);

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

        match SMBiosEntryPoint32::try_from(entry_point) {
            Ok(entry_point) => {
                version = SMBiosVersion {
                    major: entry_point.major_version(),
                    minor: entry_point.minor_version(),
                    revision: 0,
                }
            }
            Err(err) => return Err(err),
        }

        // 2. Get the table
    }

    let data = vec![]; // TODO: get the table
    Ok(SMBiosData::from_vec_and_version(data, Some(version)))
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
