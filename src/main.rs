use smbioslib::*;

#[cfg(target_family = "windows")]
fn main() {
    match get_raw_smbios_data() {
        Ok(raw_data) => {
            for parts in &raw_data.smbios_table_data {
                println!("{:?}", parts.struct_type_name());
            }
        }
        Err(err) => panic!("failure: {:?}", err),
    }
}

#[cfg(target_family = "unix")]
fn main() {
    match table_load_from_device() {
        Ok(raw_data) => {
            // println!("raw_data: {:?}", raw_data);

            for parts in raw_data.into_iter() {
                println!("{:?}", parts.struct_type_name());
            }
        }
        Err(err) => panic!("failure: {:?}", err),
    }
}
