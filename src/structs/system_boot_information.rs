use super::*;

pub struct SMBiosSystemBootInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemBootInformation<'a> {
    const STRUCT_TYPE: u8 = 32u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemBootInformation<'a> {
    // fn reserved(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x04)
    // }

    // fn boot_status(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x0A)
    // }
}

impl fmt::Debug for SMBiosSystemBootInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemBootInformation>())
            .field("header", &self.parts.header)
            // .field("reserved", &self.reserved())
            // .field("boot_status", &self.boot_status())
            .finish()
    }
}
