use super::*;

pub struct SMBiosAdditionalInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosAdditionalInformation<'a> {
    const STRUCT_TYPE: u8 = 40u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosAdditionalInformation<'a> {
    fn number_of_entries(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    // fn entries(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x05)
    // }

    // fn minimum_ending_offset(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x0B)
    // }
}

impl fmt::Debug for SMBiosAdditionalInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosAdditionalInformation>())
        .field("header", &self.parts.header)
        .field("number_of_entries", &self.number_of_entries())
        // .field("entries", &self.entries())
        // .field("minimum_ending_offset", &self.minimum_ending_offset())
        .finish()
    }
}

