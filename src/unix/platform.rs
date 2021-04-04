use crate::*;
use std::{io::Error, io::ErrorKind};

    #[cfg(any(target_os = "linux"))]
    /// Full path to smbios_entry_point file on Linux (contains entry point data)
    pub const SYS_ENTRY_FILE: &'static str = "/sys/firmware/dmi/tables/smbios_entry_point";

    #[cfg(any(target_os = "linux"))]
    /// Full path to the DMI file on Linux (contains BIOS table data)
    pub const SYS_TABLE_FILE: &'static str = "/sys/firmware/dmi/tables/DMI";

    /// Full path to the memory device (contains BIOS entry point and table data on *nix platforms)
    pub const DEV_MEM_FILE: &'static str = "/dev/mem";

// Example of Linux structure:
/*
    /sys/firmware/dmi/tables$ sudo hexdump -C smbios_entry_point
    00000000  5f 53 4d 33 5f 7e 18 03  03 00 01 00 83 04 00 00  |_SM3_~..........|
    00000010  00 20 b0 7b 00 00 00 00                           |. .{....|
    00000018

    Note: _SM3_ indicates the version of the entry point.  Offsets 0x7-0x9 are
    the BIOS version 0x03, 0x03, 0x00 (3.0.0)

    jeff@blacktop:/sys/firmware/dmi/tables$ sudo hexdump -C DMI
    00000000  00 1a 00 00 01 02 00 00  03 ff 80 18 19 0c 00 00  |................|
    00000010  00 00 03 0d ff ff ff ff  00 00 4d 69 63 72 6f 73  |..........Micros|
    00000020  6f 66 74 20 43 6f 72 70  6f 72 61 74 69 6f 6e 00  |oft Corporation.|
    00000030  39 2e 31 30 32 2e 31 34  30 00 31 31 2f 31 36 2f  |9.102.140.11/16/|
    00000040  32 30 32 30 00 00 01 1b  01 00 01 02 03 04 86 76  |2020...........v|
    00000050  fb 97 d5 7b 15 d0 b2 39  6b ba a4 df c0 45 02 05  |...{...9k....E..|
    00000060  06 4d 69 63 72 6f 73 6f  66 74 20 43 6f 72 70 6f  |.Microsoft Corpo|
    00000070  72 61 74 69 6f 6e 00 53  75 72 66 61 63 65 20 4c  |ration.Surface L|
    00000080  61 70 74 6f 70 20 33 00  31 32 34 49 3a 30 30 30  |aptop 3.124I:000|
    00000090  33 36 54 3a 30 30 30 4d  3a 30 33 30 30 30 30 30  |36T:000M:0300000|
    000000a0  44 3a 30 42 3a 30 37 46  3a 31 43 3a 30 35 50 3a  |D:0B:07F:1C:05P:|
    000000b0  34 38 53 3a 30 31 45 3a  30 59 3a 31 4b 3a 30 55  |48S:01E:0Y:1K:0U|
    000000c0  3a 30 38 00 30 30 31 39  35 33 33 30 32 30 35 37  |:08.001953302057|
    000000d0  00 53 75 72 66 61 63 65  5f 4c 61 70 74 6f 70 5f  |.Surface_Laptop_|
    000000e0  33 5f 31 38 37 32 00 53  75 72 66 61 63 65 00 00  |3_1872.Surface..|
*/

// Note: /sys/class/dmi/id contains some of the BIOS values, already parsed by the kernel.
// These are useful for cross checking against the results this library produces when reading
// /sys/firmware/dmi/tables/DMI

#[cfg(any(target_os = "linux"))]
/// Loads [SMBiosData] from the device via /sys/firmware/dmi/tables (on Linux)
pub fn table_load_from_device() -> Result<SMBiosData, Error> {
    let version: SMBiosVersion;
    let entry_path = std::path::Path::new(SYS_ENTRY_FILE);

    match SMBiosEntryPoint64::try_load_from_file(entry_path) {
        Ok(entry_point) => {
            version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: entry_point.docrev(),
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::InvalidData => {
                match SMBiosEntryPoint32::try_load_from_file(entry_path) {
                    Ok(entry_point) => {
                        version = SMBiosVersion {
                            major: entry_point.major_version(),
                            minor: entry_point.minor_version(),
                            revision: 0,
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
            _ => return Err(err),
        },
    }

    SMBiosData::try_load_from_file(SYS_TABLE_FILE, Some(version))
}

#[cfg(any(target_os = "freebsd"))]
/// Loads [SMBiosData] from the device via /dev/mem (on FreeBSD)
pub fn table_load_from_device() -> Result<SMBiosData, Error> {
    const RANGE_START: u64 = 0x000F0000u64;
    const RANGE_END: u64 = 0x000FFFFFu64;
    let structure_table_address: u64;
    let structure_table_length: u32;
    let version: SMBiosVersion;

    let mut dev_mem = std::fs::File::open(DEV_MEM_FILE)?;

    match SMBiosEntryPoint32::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END) {
        Ok(entry_point) => {
            structure_table_address = entry_point.structure_table_address() as u64;
            structure_table_length = entry_point.structure_table_length() as u32;

           version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: 0,
            }
        }
        Err(error) => {
            if error.kind() != ErrorKind::UnexpectedEof {
                return Err(error);
            }

            let entry_point =
                SMBiosEntryPoint64::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END)?;

            structure_table_address = entry_point.structure_table_address();
            structure_table_length = entry_point.structure_table_maximum_size();

           version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: entry_point.docrev(),
            }
        }
    }

    if structure_table_address < RANGE_START || structure_table_address > RANGE_END {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "The entry point has given an out of range start address for the table: {}",
                structure_table_address
            ),
        ));
    }

    if structure_table_address + structure_table_length as u64 > RANGE_END {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "The entry point has given a length which exceeds the range: {}",
                structure_table_length
            ),
        ));
    }

    let table = UndefinedStructTable::try_load_from_file_offset(
        &mut dev_mem,
        structure_table_address,
        structure_table_length as usize,
    )?;
    
    Ok(SMBiosData::new(table, Some(version)))
}

#[cfg(any(target_os = "linux"))]
/// Returns smbios raw data via /sys/firmware/dmi/tables (on Linux)
pub fn raw_smbios_from_device() -> Result<Vec<u8>, Error> {
    Ok(std::fs::read(SYS_TABLE_FILE)?)
}

#[cfg(any(target_os = "freebsd"))]
/// Returns smbios raw data via /sys/firmware/dmi/tables (on Linux)
pub fn raw_smbios_from_device() -> Result<Vec<u8>, Error> {
    use std::io::{prelude::*, SeekFrom};
    const RANGE_START: u64 = 0x000F0000u64;
    const RANGE_END: u64 = 0x000FFFFFu64;
    let structure_table_address: u64;
    let structure_table_length: usize;

    let mut dev_mem = std::fs::File::open(DEV_MEM_FILE)?;

    match SMBiosEntryPoint32::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END) {
        Ok(entry_point) => {
            structure_table_address = entry_point.structure_table_address() as u64;
            structure_table_length = entry_point.structure_table_length() as usize;
        }
        Err(error) => {
            if error.kind() != ErrorKind::UnexpectedEof {
                return Err(error);
            }

            let entry_point =
                SMBiosEntryPoint64::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END)?;

            structure_table_address = entry_point.structure_table_address();
            structure_table_length = entry_point.structure_table_maximum_size() as usize;
        }
    }

    if structure_table_address < RANGE_START || structure_table_address > RANGE_END {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "The entry point has given an out of range start address for the table: {}",
                structure_table_address
            ),
        ));
    }

    if structure_table_address + structure_table_length as u64 > RANGE_END {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "The entry point has given a length which exceeds the range: {}",
                structure_table_length
            ),
        ));
    }

    if structure_table_length < Header::SIZE + 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("The table has an invalid size: {}", structure_table_length),
        ));
    }

    dev_mem.seek(SeekFrom::Start(structure_table_address))?;
    let mut table = Vec::with_capacity(structure_table_length);
    table.resize(structure_table_length, 0);
    dev_mem.read_exact(&mut table)?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_dev_mem_scan() -> io::Result<()> {
        const RANGE_START: u64 = 0x000F0000u64;
        const RANGE_END: u64 = 0x000FFFFFu64;
        let mut dev_mem = File::open(DEV_MEM_FILE)?;
        let structure_table_address: u64;
        let structure_table_length: u32;

        match SMBiosEntryPoint32::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END) {
            Ok(entry_point) => {
                structure_table_address = entry_point.structure_table_address() as u64;
                structure_table_length = entry_point.structure_table_length() as u32;
                println!(
                    "SMBIOS {}.{} present.",
                    entry_point.major_version(),
                    entry_point.minor_version()
                );
                println!(
                    "{} structures occupying {} bytes.",
                    entry_point.number_of_smbios_structures(),
                    entry_point.structure_table_length()
                );
                println!("Table at: {:#010X}.", entry_point.structure_table_address());
            }
            Err(error) => {
                if error.kind() != ErrorKind::UnexpectedEof {
                    return Err(error);
                }

                let entry_point =
                    SMBiosEntryPoint64::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END)?;

                structure_table_address = entry_point.structure_table_address();
                structure_table_length = entry_point.structure_table_maximum_size();

                println!(
                    "SMBIOS {}.{}.{} present.",
                    entry_point.major_version(),
                    entry_point.minor_version(),
                    entry_point.docrev()
                );
                println!(
                    "Occupying {} bytes maximum.",
                    entry_point.structure_table_maximum_size()
                );
                println!("Table at: {:#010X}.", entry_point.structure_table_address());
            }
        }

        if structure_table_address < RANGE_START || structure_table_address > RANGE_END {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "The entry point has given an out of range start address for the table: {}",
                    structure_table_address
                ),
            ));
        }

        if structure_table_address + structure_table_length as u64 > RANGE_END {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "The entry point has given a length which exceeds the range: {}",
                    structure_table_length
                ),
            ));
        }

        let table = UndefinedStructTable::try_load_from_file_offset(
            &mut dev_mem,
            structure_table_address,
            structure_table_length as usize,
        )?;
        println!("{:#X?}", table);

        Ok(())
    }
}
