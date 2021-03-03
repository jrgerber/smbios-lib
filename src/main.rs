use smbioslib::*;


fn print_usage(program: &str, opts: getopts::Options) {
	let brief = format!("Usage: {} [-f|-o FILE]", program);
	print!("{}", opts.usage(&brief));
}

fn main() {
    let file_option = "f";
    let output_option = "o";

    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt(file_option, "", "read smbios table from file", "FILE");
    opts.optopt(output_option, "", "dump smbios table to a file", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(err) => {
            eprintln!("Error: {:?}", err);
            print_usage(&args[0].clone(), opts);
            std::process::exit(1);
        }
    };

    if !matches.opt_present(file_option) && 
        !matches.opt_present(output_option) {
        match table_load_from_device() {
            Ok(table) => {
                println!("table_data: {:#?}", table);
                return;
            },
            Err(err) => {
                panic!("Unable to load table from device: {:#?}", err)
            }
        }
	}

    match matches.opt_str(file_option) {
        Some(filename) => {
            match load_smbios_data_from_file(&filename) {
                Ok(table) => {
                    println!("Load table from file: {} \n{:#?}", &filename, table);
                    return
                },
                Err(err) => {
                    eprintln!("Error loading from file: {:?}", err);
                    std::process::exit(1);
                }
            }
        },
        None => ()
    }

    match matches.opt_str(output_option) {
        Some(filename) => {
            match raw_smbios_from_device() {
                Ok(raw) => {
                    match dump_raw(raw, &filename) {
                        Err(err) => panic!("{}", err),
                        Ok(_) => return
                    }
                },
                Err(err) => {
                    panic!("Unable to load table from device: {:#?}", err)
                }
            }
        }
        None => ()
    }
}
