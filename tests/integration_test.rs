use smbioslib::*;

#[cfg(target_family = "windows")]
#[test]
fn windows_dump() {
    match load_windows_smbios_data() {
        Ok(windows_data) => {
            println!("windows_data: {:?}", windows_data);

            for undefined_struct in windows_data.smbios_data.into_iter() {
                println!("{:#?}", undefined_struct.defined_struct());
            }
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
