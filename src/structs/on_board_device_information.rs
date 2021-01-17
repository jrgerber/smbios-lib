use super::*;

/// # On Board Devices Information (Type 10, Obsolete)
///
///  The information in this structure defines the attributes of devices that are onboard
/// (soldered onto) a system element, usually the baseboard. In general, an entry in this table implies that the
/// BIOS has some level of control over the enabling of the associated device for use by the system.
///
/// NOTE This structure is obsolete starting with version 2.6 of this specification; the [SMBiosOnboardDevicesExtendedInformation]
/// (Type 41) structure should be used instead. BIOS providers can choose to implement
/// both types to allow existing SMBIOS browsers to properly display the system’s onboard devices information.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOnBoardDeviceInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOnBoardDeviceInformation<'a> {
    const STRUCT_TYPE: u8 = 10u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOnBoardDeviceInformation<'a> {
    // TODO: This is an array of structures to be implemented

    // fn device_type(&self) -> Option<u8> {
    //     self.parts.get_field_byte(0x4)
    // }

    // fn device_description(&self) -> Option<u8> {
    //     self.parts.get_field_byte(0x5)
    // }

    // fn minimum_ending_offset(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x6)
    // }
}

impl fmt::Debug for SMBiosOnBoardDeviceInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOnBoardDeviceInformation>())
            .field("header", &self.parts.header)
            // .field("device_type", &self.device_type())
            // .field("device_description", &self.device_description())
            // .field("minimum_ending_offset", &self.minimum_ending_offset())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type10 = vec![
            0x0A, 0x06, 0x21, 0x00, 0x83, 0x01, 0x20, 0x20, 0x20, 0x54, 0x6F, 0x20, 0x42, 0x65,
            0x20, 0x46, 0x69, 0x6C, 0x6C, 0x65, 0x64, 0x20, 0x42, 0x79, 0x20, 0x4F, 0x2E, 0x45,
            0x2E, 0x4D, 0x2E, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type10.as_slice());
        let test_struct = SMBiosOnBoardDeviceInformation::new(&parts);
    }
}
