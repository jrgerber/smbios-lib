use smbios;

#[test]
fn windows_dump() {
    let fun = smbios::windows::get_raw_smbios_data();
    match fun {
        Some(thing) => {
            println!("data: {:?}", thing);
            match thing.smbios_table_data() {
                Some(table_data) => {
                    for parts in table_data {
                        println!("{:?}", parts.struct_type_name());
                    }
                }
                None => (),
            }
        }
        None => (),
    }
}
