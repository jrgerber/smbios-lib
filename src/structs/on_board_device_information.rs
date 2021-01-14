use super::*;

pub struct SMBiosOnBoardDeviceInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOnBoardDeviceInformation<'a> {
    const STRUCT_TYPE: u8 = 10u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOnBoardDeviceInformation<'a> {
    fn device_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x4)
    }

    fn device_description(&self) -> Option<u8> {
        self.parts.get_field_byte(0x5)
    }

    // fn minimum_ending_offset(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x6)
    // }
}

impl fmt::Debug for SMBiosOnBoardDeviceInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOnBoardDeviceInformation>())
            .field("header", &self.parts.header)
            .field("device_type", &self.device_type())
            .field("device_description", &self.device_description())
            // .field("minimum_ending_offset", &self.minimum_ending_offset())
            .finish()
    }
}
