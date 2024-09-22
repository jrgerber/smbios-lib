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
        Ok(data) => match data.find_map(|sys_info: SMBiosSystemInformation| sys_info.uuid()) {
            Some(uuid) => println!("System Information UUID == {:?}", uuid),
            None => println!("No System Information (Type 1) structure found with a UUID field"),
        },
        Err(err) => assert!(false, "Failure: {:?}", err)
    }
}

#[test]
fn print_all_memory_devices() {
    match table_load_from_device() {
        Ok(data) => {
            for memory_device in data.collect::<SMBiosMemoryDevice>() {
                println!("{:#?}", memory_device);
            }
        },
        Err(err) => assert!(false, "Failure: {:?}", err)
    }
}

/// Finds an associated struct by handle
#[test]
fn struct_struct_association() {
    match table_load_from_device() {
        Ok(data) => match data.first::<SMBiosMemoryDevice>() {
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
        Err(err) => assert!(false, "Failure: {:?}", err)
    }
}

/// Test find() - finds the first populated CPU socket
#[test]
fn find_first_cpu() {
    match table_load_from_device() {
        Ok(data) => match data.find(|proc_info: &SMBiosProcessorInformation| match (proc_info.status(), proc_info.processor_type()) {
            (Some(status), Some(proc_type)) => { status.socket_populated() && proc_type.value == ProcessorType::CentralProcessor }
            _ => { false }
        }) {
            Some(first_cpu) => {
                println!("First populated CPU socket: {:#?}", first_cpu);
            }
            None => println!("No Processor Information (Type 4) structure found that is a CPU with a populated socket"),
        },
        Err(err) => assert!(false, "Failure: {:?}", err)
    }
}

/// Test filter() - finds all populated memory sockets
#[test]
fn find_installed_memory() {
    match table_load_from_device() {
        Err(err) => assert!(false, "Failure: {:?}", err),
        Ok(data) => data
            .filter(
                |memory_device: &SMBiosMemoryDevice| match memory_device.size() {
                    Some(size) => size != MemorySize::NotInstalled,
                    _ => false,
                },
            )
            .for_each(|installed_memory| println!("Installed memory: {:#X?}", installed_memory)),
    }
}
