use super::*;

pub struct SMBiosMemoryModuleInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryModuleInformation<'a> {
    const STRUCT_TYPE: u8 = 6u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryModuleInformation<'a> {
    fn socket_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn bank_connections(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn current_speed(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn current_memory_type(&self) -> Option<u16> {
        self.parts.get_field_word(0x07)
    }

    fn installed_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    fn enabled_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0A)
    }

    fn error_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0B)
    }
}

impl fmt::Debug for SMBiosMemoryModuleInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryModuleInformation>())
        .field("header", &self.parts.header)
        .field("socket_designation", &self.socket_designation())
        .field("bank_connections", &self.bank_connections())
        .field("current_speed", &self.current_speed())
        .field("current_memory_type", &self.current_memory_type())
        .field("installed_size", &self.installed_size())
        .field("enabled_size", &self.enabled_size())
        .field("error_status", &self.error_status())
        .finish()
    }
}

