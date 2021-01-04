use super::*;

pub struct SMBiosUnknown<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosUnknown<'a> {
    pub fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        SMBiosUnknown { parts: parts }
    }

    pub fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl fmt::Debug for SMBiosUnknown<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosUnknown>())
        .field("header", &self.parts.header)
        .finish()
    }
}
