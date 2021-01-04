use super::*;

pub struct SMBiosBiosLanguageInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosBiosLanguageInformation<'a> {
    const STRUCT_TYPE: u8 = 13u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosBiosLanguageInformation<'a> {
    fn installable_languages(&self) -> Option<u8> {
        self.parts.get_field_byte(0x4)
    }

    fn flags(&self) -> Option<u8> {
        self.parts.get_field_byte(0x5)
    }

    // fn reserved(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x6)
    // }

    fn current_language(&self) -> Option<String> {
        self.parts.get_field_string(0x15)
    }
}

impl fmt::Debug for SMBiosBiosLanguageInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosBiosLanguageInformation>())
        .field("header", &self.parts.header)
        .field("installable_languages", &self.installable_languages())
        .field("flags", &self.flags())
        // .field("reserved", &self.reserved())
        .field("current_language", &self.current_language())
        .finish()
    }
}

