use super::*;

/// #  Voltage Probe (Type 26)
/// 
/// This describes the attributes for a voltage probe in the system. Each structure describes a single voltage
/// probe.
/// 
/// NOTE This structure type was added in version 2.2 of this specification.
pub struct SMBiosVoltageProbe<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosVoltageProbe<'a> {
    const STRUCT_TYPE: u8 = 26u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosVoltageProbe<'a> {
    /// Description
    /// 
    /// Additional descriptive information about the probe or its location
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Location and status bit-field
    /// 
    /// Probe’s physical location and status of the voltage
    /// monitored by this voltage probe
    pub fn location_and_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Maximum value
    /// 
    /// Maximum voltage level readable by this probe, in
    /// millivolts
    /// 
    /// If the value is unknown, the field is set to 0x8000.
    pub fn maximum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    /// Minimum value
    /// 
    /// Minimum voltage level readable by this probe, in millivolts
    /// 
    /// If the value is unknown, the field is set to 0x8000.
    pub fn minimum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Resolution
    /// 
    /// Resolution for the probe’s reading, in tenths of millivolts
    /// 
    /// If the value is unknown, the field is set to 0x8000.
    pub fn resolution(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Tolerance
    /// 
    /// Tolerance for reading from this probe, in plus/minus
    /// millivolts
    /// 
    /// If the value is unknown, the field is set to 0x8000.
    pub fn tolerance(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Accuracy
    /// 
    /// Accuracy for reading from this probe, in plus/minus
    /// 1/100th of a percent
    /// 
    /// If the value is unknown, the field is set to 0x8000.
    pub fn accuracy(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }

    /// OEM defined
    /// 
    /// OEM- or BIOS vendor-specific information.
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    /// Nominal value
    /// 
    /// Nominal value for the probe’s reading in millivolts
    /// 
    /// If the value is unknown, the field is set to 0x8000. This
    /// field is present in the structure only if the structure’s
    /// length is larger than 14h.
    pub fn nominal_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }
}

impl fmt::Debug for SMBiosVoltageProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosVoltageProbe>())
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
