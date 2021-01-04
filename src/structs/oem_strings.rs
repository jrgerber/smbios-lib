use super::*;

pub struct SMBiosOemStrings<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOemStrings<'a> {
    const STRUCT_TYPE: u8 = 11u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOemStrings<'a> {
}

impl fmt::Debug for SMBiosOemStrings<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOemStrings>())
        .field("header", &self.parts.header)

        .finish()
    }
}

