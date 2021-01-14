use super::*;

pub struct SMBiosSystemConfigurationOptions<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemConfigurationOptions<'a> {
    const STRUCT_TYPE: u8 = 12u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemConfigurationOptions<'a> {}

impl fmt::Debug for SMBiosSystemConfigurationOptions<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemConfigurationOptions>())
            .field("header", &self.parts.header)
            .finish()
    }
}
