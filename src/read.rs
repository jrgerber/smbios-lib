//! Loads files containing raw Windows bios binary data.
//!
//! When testing this library it is useful to read stored
//! raw data and then load it into the structures.

use crate::windows;
use std::fs;
use std::io;

/// Raw SMBIOS data file load errors
#[derive(Debug)]
pub enum FileLoadError {
    /// Error while loading data
    IOError(io::Error),
    /// Data loaded but was invalid
    DataError(windows::DataError),
}

/// Result returned when loading raw SMBIOS data from a file
pub type FileLoadResult = Result<windows::RawSMBiosData, FileLoadError>;

/// Loads raw smbios data from a file
pub fn load_smbios_data_file(filename: &str) -> FileLoadResult {
    match fs::read(filename) {
        Ok(data) => match windows::RawSMBiosData::new(data) {
            Ok(data) => Ok(data),
            Err(err) => Err(FileLoadError::DataError(err)),
        },
        Err(err) => Err(FileLoadError::IOError(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_smbios_data_file() {
        let filename = r".\tests\jeffgerlap_3_2_0.dat";

        match load_smbios_data_file(&filename) {
            Ok(raw_data) => {
                println!("raw_data: {:?}", raw_data);
                for parts in raw_data.smbios_table_data() {
                    println!("{:?}", parts.struct_type_name());
                }
            }
            _ => panic!("Expected data!"),
        }
    }
}
