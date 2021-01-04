use super::*;

pub struct SMBiosTemperatureProbe<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosTemperatureProbe<'a> {
    const STRUCT_TYPE: u8 = 28u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosTemperatureProbe<'a> {
    fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn location_and_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn maximum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    fn minimum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    fn resolution(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    fn tolerance(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    fn accuracy(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }

    fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    fn nominal_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }
}

impl fmt::Debug for SMBiosTemperatureProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosTemperatureProbe>())
        .field("header", &self.parts.header)
        .field("description", &self.description())
        .field("location_and_status", &self.location_and_status())
        .field("maximum_value", &self.maximum_value())
        .field("minimum_value", &self.minimum_value())
        .field("resolution", &self.resolution())
        .field("tolerance", &self.tolerance())
        .field("accuracy", &self.accuracy())
        .field("oem_defined", &self.oem_defined())
        .field("nominal_value", &self.nominal_value())
        .finish()
    }
}

