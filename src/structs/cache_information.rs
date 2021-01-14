use super::*;

pub struct SMBiosCacheInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosCacheInformation<'a> {
    const STRUCT_TYPE: u8 = 7u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosCacheInformation<'a> {
    fn socket_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn cache_configuration(&self) -> Option<u16> {
        self.parts.get_field_word(0x05)
    }

    fn maximum_cache_size(&self) -> Option<u16> {
        self.parts.get_field_word(0x07)
    }

    fn installed_size(&self) -> Option<u16> {
        self.parts.get_field_word(0x09)
    }

    fn supported_sram_type(&self) -> Option<u16> {
        self.parts.get_field_word(0x0B)
    }

    fn current_sram_type(&self) -> Option<u16> {
        self.parts.get_field_word(0x0D)
    }

    fn cache_speed(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    fn error_correction_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x10)
    }

    fn system_cache_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    fn associativity(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    fn maximum_cache_size_2(&self) -> Option<u32> {
        self.parts.get_field_dword(0x13)
    }

    fn installed_cache_size_2(&self) -> Option<u32> {
        self.parts.get_field_dword(0x17)
    }
}

impl fmt::Debug for SMBiosCacheInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosCacheInformation>())
            .field("header", &self.parts.header)
            .field("socket_designation", &self.socket_designation())
            .field("cache_configuration", &self.cache_configuration())
            .field("maximum_cache_size", &self.maximum_cache_size())
            .field("installed_size", &self.installed_size())
            .field("supported_sram_type", &self.supported_sram_type())
            .field("current_sram_type", &self.current_sram_type())
            .field("cache_speed", &self.cache_speed())
            .field("error_correction_type", &self.error_correction_type())
            .field("system_cache_type", &self.system_cache_type())
            .field("associativity", &self.associativity())
            .field("maximum_cache_size_2", &self.maximum_cache_size_2())
            .field("installed_cache_size_2", &self.installed_cache_size_2())
            .finish()
    }
}
