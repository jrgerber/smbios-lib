use super::*;

pub struct SMBiosCoolingDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosCoolingDevice<'a> {
    const STRUCT_TYPE: u8 = 27u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosCoolingDevice<'a> {
    fn temperature_probe_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x04)
    }

    fn device_type_and_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn cooling_unit_group(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x08)
    }

    fn nominal_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x0E)
    }
}

impl fmt::Debug for SMBiosCoolingDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosCoolingDevice>())
            .field("header", &self.parts.header)
            .field("temperature_probe_handle", &self.temperature_probe_handle())
            .field("device_type_and_status", &self.device_type_and_status())
            .field("cooling_unit_group", &self.cooling_unit_group())
            .field("oem_defined", &self.oem_defined())
            .field("nominal_speed", &self.nominal_speed())
            .field("description", &self.description())
            .finish()
    }
}
