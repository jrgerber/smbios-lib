//! Loads files containing raw bios binary data.
//!
//! When testing this library it is useful to read stored
//! raw data and then load it into the structures.
use windows::WinSMBiosData;

use crate::*;
use std::fs::{read, read_dir};
use std::io::Error;

/// Loads raw smbios data from a file and returns [SMBiosTableData] or [std::io::Error] on error.
///
/// Currently supports reading raw files containing only SMBIOS table data or
/// Windows raw files containing the windows header and SMBIOS table data.
pub fn load_smbios_data_from_file(filename: &str) -> Result<SMBiosTableData, Error> {
    let data = read(filename)?;
    if WinSMBiosData::is_valid_win_smbios_data(&data) {
        let win_smbios = WinSMBiosData::new(data)
            .expect("Structure shouldn't be invalid it was already checked.");
        Ok(win_smbios.smbios_table_data)
    } else {
        Ok(SMBiosTableData::new(data))
    }
}

/// Loads raw smbios data files from a given _folder_ and returns [Vec<SMBiosTableData>]
pub fn load_raw_files(folder: &str) -> Vec<SMBiosTableData> {
    let mut result = Vec::new();

    let entries = read_dir(folder)
        .expect("valid files")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()
        .expect("msg");

    for elem in entries {
        let file_name = elem.to_str().expect("valid filename characters");

        // Temporary output to see what files we found
        // println!("{}", file_name);

        let smbios_table_data = load_smbios_data_from_file(file_name);
        match smbios_table_data {
            Ok(data) => result.push(data),
            Err(_) => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_smbios_table_data() {
        let mut path = PathBuf::new();
        path.push(".");
        path.push("tests");
        path.push("jeffgerlap_3_2_0");
        path.set_extension("dat");

        // TODO: This path has doesn't work on linux, why?
        let filename = path.display().to_string();

        match load_smbios_data_from_file(&filename) {
            Ok(table_data) => {
                for parts in table_data.into_iter() {
                    println!("{:?}", parts.struct_type_name());
                }
            }
            _ => panic!("Expected data!"),
        }
    }
}
