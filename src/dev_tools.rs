//! Tools to speed library development.
//!
//! Place any functions here which are useful while developing the library.

use crate::read::*;
use crate::windows::*;
use std::fs;
use std::io;

// TODO:
// 1. Write a function to iterate through a folder full of windows DAT files
// 2. Write a function to iterate through a folder full of non-windows DAT files
// 3. Write a test function to output rust code for unit tests of each type by
//    finding candidates from a file collection.

fn load_windows_raw_files(folder: &str) -> Vec<RawSMBiosData> {
    let mut result = Vec::new();

    let entries = fs::read_dir(folder)
        .expect("valid files")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("msg");

    for elem in entries {
        let file_name = elem.to_str().expect("valid filename characters");

        // Temporary output to see what files we found
        println!("{}", file_name);

        let raw_smbios_data = load_smbios_data_file(file_name);
        match raw_smbios_data {
            Ok(data) => result.push(data),
            Err(_) => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_type() {
        let type_to_find = 2u8;
        let results = load_windows_raw_files(r"C:\Users\Jeff\Desktop\BIOSRawFiles\WindowsHeader");
        for raw_data in results {
            let found_structure = raw_data
                .smbios_table_data()
                .into_iter()
                .find(|x| x.header.struct_type() == type_to_find);

            if found_structure.is_some() {
                let structure = found_structure.unwrap();
                println!("{:?}", structure.struct_type_name());
                println!();
                println!("{:?}", structure);
                println!();
            }
        }
    }
}
