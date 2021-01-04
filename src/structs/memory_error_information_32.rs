use super::*;

pub struct SMBiosMemoryErrorInformation32<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryErrorInformation32<'a> {
    const STRUCT_TYPE: u8 = 18u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryErrorInformation32<'a> {
    fn error_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn error_granularity(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn error_operation(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn vendor_syndrome(&self) -> Option<u32> {
        self.parts.get_field_dword(0x07)
    }

    fn memory_array_error_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0B)
    }

    fn device_error_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0F)
    }

    fn error_resolution(&self) -> Option<u32> {
        self.parts.get_field_dword(0x13)
    }
}

impl fmt::Debug for SMBiosMemoryErrorInformation32<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryErrorInformation32>())
        .field("header", &self.parts.header)
        .field("error_type", &self.error_type())
        .field("error_granularity", &self.error_granularity())
        .field("error_operation", &self.error_operation())
        .field("vendor_syndrome", &self.vendor_syndrome())
        .field("memory_array_error_address", &self.memory_array_error_address())
        .field("device_error_address", &self.device_error_address())
        .field("error_resolution", &self.error_resolution())
        .finish()
    }
}

