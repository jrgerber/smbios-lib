use super::*;

pub struct SMBiosEndOfTable<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosEndOfTable<'a> {
    const STRUCT_TYPE: u8 = 127u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosEndOfTable<'a> {}

impl fmt::Debug for SMBiosEndOfTable<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosEndOfTable>())
            .field("header", &self.parts.header)
            .finish()
    }
}
