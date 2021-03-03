use smbioslib::*;


fn print_usage(program: &str, opts: getopts::Options) {
	let brief = format!("Usage: {} -f FILE", program);
	print!("{}", opts.usage(&brief));
}

fn main() {
    let file_option = "f";
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt(file_option, "", "smbios table filename", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(err) => {
            print_usage(&args[0].clone(), opts);
            std::process::exit(1);
        }
    };

    match matches.opt_str(file_option) {
        Some(s) => {
            match load_smbios_data_from_file(&s) {
                Ok(table) => { println!("table from file: {} \n{:#?}", &s, table) },
                Err(err) => {
                    eprintln!("Error loading file: {:?}", err);
                    std::process::exit(1);
                }
            }
        },
        None => {
            println!("No option specified. Getting table from device");
            match table_load_from_device() {
                Ok(table) => {
                    println!("table_data: {:#?}", table);
                    return
                },
                Err(err) => { panic!("Unable to load table from device: {:#?}", err) }
            }
         }
    }
}
