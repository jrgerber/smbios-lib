use smbioslib::*;

#[cfg(target_family = "windows")]
#[test]
fn windows_dump() {
    match load_windows_smbios_data() {
        Ok(windows_data) => {
            println!("windows_data: {:#?}", windows_data);
        }
        Err(err) => panic!("failure: {:?}", err),
    }
}

#[test]
fn retrieve_system_uuid() {
    match table_load_from_device() {
        Ok(data) => match data.find_first::<SMBiosSystemInformation>() {
            Some(system_information) => println!(
                "System Information UUID == {:?}",
                system_information.uuid().unwrap()
            ),
            None => println!("No System Information (Type 1) structure found"),
        },
        Err(err) => println!("failure: {:?}", err),
    }
}

#[test]
fn print_all_memory_devices() {
    match table_load_from_device() {
        Ok(data) => {
            for memory_device in data.find_all::<SMBiosMemoryDevice>() {
                println!("{:#?}", memory_device);
            }
        }
        Err(err) => println!("failure: {:?}", err),
    }
}

/// Finds an associated struct by handle
#[test]
fn struct_struct_association() {
    match table_load_from_device() {
        Ok(data) => match data.find_first::<SMBiosMemoryDevice>() {
            Some(first_memory_device) => {
                let handle = first_memory_device.physical_memory_array_handle().unwrap();
                match data.find_by_handle(&handle) {
                    Some(undefined_struct) => {
                        let physical_memory_array = undefined_struct.defined_struct();
                        println!("{:#?}", physical_memory_array)
                    }
                    None => println!("No Physical Memory Array (Type 16) structure found"),
                }
            }
            None => println!("No Memory Device (Type 17) structure found"),
        },
        Err(err) => println!("failure: {:?}", err),
    }
}
