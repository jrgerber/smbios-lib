use super::*;

/// # Temperature Probe (Type 28)
///
/// This structure describes the attributes for a temperature probe in the system. Each structure describes a
/// single temperature probe.
///
/// NOTE This structure type was added in version 2.2 of this specification.
pub struct SMBiosTemperatureProbe<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosTemperatureProbe<'a> {
    const STRUCT_TYPE: u8 = 28u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosTemperatureProbe<'a> {
    /// Description
    ///
    /// additional descriptive information about the probe or its location
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Location and status
    ///
    /// Probe’s physical location and the status of the temperature
    /// monitored by this temperature probe
    pub fn location_and_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Maximum value
    ///
    /// Maximum temperature readable by this probe, in 1/10th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn maximum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    /// Minimum value
    ///
    /// Minimum temperature readable by this probe, in 1/10th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn minimum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Resolution
    ///
    /// Resolution for the probe’s reading, in 1/1000th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn resolution(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Tolerance
    ///
    /// Tolerance for reading from this probe, in plus/minus 1/10th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn tolerance(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Accuracy
    ///
    /// Accuracy for reading from this probe, in plus/minus 1/100th of a percent
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn accuracy(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }

    /// OEM defined
    ///
    /// OEM- or BIOS vendor-specific information
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    /// Nominal value for the probe’s reading in 1/10th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000. This field is
    /// present in the structure only if the structure’s Length is larger
    /// than 14h.
    pub fn nominal_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }
}

impl fmt::Debug for SMBiosTemperatureProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosTemperatureProbe>())
            .field("header", &self.parts.header)
            .field("description", &self.description())
            .field("location_and_status", &self.location_and_status())
            .field("maximum_value", &self.maximum_value())
            .field("minimum_value", &self.minimum_value())
            .field("resolution", &self.resolution())
            .field("tolerance", &self.tolerance())
            .field("accuracy", &self.accuracy())
            .field("oem_defined", &self.oem_defined())
            .field("nominal_value", &self.nominal_value())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type28 = vec![
            0x1C, 0x16, 0x2A, 0x00, 0x01, 0x67, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
            0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x4C, 0x4D, 0x37, 0x38, 0x41, 0x00,
            0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type28.as_slice());
        let test_struct = SMBiosTemperatureProbe::new(&parts);

        assert_eq!(test_struct.description(), Some("LM78A".to_string()));
        assert_eq!(test_struct.location_and_status(), Some(103));
        assert_eq!(test_struct.maximum_value(), Some(32768));
        assert_eq!(test_struct.minimum_value(), Some(32768));
        assert_eq!(test_struct.resolution(), Some(32768));
        assert_eq!(test_struct.tolerance(), Some(32768));
        assert_eq!(test_struct.accuracy(), Some(32768));
        assert_eq!(test_struct.oem_defined(), Some(0));
        assert_eq!(test_struct.nominal_value(), Some(32768));
    }
}
