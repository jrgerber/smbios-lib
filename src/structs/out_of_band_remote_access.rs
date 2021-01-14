use super::*;

pub struct SMBiosOutOfBandRemoteAccess<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOutOfBandRemoteAccess<'a> {
    const STRUCT_TYPE: u8 = 30u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOutOfBandRemoteAccess<'a> {
    fn manufacturer_name(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn connections(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }
}

impl fmt::Debug for SMBiosOutOfBandRemoteAccess<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOutOfBandRemoteAccess>())
            .field("header", &self.parts.header)
            .field("manufacturer_name", &self.manufacturer_name())
            .field("connections", &self.connections())
            .finish()
    }
}
