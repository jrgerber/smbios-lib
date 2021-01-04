use super::*;

pub struct SMBiosSystemChassisInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemChassisInformation<'a> {
    const STRUCT_TYPE: u8 = 3u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemChassisInformation<'a> {
    fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn chassis_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    fn asset_tag_number(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    fn bootup_state(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    fn power_supply_state(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0A)
    }

    fn thermal_state(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0B)
    }

    fn security_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0C)
    }

    fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0D)
    }

    fn height(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    fn number_of_power_cords(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    fn contained_element_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x13)
    }

    fn contained_element_record_length(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    // fn contained_elements(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x15)
    // }

    fn sku_number(&self) -> Option<String> {
        self.parts.get_field_string(0x15)
    }
}

impl fmt::Debug for SMBiosSystemChassisInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemChassisInformation>())
        .field("header", &self.parts.header)
        .field("manufacturer", &self.manufacturer())
        .field("chassis_type", &self.chassis_type())
        .field("version", &self.version())
        .field("serial_number", &self.serial_number())
        .field("asset_tag_number", &self.asset_tag_number())
        .field("bootup_state", &self.bootup_state())
        .field("power_supply_state", &self.power_supply_state())
        .field("thermal_state", &self.thermal_state())
        .field("security_status", &self.security_status())
        .field("oem_defined", &self.oem_defined())
        .field("height", &self.height())
        .field("number_of_power_cords", &self.number_of_power_cords())
        .field("contained_element_count", &self.contained_element_count())
        .field("contained_element_record_length", &self.contained_element_record_length())
        // .field("contained_elements", &self.contained_elements())
        .field("sku_number", &self.sku_number())
        .finish()
    }
}

