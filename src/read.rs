use crate::windows;
use std::fs;
use std::io;

/// Loads raw smbios data from a file
pub fn load_smbios_data_file(filename: &str) -> Result<windows::RawSMBiosData, io::Error> {
    let data = fs::read(filename)?;
    Ok(windows::RawSMBiosData::new(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_smbios_data_file() {
        let filename = ".\\tests\\jeffgerlap_3_2_0.dat";
        let data = load_smbios_data_file(&filename);
        match data {
            Ok(thing) => {
                println!("data: {:?}", thing);
                match thing.smbios_table_data() {
                    Some(table_data) => {
                        for parts in table_data {
                            println!("{:?}", parts.struct_type_name());
                        }
                    }
                    _ => panic!("Expected table data!"),
                }
            }
            _ => panic!("Expected data!"),
        }
    }
}
