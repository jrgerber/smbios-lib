use smbioslib::*;

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