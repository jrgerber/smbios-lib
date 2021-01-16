use super::*;

/// # Onboard Devices Extended Information (Type 41)
///
/// The information in this structure defines the attributes of devices that are onboard (soldered onto) a
/// system element, usually the baseboard.
///
/// In general, an entry in this table implies that the BIOS has some level of control over the enablement of
/// the associated device for use by the system.
///
/// NOTE: This structure replaces Onboard Device Information (Type 10) starting with version 2.6 of this specification.
/// BIOS providers can choose to implement both types to allow existing SMBIOS browsers to properly display
/// the system’s onboard devices information.
///  
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOnboardDevicesExtendedInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOnboardDevicesExtendedInformation<'a> {
    const STRUCT_TYPE: u8 = 41u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOnboardDevicesExtendedInformation<'a> {
    /// The onboard device reference designation
    pub fn reference_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x4)
    }

    /// Device type bit field and enum
    pub fn device_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x5)
    }

    /// Device type instance
    pub fn device_type_instance(&self) -> Option<u8> {
        self.parts.get_field_byte(0x6)
    }

    /// Segment group number
    pub fn segment_group_number(&self) -> Option<u16> {
        self.parts.get_field_word(0x7)
    }

    /// Bus number
    pub fn bus_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0x9)
    }

    /// Device/Function number
    pub fn device_function_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0xA)
    }
}

impl fmt::Debug for SMBiosOnboardDevicesExtendedInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOnboardDevicesExtendedInformation>())
            .field("header", &self.parts.header)
            .field("reference_designation", &self.reference_designation())
            .field("device_type", &self.device_type())
            .field("device_type_instance", &self.device_type_instance())
            .field("segment_group_number", &self.segment_group_number())
            .field("bus_number", &self.bus_number())
            .field("device_function_number", &self.device_function_number())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type41 = vec![
            0x29, 0x0B, 0x3B, 0x00, 0x01, 0x85, 0x01, 0x00, 0x00, 0x00, 0xFE, 0x69, 0x32, 0x31,
            0x39, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type41.as_slice());
        let test_struct = SMBiosOnboardDevicesExtendedInformation::new(&parts);

        assert_eq!(
            test_struct.reference_designation(),
            Some("i219".to_string())
        );
        assert_eq!(test_struct.device_type(), Some(133));
        assert_eq!(test_struct.device_type_instance(), Some(1));
        assert_eq!(test_struct.segment_group_number(), Some(0));
        assert_eq!(test_struct.bus_number(), Some(0));
        assert_eq!(test_struct.device_function_number(), Some(254));
    }
}
