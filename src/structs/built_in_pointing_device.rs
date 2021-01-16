use super::*;

/// # Built-in Pointing Device (Type 21)
///
/// This structure describes the attributes of the built-in pointing device for the system.
/// Details are provided in Table 87.
/// The presence of this structure does not imply that the built-in pointing device is active
/// for the system’s use.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosBuiltInPointingDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosBuiltInPointingDevice<'a> {
    const STRUCT_TYPE: u8 = 21u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosBuiltInPointingDevice<'a> {
    /// Type of pointing device.
    pub fn device_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Interface type for the pointing device.
    pub fn interface(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Number of buttons on the pointing device.
    /// If the device has 3 buttons, the field value is 3.
    pub fn number_of_buttons(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }
}

impl fmt::Debug for SMBiosBuiltInPointingDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosBuiltInPointingDevice>())
            .field("header", &self.parts.header)
            .field("device_type", &self.device_type())
            .field("interface", &self.interface())
            .field("number_of_buttons", &self.number_of_buttons())
            .finish()
    }
}
