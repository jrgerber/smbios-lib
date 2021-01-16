use super::*;

/// # Hardware Security (Type 24)
///
/// This structure describes the system-wide hardware security settings.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosHardwareSecurity<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosHardwareSecurity<'a> {
    const STRUCT_TYPE: u8 = 24u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosHardwareSecurity<'a> {
    /// Bit field that identifies the password and reset status for the system
    pub fn hardware_security_settings(&self) -> Option<u8> {
        self.parts.get_field_byte(0x4)
    }
}

impl fmt::Debug for SMBiosHardwareSecurity<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosHardwareSecurity>())
            .field("header", &self.parts.header)
            .field(
                "hardware_security_settings",
                &self.hardware_security_settings(),
            )
            .finish()
    }
}
