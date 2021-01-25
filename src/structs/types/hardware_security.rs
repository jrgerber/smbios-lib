use crate::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type24 = vec![0x18, 0x05, 0x24, 0x00, 0x16, 0x00, 0x00];

        let parts = SMBiosStructParts::new(struct_type24.as_slice());
        let test_struct = SMBiosHardwareSecurity::new(&parts);

        assert_eq!(test_struct.hardware_security_settings(), Some(22));
    }
}
