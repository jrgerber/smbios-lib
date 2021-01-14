use super::*;

pub struct SMBiosInactive<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosInactive<'a> {
    const STRUCT_TYPE: u8 = 126u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosInactive<'a> {}

impl fmt::Debug for SMBiosInactive<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosInactive>())
            .field("header", &self.parts.header)
            .finish()
    }
}
