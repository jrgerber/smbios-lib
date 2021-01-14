use super::*;

pub struct SMBiosMemoryChannel<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryChannel<'a> {
    const STRUCT_TYPE: u8 = 37u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryChannel<'a> {
    fn channel_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn maximum_channel_load(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn memory_device_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn memory_device_load(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    fn memory_device_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }
}

impl fmt::Debug for SMBiosMemoryChannel<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryChannel>())
            .field("header", &self.parts.header)
            .field("channel_type", &self.channel_type())
            .field("maximum_channel_load", &self.maximum_channel_load())
            .field("memory_device_count", &self.memory_device_count())
            .field("memory_device_load", &self.memory_device_load())
            .field("memory_device_handle", &self.memory_device_handle())
            .finish()
    }
}
