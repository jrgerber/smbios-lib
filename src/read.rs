//! Loads files containing raw Windows bios binary data.
//!
//! When testing this library it is useful to read stored
//! raw data and then load it into the structures.

use crate::structs::SMBiosTableData;
use crate::windows;
use std::fs;
use std::io;

/// Loads raw smbios data from a file and returns [SMBiosTableData] or [io:Error] on error.
///
/// Currently supports reading raw files containing only SMBIOS table data or
/// Windows raw files containing the windows header and SMBIOS table data.
pub fn load_smbios_table_data(filename: &str) -> Result<SMBiosTableData, io::Error> {
    let data = fs::read(filename)?;
    if windows::WinSMBiosData::is_valid_win_smbios_data(&data) {
        let win_smbios = windows::WinSMBiosData::new(data)
            .expect("Structure shouldn't be invalid it was already checked.");
        Ok(win_smbios.smbios_table_data)
    } else {
        Ok(SMBiosTableData::new(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_smbios_table_data() {
        let filename = r".\tests\jeffgerlap_3_2_0.dat";

        match load_smbios_table_data(&filename) {
            Ok(table_data) => {
                for parts in table_data.into_iter() {
                    println!("{:?}", parts.struct_type_name());
                }
            }
            _ => panic!("Expected data!"),
        }
    }
}
