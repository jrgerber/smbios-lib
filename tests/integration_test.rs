use smbios;

#[test]
fn windows_dump() {
    match smbios::windows::get_raw_smbios_data() {
        Ok(raw_data) => {
            println!("raw_data: {:?}", raw_data);

            for parts in raw_data.smbios_table_data() {
                println!("{:?}", parts.struct_type_name());
            }
        }
        Err(err) => panic!("failure: {:?}", err),
    }
}
