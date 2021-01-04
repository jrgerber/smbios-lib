use super::*;

pub struct SMBiosPortableBattery<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosPortableBattery<'a> {
    const STRUCT_TYPE: u8 = 22u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosPortableBattery<'a> {
    fn location(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    fn manufacture_date(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    fn device_name(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    fn device_chemistry(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    fn design_capacity(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    fn design_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    fn sbds_version_number(&self) -> Option<String> {
        self.parts.get_field_string(0x0E)
    }

    fn maximum_error_in_battery_data(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    fn sbds_serial_number(&self) -> Option<u16> {
        self.parts.get_field_word(0x10)
    }

    fn sbds_manufacture_date(&self) -> Option<u16> {
        self.parts.get_field_word(0x12)
    }

    fn sbds_device_chemistry(&self) -> Option<String> {
        self.parts.get_field_string(0x14)
    }

    fn design_capacity_multiplier(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    fn oem_specific(&self) -> Option<u32> {
        self.parts.get_field_dword(0x16)
    }
}

impl fmt::Debug for SMBiosPortableBattery<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPortableBattery>())
        .field("header", &self.parts.header)
        .field("location", &self.location())
        .field("manufacturer", &self.manufacturer())
        .field("manufacture_date", &self.manufacture_date())
        .field("serial_number", &self.serial_number())
        .field("device_name", &self.device_name())
        .field("device_chemistry", &self.device_chemistry())
        .field("design_capacity", &self.design_capacity())
        .field("design_voltage", &self.design_voltage())
        .field("sbds_version_number", &self.sbds_version_number())
        .field("maximum_error_in_battery_data", &self.maximum_error_in_battery_data())
        .field("sbds_serial_number", &self.sbds_serial_number())
        .field("sbds_manufacture_date", &self.sbds_manufacture_date())
        .field("sbds_device_chemistry", &self.sbds_device_chemistry())
        .field("design_capacity_multiplier", &self.design_capacity_multiplier())
        .field("oem_specific", &self.oem_specific())
        .finish()
    }
}

