use super::*;

pub struct SMBiosBuiltInPointingDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosBuiltInPointingDevice<'a> {
    const STRUCT_TYPE: u8 = 21u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosBuiltInPointingDevice<'a> {
    fn device_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn interface(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn number_of_buttons(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }
}

impl fmt::Debug for SMBiosBuiltInPointingDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosBuiltInPointingDevice>())
            .field("header", &self.parts.header)
            .field("device_type", &self.device_type())
            .field("interface", &self.interface())
            .field("number_of_buttons", &self.number_of_buttons())
            .finish()
    }
}
