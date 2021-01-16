use super::*;

/// # Electrical Current Probe (Type 29)
///
/// This structure describes the attributes for an electrical current probe in the system. Each structure describes a single electrical current probe.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosElectricalCurrentProbe<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosElectricalCurrentProbe<'a> {
    const STRUCT_TYPE: u8 = 29u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosElectricalCurrentProbe<'a> {
    ///  A string that contains additional descriptive information about the probe or its location
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Probe’s physical location and status of the current monitored by this current probe
    pub fn location_and_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Maximum current level readable by this probe, in milliamps
    pub fn maximum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    /// Minimum temperature level readable by this probe, in milliamps
    pub fn minimum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Resolution for the probe’s reading, in tenths of milliamps
    pub fn resolution(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Tolerance for reading from this probe, in plus/minus milliamps
    pub fn tolerance(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Accuracy for reading from this probe, in plus/minus 1/100th of a percent
    pub fn accuracy(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }

    /// OEM- or BIOS vendor-specific information.
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    /// Nominal value for the probe’s reading in milliamps
    pub fn nominal_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }
}

impl fmt::Debug for SMBiosElectricalCurrentProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosElectricalCurrentProbe>())
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
