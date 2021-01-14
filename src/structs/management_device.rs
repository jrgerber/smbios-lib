use super::*;

/// # Management Device (Type 34)
///
/// The information in this structure defines the attributes of a Management Device.
/// 
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
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
    /// Additional descriptive information about the device or its location
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Device's type
    pub fn device_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Device's address
    pub fn address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x06)
    }

    /// Type of addressing used to access the device
    pub fn address_type(&self) -> Option<u8> {
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
