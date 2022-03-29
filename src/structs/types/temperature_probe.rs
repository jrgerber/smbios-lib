use crate::core::{strings::*, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;

/// # Temperature Probe (Type 28)
///
/// This structure describes the attributes for a temperature probe in the system. Each structure describes a
/// single temperature probe.
///
/// NOTE This structure type was added in version 2.2 of this specification.
pub struct SMBiosTemperatureProbe<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosTemperatureProbe<'a> {
    const STRUCT_TYPE: u8 = 28u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosTemperatureProbe<'a> {
    /// Description
    ///
    /// additional descriptive information about the probe or its location
    pub fn description(&self) -> SMBiosString {
        self.parts.get_field_string(0x04)
    }

    /// Location and status
    ///
    /// Probe’s physical location and the status of the temperature
    /// monitored by this temperature probe
    pub fn location_and_status(&self) -> Option<TemperatureProbeLocationAndStatus> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| TemperatureProbeLocationAndStatus::from(raw))
    }

    /// Maximum value
    ///
    /// Maximum temperature readable by this probe, in 1/10th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn maximum_value(&self) -> Option<ProbeTemperature> {
        self.parts
            .get_field_word(0x06)
            .map(|raw| ProbeTemperature::from(raw))
    }

    /// Minimum value
    ///
    /// Minimum temperature readable by this probe, in 1/10th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn minimum_value(&self) -> Option<ProbeTemperature> {
        self.parts
            .get_field_word(0x08)
            .map(|raw| ProbeTemperature::from(raw))
    }

    /// Resolution
    ///
    /// Resolution for the probe’s reading, in 1/1000th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn resolution(&self) -> Option<TemperatureProbeResolution> {
        self.parts
            .get_field_word(0x0A)
            .map(|raw| TemperatureProbeResolution::from(raw))
    }

    /// Tolerance
    ///
    /// Tolerance for reading from this probe, in plus/minus 1/10th degrees C
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn tolerance(&self) -> Option<ProbeTemperature> {
        self.parts
            .get_field_word(0x0C)
            .map(|raw| ProbeTemperature::from(raw))
    }

    /// Accuracy
    ///
    /// Accuracy for reading from this probe, in plus/minus 1/100th of a percent
    ///
    /// If the value is unknown, the field is set to 0x8000.
    pub fn accuracy(&self) -> Option<TemperatureProbeAccuracy> {
        self.parts
            .get_field_word(0x0E)
            .map(|raw| TemperatureProbeAccuracy::from(raw))
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
    pub fn nominal_value(&self) -> Option<ProbeTemperature> {
        self.parts
            .get_field_word(0x14)
            .map(|raw| ProbeTemperature::from(raw))
    }
}

impl fmt::Debug for SMBiosTemperatureProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosTemperatureProbe<'_>>())
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

impl Serialize for SMBiosTemperatureProbe<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosTemperatureProbe", 10)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("location_and_status", &self.location_and_status())?;
        state.serialize_field("maximum_value", &self.maximum_value())?;
        state.serialize_field("minimum_value", &self.minimum_value())?;
        state.serialize_field("resolution", &self.resolution())?;
        state.serialize_field("tolerance", &self.tolerance())?;
        state.serialize_field("accuracy", &self.accuracy())?;
        state.serialize_field("oem_defined", &self.oem_defined())?;
        state.serialize_field("nominal_value", &self.nominal_value())?;
        state.end()
    }
}

/// # Temperature Probe Location and Status
#[derive(PartialEq, Eq)]
pub struct TemperatureProbeLocationAndStatus {
    /// Raw value
    pub raw: u8,
}

impl From<u8> for TemperatureProbeLocationAndStatus {
    fn from(raw: u8) -> Self {
        TemperatureProbeLocationAndStatus { raw }
    }
}

impl TemperatureProbeLocationAndStatus {
    /// Temperature Probe Location
    pub fn location(&self) -> TemperatureProbeLocation {
        TemperatureProbeLocation::from(self.raw)
    }

    /// Temperature Probe Status
    pub fn status(&self) -> TemperatureProbeStatus {
        TemperatureProbeStatus::from(self.raw)
    }
}

impl fmt::Debug for TemperatureProbeLocationAndStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<TemperatureProbeLocationAndStatus>())
            .field("raw", &self.raw)
            .field("location", &self.location())
            .field("status", &self.status())
            .finish()
    }
}

impl Serialize for TemperatureProbeLocationAndStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TemperatureProbeLocationAndStatus", 3)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("location", &self.location())?;
        state.serialize_field("status", &self.status())?;
        state.end()
    }
}

/// # Temperature Probe Status
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum TemperatureProbeStatus {
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

impl From<u8> for TemperatureProbeStatus {
    fn from(raw: u8) -> Self {
        match raw & 0b1110_0000 {
            0b0000_0000 => TemperatureProbeStatus::None,
            0b0010_0000 => TemperatureProbeStatus::Other,
            0b0100_0000 => TemperatureProbeStatus::Unknown,
            0b0110_0000 => TemperatureProbeStatus::OK,
            0b1000_0000 => TemperatureProbeStatus::NonCritical,
            0b1010_0000 => TemperatureProbeStatus::Critical,
            0b1100_0000 => TemperatureProbeStatus::NonRecoverable,
            0b1110_0000 => TemperatureProbeStatus::None,
            _ => panic!("impossible value"),
        }
    }
}

/// # Temperature Probe Location
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum TemperatureProbeLocation {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Processor
    Processor,
    /// Disk
    Disk,
    /// Peripheral Bay
    PeripheralBay,
    /// System Management Moduel
    SystemManagementModule,
    /// Motherboard
    Motherboard,
    /// Memory Module
    MemoryModule,
    /// Processor Module
    ProcessorModule,
    /// Power Unit
    PowerUnit,
    /// Add-in Card
    AddInCard,
    /// Front Panel Board
    FrontPanelBoard,
    /// Back Panel Board
    BackPanelBoard,
    /// Power System Board
    PowerSystemBoard,
    /// Drive Back Plane
    DriveBackPlane,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for TemperatureProbeLocation {
    fn from(raw: u8) -> Self {
        match raw & 0b0001_1111 {
            0b0000_0001 => TemperatureProbeLocation::Other,
            0b0000_0010 => TemperatureProbeLocation::Unknown,
            0b0000_0011 => TemperatureProbeLocation::Processor,
            0b0000_0100 => TemperatureProbeLocation::Disk,
            0b0000_0101 => TemperatureProbeLocation::PeripheralBay,
            0b0000_0110 => TemperatureProbeLocation::SystemManagementModule,
            0b0000_0111 => TemperatureProbeLocation::Motherboard,
            0b0000_1000 => TemperatureProbeLocation::MemoryModule,
            0b0000_1001 => TemperatureProbeLocation::ProcessorModule,
            0b0000_1010 => TemperatureProbeLocation::PowerUnit,
            0b0000_1011 => TemperatureProbeLocation::AddInCard,
            0b0000_1100 => TemperatureProbeLocation::FrontPanelBoard,
            0b0000_1101 => TemperatureProbeLocation::BackPanelBoard,
            0b0000_1110 => TemperatureProbeLocation::PowerSystemBoard,
            0b0000_1111 => TemperatureProbeLocation::DriveBackPlane,
            _ => TemperatureProbeLocation::None,
        }
    }
}

/// # Probe Temperature
#[derive(Serialize, Debug)]
pub enum ProbeTemperature {
    /// Temperature in 1/10 degrees C
    OneTenthDegreesC(u16),
    /// Temperature is unknown
    Unknown,
}

impl From<u16> for ProbeTemperature {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => ProbeTemperature::Unknown,
            _ => ProbeTemperature::OneTenthDegreesC(raw),
        }
    }
}

/// # Temperature Probe Resolution
#[derive(Serialize, Debug)]
pub enum TemperatureProbeResolution {
    /// Resolution for the probe's reading in 1/1000 degrees C
    OneOneThousandthDegreesC(u16),
    /// Resolution is unknown
    Unknown,
}

impl From<u16> for TemperatureProbeResolution {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => TemperatureProbeResolution::Unknown,
            _ => TemperatureProbeResolution::OneOneThousandthDegreesC(raw),
        }
    }
}

/// # Temperature Probe Accuracy
#[derive(Serialize, Debug)]
pub enum TemperatureProbeAccuracy {
    /// Accuracy for the probe's reading in 1/100 degrees C
    OneOneHundredthDegreesC(u16),
    /// Accuracy is unknown
    Unknown,
}

impl From<u16> for TemperatureProbeAccuracy {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => TemperatureProbeAccuracy::Unknown,
            _ => TemperatureProbeAccuracy::OneOneHundredthDegreesC(raw),
        }
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

        let parts = UndefinedStruct::new(&struct_type28);
        let test_struct = SMBiosTemperatureProbe::new(&parts);

        assert_eq!(test_struct.description().to_string(), "LM78A".to_string());
        assert_eq!(
            test_struct.location_and_status(),
            Some(TemperatureProbeLocationAndStatus::from(103))
        );
        match test_struct.maximum_value().unwrap() {
            ProbeTemperature::OneTenthDegreesC(_) => panic!("expected unknown"),
            ProbeTemperature::Unknown => (),
        }
        match test_struct.minimum_value().unwrap() {
            ProbeTemperature::OneTenthDegreesC(_) => panic!("expected unknown"),
            ProbeTemperature::Unknown => (),
        }
        match test_struct.resolution().unwrap() {
            TemperatureProbeResolution::OneOneThousandthDegreesC(_) => panic!("expected unknown"),
            TemperatureProbeResolution::Unknown => (),
        }
        match test_struct.tolerance().unwrap() {
            ProbeTemperature::OneTenthDegreesC(_) => panic!("expected unknown"),
            ProbeTemperature::Unknown => (),
        }
        match test_struct.accuracy().unwrap() {
            TemperatureProbeAccuracy::OneOneHundredthDegreesC(_) => panic!("expected unknown"),
            TemperatureProbeAccuracy::Unknown => (),
        }
        assert_eq!(test_struct.oem_defined(), Some(0));
        match test_struct.nominal_value().unwrap() {
            ProbeTemperature::OneTenthDegreesC(_) => panic!("expected unknown"),
            ProbeTemperature::Unknown => (),
        }
    }
}
