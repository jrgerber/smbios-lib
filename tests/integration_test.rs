use smbioslib::*;

#[cfg(target_family = "windows")]
#[test]
fn windows_dump() {
    match get_raw_smbios_data() {
        Ok(raw_data) => {
            println!("raw_data: {:?}", raw_data);

            for parts in &raw_data.smbios_table_data {
                println!("{:?}", parts.struct_type_name());
            }
        }
        Err(err) => panic!("failure: {:?}", err),
    }
}

#[cfg(target_family = "windows")]
#[test]
fn windows_retrieve_system_uuid() {
    // Load table data from the Windows device
    match get_raw_smbios_data() {
        Ok(raw_data) => {
            let mut iterator = raw_data.smbios_table_data.into_iter();

            // Search the table data for the first System Information (Type 1) structure
            match iterator.find(|current_struct| {
                current_struct.header.struct_type() == SMBiosSystemInformation::STRUCT_TYPE
            }) {
                Some(base_struct) =>
                // Down cast the structure to an SMBIOSSystemInformation structure
                {
                    match base_struct.struct_type_name() {
                        DefinedStruct::SystemInformation(system_information) => {
                            println!(
                                "System Information UUID == {:?}",
                                system_information.uuid().unwrap()
                            )
                        }
                        _ => panic!("Downcasting library design failure"),
                    }
                }
                None => println!("No System Information (Type 1) structure found"),
            }
        }
        Err(err) => println!("failure: {:?}", err),
    }
}
