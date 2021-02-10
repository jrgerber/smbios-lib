use smbioslib::*;

fn main() {
    match table_load_from_device() {
        Ok(table) => {
            // TODO: Make SMBiosTableData dump all contents for fmt::Debug
            // and then remove the below for loop
            // println!("table_data: {:#?}", table);

            for smbios_structure in table.into_iter() {
                println!("{:#?}", smbios_structure.defined_struct());
            }
        }
        Err(err) => panic!("failure: {:?}", err),
    }
}
