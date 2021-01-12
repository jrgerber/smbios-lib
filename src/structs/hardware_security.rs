use super::*;

pub struct SMBiosHardwareSecurity<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosHardwareSecurity<'a> {
    const STRUCT_TYPE: u8 = 24u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosHardwareSecurity<'a> {
    fn hardware_security_settings(&self) -> Option<u8> {
        self.parts.get_field_byte(0x4)
    }
}

impl fmt::Debug for SMBiosHardwareSecurity<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosHardwareSecurity>())
        .field("header", &self.parts.header)
        .field("hardware_security_settings", &self.hardware_security_settings())
        .finish()
    }
}

