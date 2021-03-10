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
        Ok(data) => match data.first_defined_struct::<SMBiosSystemInformation>() {
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
            for memory_device in data.collect_defined_struct::<SMBiosMemoryDevice>() {
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
        Ok(data) => match data.first_defined_struct::<SMBiosMemoryDevice>() {
            Some(first_memory_device) => {
                let handle = first_memory_device.physical_memory_array_handle().unwrap();
                match data.find_handle(&handle) {
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

/// Test find_defined_struct() - finds the first populated CPU socket
#[test]
fn find_first_cpu() {
    match table_load_from_device() {
        Ok(data) => match data.find_defined_struct(|proc_info: &SMBiosProcessorInformation| match (proc_info.status(), proc_info.processor_type()) {
            (Some(status), Some(proc_type)) => { status.socket_populated() && proc_type.value == ProcessorType::CentralProcessor }
            _ => { false }
        }) {
            Some(first_cpu) => {
                println!("First populated CPU socket: {:#?}", first_cpu);
            }
            None => println!("No Processor Information (Type 4) structure found that is a CPU with a populated socket"),
        },
        Err(err) => println!("Table load failure: {:?}", err),
    }
}
