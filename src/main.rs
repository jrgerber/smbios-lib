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
        "bios-vendor" => match data.first::<SMBiosInformation>() {
            Some(bios_info) => match bios_info.vendor() {
                Ok(vendor) => Ok(vendor),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BiosVendorNotFound),
                },
            },
            None => Err(BiosParseError::BiosVendorNotFound),
        },
        "bios-version" => match data.first::<SMBiosInformation>() {
            Some(bios_info) => match bios_info.version() {
                Ok(version) => Ok(version),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BiosVersionNotFound),
                },
            },
            None => Err(BiosParseError::BiosVersionNotFound),
        },
        "bios-release-date" => match data.first::<SMBiosInformation>() {
            Some(bios_info) => match bios_info.release_date() {
                Ok(release_date) => Ok(release_date),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BiosReleaseDateNotFound),
                },
            },
            None => Err(BiosParseError::BiosReleaseDateNotFound),
        },
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
        "system-manufacturer" => match data.first::<SMBiosSystemInformation>() {
            Some(system_info) => match system_info.manufacturer() {
                Ok(manufacturer) => Ok(manufacturer),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::SystemManufacturerNotFound),
                },
            },
            None => Err(BiosParseError::SystemManufacturerNotFound),
        },
        "system-product-name" => match data.first::<SMBiosSystemInformation>() {
            Some(system_info) => match system_info.product_name() {
                Ok(system_product_name) => Ok(system_product_name),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::SystemProductNameNotFound),
                },
            },
            None => Err(BiosParseError::SystemProductNameNotFound),
        },
        "system-version" => match data.first::<SMBiosSystemInformation>() {
            Some(system_info) => match system_info.version() {
                Ok(version) => Ok(version),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::SystemVersionNotFound),
                },
            },
            None => Err(BiosParseError::SystemVersionNotFound),
        },
        "system-serial-number" => match data.first::<SMBiosSystemInformation>() {
            Some(system_info) => match system_info.serial_number() {
                Ok(serial_number) => Ok(serial_number),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::SystemSerialNumberNotFound),
                },
            },
            None => Err(BiosParseError::SystemSerialNumberNotFound),
        },
        "system-uuid" => {
            match data.find_map(|system_info: SMBiosSystemInformation| system_info.uuid()) {
                // SystemUuidData is an enum that can be broken down further if desired
                Some(uuid) => Ok(format!("{}", uuid)),
                None => Err(BiosParseError::SystemUuidNotFound),
            }
        }
        "system-sku-number" => match data.first::<SMBiosSystemInformation>() {
            Some(system_info) => match system_info.sku_number() {
                Ok(sku_number) => Ok(sku_number),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::SystemSkuNumberNotFound),
                },
            },
            None => Err(BiosParseError::SystemSkuNumberNotFound),
        },
        "system-family" => match data.first::<SMBiosSystemInformation>() {
            Some(system_info) => match system_info.family() {
                Ok(family) => Ok(family),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::SystemFamilyNotFound),
                },
            },
            None => Err(BiosParseError::SystemFamilyNotFound),
        },
        "baseboard-manufacturer" => match data.first::<SMBiosBaseboardInformation>() {
            Some(baseboard_info) => match baseboard_info.manufacturer() {
                Ok(manufacturer) => Ok(manufacturer),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BaseboardManufacturerNotFound),
                },
            },
            None => Err(BiosParseError::BaseboardManufacturerNotFound),
        },
        "baseboard-product-name" => match data.first::<SMBiosBaseboardInformation>() {
            Some(baseboard_info) => match baseboard_info.product() {
                Ok(product) => Ok(product),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BaseboardProductNameNotFound),
                },
            },
            None => Err(BiosParseError::BaseboardProductNameNotFound),
        },
        "baseboard-version" => match data.first::<SMBiosBaseboardInformation>() {
            Some(baseboard_info) => match baseboard_info.version() {
                Ok(version) => Ok(version),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BaseboardVersionNotFound),
                },
            },
            None => Err(BiosParseError::BaseboardVersionNotFound),
        },
        "baseboard-serial-number" => match data.first::<SMBiosBaseboardInformation>() {
            Some(baseboard_info) => match baseboard_info.serial_number() {
                Ok(serial_number) => Ok(serial_number),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BaseboardSerialNumberNotFound),
                },
            },
            None => Err(BiosParseError::BaseboardSerialNumberNotFound),
        },
        "baseboard-asset-tag" => match data.first::<SMBiosBaseboardInformation>() {
            Some(baseboard_info) => match baseboard_info.asset_tag() {
                Ok(asset_tag) => Ok(asset_tag),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::BaseboardAssetTagNotFound),
                },
            },
            None => Err(BiosParseError::BaseboardAssetTagNotFound),
        },
        "chassis-manufacturer" => match data.first::<SMBiosSystemChassisInformation>() {
            Some(chassis_info) => match chassis_info.manufacturer() {
                Ok(manufacturer) => Ok(manufacturer),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::ChassisManufacturerNotFound),
                },
            },
            None => Err(BiosParseError::ChassisManufacturerNotFound),
        },
        "chassis-type" => match data
            .find_map(|chassis_info: SMBiosSystemChassisInformation| chassis_info.chassis_type())
        {
            Some(chassis_type) => Ok(format!("{}", chassis_type)),
            None => Err(BiosParseError::ChassisTypeNotFound),
        },
        "chassis-version" => match data.first::<SMBiosSystemChassisInformation>() {
            Some(chassis_info) => match chassis_info.version() {
                Ok(version) => Ok(version),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::ChassisVersionNotFound),
                },
            },
            None => Err(BiosParseError::ChassisVersionNotFound),
        },
        "chassis-serial-number" => match data.first::<SMBiosSystemChassisInformation>() {
            Some(chassis_info) => match chassis_info.serial_number() {
                Ok(serial_number) => Ok(serial_number),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::ChassisSerialNumberNotFound),
                },
            },
            None => Err(BiosParseError::ChassisSerialNumberNotFound),
        },
        "chassis-asset-tag" => match data.first::<SMBiosSystemChassisInformation>() {
            Some(chassis_info) => match chassis_info.asset_tag_number() {
                Ok(asset_tag_number) => Ok(asset_tag_number),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::ChassisAssetTagNotFound),
                },
            },
            None => Err(BiosParseError::ChassisAssetTagNotFound),
        },
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
        "processor-manufacturer" => match data.first::<SMBiosProcessorInformation>() {
            Some(processor_info) => match processor_info.processor_manufacturer() {
                Ok(processor_manufacturer) => Ok(processor_manufacturer),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::ProcessorManufacturerNotFound),
                },
            },
            None => Err(BiosParseError::ProcessorManufacturerNotFound),
        },
        "processor-version" => match data.first::<SMBiosProcessorInformation>() {
            Some(processor_info) => match processor_info.processor_version() {
                Ok(processor_version) => Ok(processor_version),
                Err(err) => match err {
                    SMBiosStringError::Utf8(utf8) => {
                        Ok(String::from_utf8_lossy(utf8.as_bytes()).to_string())
                    }
                    _ => Err(BiosParseError::ProcessorVersionNotFound),
                },
            },
            None => Err(BiosParseError::ProcessorVersionNotFound),
        },
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
