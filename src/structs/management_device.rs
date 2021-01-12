use super::*;

pub struct SMBiosManagementDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementDevice<'a> {
    const STRUCT_TYPE: u8 = 34u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosManagementDevice<'a> {
    fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn device_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x06)
    }

    fn address_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0A)
    }
}

impl fmt::Debug for SMBiosManagementDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosManagementDevice>())
        .field("header", &self.parts.header)
        .field("description", &self.description())
        .field("device_type", &self.device_type())
        .field("address", &self.address())
        .field("address_type", &self.address_type())
        .finish()
    }
}

