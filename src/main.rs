use smbioslib::*;

fn main() {
    match table_load_from_device() {
        Ok(table) => {
            println!("table_data: {:#?}", table);
        }
        Err(err) => panic!("failure: {:?}", err),
    }
}
