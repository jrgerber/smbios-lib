use super::*;

pub struct SMBiosSystemReset<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemReset<'a> {
    const STRUCT_TYPE: u8 = 23u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemReset<'a> {
    fn capabilities(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn reset_count(&self) -> Option<u16> {
        self.parts.get_field_word(0x05)
    }

    fn reset_limit(&self) -> Option<u16> {
        self.parts.get_field_word(0x07)
    }

    fn timer_interval(&self) -> Option<u16> {
        self.parts.get_field_word(0x09)
    }

    fn timeout(&self) -> Option<u16> {
        self.parts.get_field_word(0x0B)
    }
}

impl fmt::Debug for SMBiosSystemReset<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemReset>())
        .field("header", &self.parts.header)
        .field("capabilities", &self.capabilities())
        .field("reset_count", &self.reset_count())
        .field("reset_limit", &self.reset_limit())
        .field("timer_interval", &self.timer_interval())
        .field("timeout", &self.timeout())
        .finish()
    }
}

