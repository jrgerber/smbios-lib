use super::*;

/// # Cooling Device (Type 27)
/// 
/// This structure describes the attributes for a cooling device in the system. Each structure describes a single cooling device.
/// 
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosCoolingDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosCoolingDevice<'a> {
    const STRUCT_TYPE: u8 = 27u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosCoolingDevice<'a> {
    /// Handle, or instance number, of the temperature
    /// probe monitoring this cooling device.
    /// A value of 0xFFFF indicates that no probe is
    /// provided.
    pub fn temperature_probe_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x04)
    }

    /// Cooling device type and status.
    pub fn device_type_and_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Cooling unit group to which this cooling device is associated
    /// Having multiple cooling devices in the same
    /// cooling unit implies a redundant configuration. The
    /// value is 00h if the cooling device is not a member
    /// of a redundant cooling unit. Non-zero values imply
    /// redundancy and that at least one other cooling
    /// device will be enumerated with the same value
    pub fn cooling_unit_group(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    /// OEM or BIOS vendor-specific information.
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x08)
    }

    /// Nominal value for the cooling device’s rotational
    /// speed, in revolutions-per-minute (rpm)
    /// If the value is unknown or the cooling device is
    /// non-rotating, the field is set to 0x8000. This field is
    /// present in the structure only if the structure’s
    /// length is larger than 0Ch
    pub fn nominal_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Additional descriptive information about the cooling device or its location
    /// This field is present in the structure only if the
    /// structure’s length is 0Fh or larger.
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x0E)
    }
}

impl fmt::Debug for SMBiosCoolingDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosCoolingDevice>())
        .field("header", &self.parts.header)
        .field("temperature_probe_handle", &self.temperature_probe_handle())
        .field("device_type_and_status", &self.device_type_and_status())
        .field("cooling_unit_group", &self.cooling_unit_group())
        .field("oem_defined", &self.oem_defined())
        .field("nominal_speed", &self.nominal_speed())
        .field("description", &self.description())
        .finish()
    }
}

