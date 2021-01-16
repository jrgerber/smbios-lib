use super::*;

/// # Out-of-Band Remote Access (Type 30)
///
/// This structure describes the attributes and policy settings of a hardware facility that may be used to gain
/// remote access to a hardware system when the operating system is not available due to power-down
/// status, hardware failures, or boot failures.
///
/// NOTE: This structure type was added in version 2.2 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOutOfBandRemoteAccess<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOutOfBandRemoteAccess<'a> {
    const STRUCT_TYPE: u8 = 30u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOutOfBandRemoteAccess<'a> {
    ///  The manufacturer of the out-of-band access facility
    pub fn manufacturer_name(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Current remote-access connections (bit field)
    pub fn connections(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }
}

impl fmt::Debug for SMBiosOutOfBandRemoteAccess<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOutOfBandRemoteAccess>())
            .field("header", &self.parts.header)
            .field("manufacturer_name", &self.manufacturer_name())
            .field("connections", &self.connections())
            .finish()
    }
}
