use std::{error::Error, fmt::Display, path::Path};

use smbioslib::*;

#[derive(Debug)]
enum BiosParseError {
    BiosVendorNotFound,
    BiosVersionNotFound,
    BiosReleaseDateNotFound,
    BiosRevisionNotFound,
    FirmewareRevisionNotFound,
    SystemManufacturerNotFound,
    SystemProductNameNotFound,
    SystemVersionNotFound,
    SystemSerialNumberNotFound,
    SystemUuidNotFound,
    SystemSkuNumberNotFound,
    SystemFamilyNotFound,
    BaseboardManufacturerNotFound,
    BaseboardProductNameNotFound,
    BaseboardVersionNotFound,
    BaseboardSerialNumberNotFound,
    BaseboardAssetTagNotFound,
    ChassisManufacturerNotFound,
    ChassisTypeNotFound,
    ChassisVersionNotFound,
    ChassisSerialNumberNotFound,
    ChassisAssetTagNotFound,
    ProcessorFamilyNotFound,
    ProcessorManufacturerNotFound,
    ProcessorVersionNotFound,
    ProcessorFrequencyNotFound,
    InvalidKeywordOnCommandLine,
}

impl Error for BiosParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for BiosParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Here we can match and turn each arm into a human readable statement.
        // We have other variants to add so we will wait before doing so.
        write!(f, "{:?}", &self)
    }
}

fn string_keyword(keyword: String, data: &SMBiosData) -> Result<String, BiosParseError> {
    match keyword.to_lowercase().as_str() {
        "bios-vendor" => data
            .find_map(|bios_info: SMBiosInformation| bios_info.vendor().to_utf8_lossy())
            .ok_or(BiosParseError::BiosVendorNotFound),
        "bios-version" => data
            .find_map(|bios_info: SMBiosInformation| bios_info.version().to_utf8_lossy())
            .ok_or(BiosParseError::BiosVersionNotFound),
        "bios-release-date" => data
            .find_map(|bios_info: SMBiosInformation| bios_info.release_date().to_utf8_lossy())
            .ok_or(BiosParseError::BiosReleaseDateNotFound),
        "bios-revision" => data
            .find_map(|bios_info: SMBiosInformation| {
                match (
                    bios_info.system_bios_major_release(),
                    bios_info.system_bios_minor_release(),
                ) {
                    (Some(major), Some(minor)) => Some(format!("{}.{}", major, minor)),
                    _ => None,
                }
            })
            .ok_or(BiosParseError::BiosRevisionNotFound),
        "firmware-revision" => data
            .find_map(|bios_info: SMBiosInformation| {
                match (
                    bios_info.e_c_firmware_major_release(),
                    bios_info.e_c_firmware_minor_release(),
                ) {
                    (Some(major), Some(minor)) => Some(format!("{}.{}", major, minor)),
                    _ => None,
                }
            })
            .ok_or(BiosParseError::FirmewareRevisionNotFound),
        "system-manufacturer" => data
            .find_map(|system_info: SMBiosSystemInformation| {
                system_info.manufacturer().to_utf8_lossy()
            })
            .ok_or(BiosParseError::SystemManufacturerNotFound),
        "system-product-name" => data
            .find_map(|system_info: SMBiosSystemInformation| {
                system_info.product_name().to_utf8_lossy()
            })
            .ok_or(BiosParseError::SystemProductNameNotFound),
        "system-version" => data
            .find_map(|system_info: SMBiosSystemInformation| system_info.version().to_utf8_lossy())
            .ok_or(BiosParseError::SystemVersionNotFound),
        "system-serial-number" => data
            .find_map(|system_info: SMBiosSystemInformation| {
                system_info.serial_number().to_utf8_lossy()
            })
            .ok_or(BiosParseError::SystemSerialNumberNotFound),
        "system-uuid" => {
            match data.find_map(|system_info: SMBiosSystemInformation| system_info.uuid()) {
                // SystemUuidData is an enum that can be broken down further if desired
                Some(uuid) => Ok(format!("{}", uuid)),
                None => Err(BiosParseError::SystemUuidNotFound),
            }
        }
        "system-sku-number" => data
            .find_map(|system_info: SMBiosSystemInformation| {
                system_info.sku_number().to_utf8_lossy()
            })
            .ok_or(BiosParseError::SystemSkuNumberNotFound),
        "system-family" => data
            .find_map(|system_info: SMBiosSystemInformation| system_info.family().to_utf8_lossy())
            .ok_or(BiosParseError::SystemFamilyNotFound),
        "baseboard-manufacturer" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation| {
                baseboard_info.manufacturer().to_utf8_lossy()
            })
            .ok_or(BiosParseError::BaseboardManufacturerNotFound),
        "baseboard-product-name" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation| {
                baseboard_info.product().to_utf8_lossy()
            })
            .ok_or(BiosParseError::BaseboardProductNameNotFound),
        "baseboard-version" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation| {
                baseboard_info.version().to_utf8_lossy()
            })
            .ok_or(BiosParseError::BaseboardVersionNotFound),
        "baseboard-serial-number" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation| {
                baseboard_info.serial_number().to_utf8_lossy()
            })
            .ok_or(BiosParseError::BaseboardSerialNumberNotFound),
        "baseboard-asset-tag" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation| {
                baseboard_info.asset_tag().to_utf8_lossy()
            })
            .ok_or(BiosParseError::BaseboardAssetTagNotFound),
        "chassis-manufacturer" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation| {
                chassis_info.manufacturer().to_utf8_lossy()
            })
            .ok_or(BiosParseError::ChassisManufacturerNotFound),
        "chassis-type" => match data
            .find_map(|chassis_info: SMBiosSystemChassisInformation| chassis_info.chassis_type())
        {
            Some(chassis_type) => Ok(format!("{}", chassis_type)),
            None => Err(BiosParseError::ChassisTypeNotFound),
        },
        "chassis-version" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation| {
                chassis_info.version().to_utf8_lossy()
            })
            .ok_or(BiosParseError::ChassisVersionNotFound),
        "chassis-serial-number" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation| {
                chassis_info.serial_number().to_utf8_lossy()
            })
            .ok_or(BiosParseError::ChassisSerialNumberNotFound),
        "chassis-asset-tag" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation| {
                chassis_info.asset_tag_number().to_utf8_lossy()
            })
            .ok_or(BiosParseError::ChassisAssetTagNotFound),
        "processor-family" => match data.first::<SMBiosProcessorInformation>() {
            Some(processor_info) => match processor_info.processor_family() {
                Some(family) => match family.value {
                    ProcessorFamily::SeeProcessorFamily2 => {
                        match processor_info.processor_family_2() {
                            Some(family) => Ok(format!("{}", family)),
                            None => Err(BiosParseError::ProcessorFamilyNotFound),
                        }
                    }
                    _ => Ok(format!("{}", family)),
                },
                None => Err(BiosParseError::ProcessorFamilyNotFound),
            },
            None => Err(BiosParseError::ProcessorFamilyNotFound),
        },
        "processor-manufacturer" => data
            .find_map(|processor_info: SMBiosProcessorInformation| {
                processor_info.processor_manufacturer().to_utf8_lossy()
            })
            .ok_or(BiosParseError::ProcessorManufacturerNotFound),
        "processor-version" => data
            .find_map(|processor_info: SMBiosProcessorInformation| {
                processor_info.processor_version().to_utf8_lossy()
            })
            .ok_or(BiosParseError::ProcessorVersionNotFound),
        "processor-frequency" => match data
            .find_map(|processor_info: SMBiosProcessorInformation| processor_info.current_speed())
        {
            Some(current_speed) => Ok(format!("{:?}", current_speed)),
            None => Err(BiosParseError::ProcessorFrequencyNotFound),
        },
        _ => Err(BiosParseError::InvalidKeywordOnCommandLine),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_option = "f";
    let output_option = "o";
    let string_option = "s";
    let json_option = "j";

    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt(file_option, "", "read smbios table from file", "FILE");
    opts.optopt(output_option, "", "dump smbios table to a file", "FILE");
    opts.optopt(
        string_option,
        "",
        "Only display the value of the DMI string identified by KEYWORD.",
        "KEYWORD",
    );
    opts.optflag(json_option, "", "output in json format");

    let matches = opts.parse(&args[1..])?;

    if !matches.opt_present(file_option)
        && !matches.opt_present(output_option)
        && !matches.opt_present(string_option)
        && !matches.opt_present(json_option)
    {
        println!("table_data: {:#?}", table_load_from_device()?);
        return Ok(());
    }

    match matches.opt_str(file_option) {
        Some(filename) => {
            let file_path = Path::new(&filename);
            println!("{:#?}", load_smbios_data_from_file(&file_path)?);
        }
        None => (),
    }

    match matches.opt_str(output_option) {
        Some(filename) => {
            let out_path = Path::new(&filename);
            dump_raw(raw_smbios_from_device()?, &out_path)?;
        }
        None => (),
    }

    match matches.opt_str(string_option) {
        Some(keyword) => {
            let smbios_data = table_load_from_device()?;
            let output = string_keyword(keyword, &smbios_data)?;
            println!("{}", output);
        }
        None => (),
    }
    if matches.opt_present(json_option) {
        let smbios_data = table_load_from_device()?;
        if let Ok(output) = serde_json::to_string(&smbios_data) {
            println!("{}", output)
        }
    }

    Ok(())
}
