use super::*;

pub struct SMBiosIpmiDeviceInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosIpmiDeviceInformation<'a> {
    const STRUCT_TYPE: u8 = 38u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosIpmiDeviceInformation<'a> {
    fn interface_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn ipmi_specification_revision(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn i2c_target_address(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn nvstorage_device_address(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    fn base_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x08)
    }

    fn base_address_modifier(&self) -> Option<u8> {
        self.parts.get_field_byte(0x10)
    }

    fn interrupt_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }
}

impl fmt::Debug for SMBiosIpmiDeviceInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosIpmiDeviceInformation>())
        .field("header", &self.parts.header)
        .field("interface_type", &self.interface_type())
        .field("ipmi_specification_revision", &self.ipmi_specification_revision())
        .field("i2c_target_address", &self.i2c_target_address())
        .field("nvstorage_device_address", &self.nvstorage_device_address())
        .field("base_address", &self.base_address())
        .field("base_address_modifier", &self.base_address_modifier())
        .field("interrupt_number", &self.interrupt_number())
        .finish()
    }
}

