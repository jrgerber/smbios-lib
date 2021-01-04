use super::*;

pub struct SMBiosSystemSlot<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemSlot<'a> {
    const STRUCT_TYPE: u8 = 9u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemSlot<'a> {
    fn data_bus_width(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }
}

impl fmt::Debug for SMBiosSystemSlot<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemSlot>())
        .field("header", &self.parts.header)
        .field("data_bus_width", &self.data_bus_width())
        .finish()
    }
}

