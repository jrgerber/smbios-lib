use smbioslib::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_option = "f";
    let output_option = "o";

    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt(file_option, "", "read smbios table from file", "FILE");
    opts.optopt(output_option, "", "dump smbios table to a file", "FILE");

    let matches = opts.parse(&args[1..])?;

    if !matches.opt_present(file_option) &&
       !matches.opt_present(output_option) {
        println!("table_data: {:#?}", table_load_from_device()?);
        return Ok(())
    }

    match matches.opt_str(file_option) {
        Some(filename) => {
            println!("Load table from file: {} \n{:#?}", &filename,
                load_smbios_data_from_file(&filename)?);
        },
        None => ()
    }
    match matches.opt_str(output_option) {
        Some(filename) => {
            dump_raw(raw_smbios_from_device()?, &filename)?;
        },
        None => ()
    }
    Ok(())
}
