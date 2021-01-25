use crate::*;

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
    pub fn device_type_and_status(&self) -> Option<CoolingDeviceTypeAndStatus> {
        self.parts
            .get_field_byte(0x06)
            .and_then(|raw| Some(CoolingDeviceTypeAndStatus::from(raw)))
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

/// # Cooling Device Type and Status
#[derive(PartialEq, Eq)]
pub struct CoolingDeviceTypeAndStatus {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The [CoolingDeviceStatus]
    pub device_status: CoolingDeviceStatus,
    /// The [CoolingDeviceType]
    pub device_type: CoolingDeviceType,
}

impl fmt::Debug for CoolingDeviceTypeAndStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<CoolingDeviceTypeAndStatus>())
            .field("raw", &self.raw)
            .field("device_status", &self.device_status)
            .field("device_type", &self.device_type)
            .finish()
    }
}

/// # Cooling Device Status
#[derive(Debug, PartialEq, Eq)]
pub enum CoolingDeviceStatus {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// OK
    OK,
    /// Non-critical
    NonCritical,
    /// Critical
    Critical,
    /// Non-recoverable
    NonRecoverable,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Cooling Device Type
#[derive(Debug, PartialEq, Eq)]
pub enum CoolingDeviceType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Fan
    Fan,
    /// Centrifugal Blower
    CentrifugalBlower,
    /// Chip Fan
    ChipFan,
    /// Cabinet Fan
    CabinetFan,
    /// Power Supply Fan
    PowerSupplyFan,
    /// Heat Pipe
    HeatPipe,
    /// Integrated Refrigeration
    IntegratedRefrigeration,
    /// Active Cooling
    ActiveCooling,
    /// Passive Cooling
    PassiveCooling,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for CoolingDeviceTypeAndStatus {
    fn from(raw: u8) -> Self {
        CoolingDeviceTypeAndStatus {
            device_status: match raw & 0b111_00000 {
                0b001_00000 => CoolingDeviceStatus::Other,
                0b010_00000 => CoolingDeviceStatus::Unknown,
                0b011_00000 => CoolingDeviceStatus::OK,
                0b100_00000 => CoolingDeviceStatus::NonCritical,
                0b101_00000 => CoolingDeviceStatus::Critical,
                0b110_00000 => CoolingDeviceStatus::NonRecoverable,
                _ => CoolingDeviceStatus::None,
            },
            device_type: match raw & 0b000_11111 {
                0b000_00001 => CoolingDeviceType::Other,
                0b000_00010 => CoolingDeviceType::Unknown,
                0b000_00011 => CoolingDeviceType::Fan,
                0b000_00100 => CoolingDeviceType::CentrifugalBlower,
                0b000_00101 => CoolingDeviceType::ChipFan,
                0b000_00110 => CoolingDeviceType::CabinetFan,
                0b000_00111 => CoolingDeviceType::PowerSupplyFan,
                0b000_01000 => CoolingDeviceType::HeatPipe,
                0b000_01001 => CoolingDeviceType::IntegratedRefrigeration,
                0b000_10000 => CoolingDeviceType::ActiveCooling,
                0b000_10001 => CoolingDeviceType::PassiveCooling,
                _ => CoolingDeviceType::None,
            },
            raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type27 = vec![
            0x1B, 0x0F, 0x2D, 0x00, 0x2A, 0x00, 0x67, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
            0x01, 0x43, 0x6F, 0x6F, 0x6C, 0x69, 0x6E, 0x67, 0x20, 0x44, 0x65, 0x76, 0x20, 0x31,
            0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type27.as_slice());
        let test_struct = SMBiosCoolingDevice::new(&parts);

        //assert_eq!(test_struct.temperature_probe_handle(), Some(Handle(42)));

        let device_type_and_status = test_struct.device_type_and_status().unwrap();
        assert_eq!(
            device_type_and_status.device_status,
            CoolingDeviceStatus::OK
        );
        assert_eq!(
            device_type_and_status.device_type,
            CoolingDeviceType::PowerSupplyFan
        );
        assert_eq!(test_struct.cooling_unit_group(), Some(1));
        assert_eq!(test_struct.oem_defined(), Some(0));
        assert_eq!(test_struct.nominal_speed(), Some(32768));
        assert_eq!(test_struct.description(), Some("Cooling Dev 1".to_string()));
    }
}
