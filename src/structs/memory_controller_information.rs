use super::*;

pub struct SMBiosMemoryControllerInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryControllerInformation<'a> {
    const STRUCT_TYPE: u8 = 5u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryControllerInformation<'a> {
    fn error_detecting_method(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn error_correcting_capability(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn supported_interleave(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn current_interleave(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    fn maximum_memory_module_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }

    fn supported_speeds(&self) -> Option<u16> {
        self.parts.get_field_word(0x09)
    }

    fn supported_memory_types(&self) -> Option<u16> {
        self.parts.get_field_word(0x0B)
    }

    fn memory_module_voltage(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0D)
    }

    fn number_of_associated_memory_slots(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0E)
    }

    fn memory_module_configuration_handles(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }
}

impl fmt::Debug for SMBiosMemoryControllerInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryControllerInformation>())
        .field("header", &self.parts.header)
        .field("error_detecting_method", &self.error_detecting_method())
        .field("error_correcting_capability", &self.error_correcting_capability())
        .field("supported_interleave", &self.supported_interleave())
        .field("current_interleave", &self.current_interleave())
        .field("maximum_memory_module_size", &self.maximum_memory_module_size())
        .field("supported_speeds", &self.supported_speeds())
        .field("supported_memory_types", &self.supported_memory_types())
        .field("memory_module_voltage", &self.memory_module_voltage())
        .field("number_of_associated_memory_slots", &self.number_of_associated_memory_slots())
        .field("memory_module_configuration_handles", &self.memory_module_configuration_handles())
        .finish()
    }
}

