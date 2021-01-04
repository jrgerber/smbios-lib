use super::*;

pub struct SMBiosPhysicalMemoryArray<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosPhysicalMemoryArray<'a> {
    const STRUCT_TYPE: u8 = 16u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosPhysicalMemoryArray<'a> {
    fn location(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn usage(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn memory_error_correction(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn maximum_capacity(&self) -> Option<u32> {
        self.parts.get_field_dword(0x07)
    }

    fn memory_error_information_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x0B)
    }

    fn number_of_memory_devices(&self) -> Option<u16> {
        self.parts.get_field_word(0x0D)
    }

    fn extended_maximum_capacity(&self) -> Option<u64> {
        self.parts.get_field_qword(0x0F)
    }
}

impl fmt::Debug for SMBiosPhysicalMemoryArray<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPhysicalMemoryArray>())
        .field("header", &self.parts.header)
        .field("location", &self.location())
        .field("usage", &self.usage())
        .field("memory_error_correction", &self.memory_error_correction())
        .field("maximum_capacity", &self.maximum_capacity())
        .field("memory_error_information_handle", &self.memory_error_information_handle())
        .field("number_of_memory_devices", &self.number_of_memory_devices())
        .field("extended_maximum_capacity", &self.extended_maximum_capacity())
        .finish()
    }
}

