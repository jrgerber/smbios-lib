use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;

/// #  Voltage Probe (Type 26)
///
/// This describes the attributes for a voltage probe in the system. Each structure describes a single voltage
/// probe.
///
/// NOTE This structure type was added in version 2.2 of this specification.
pub struct SMBiosVoltageProbe<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosVoltageProbe<'a> {
    const STRUCT_TYPE: u8 = 26u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
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
    pub fn location_and_status(&self) -> Option<VoltageProbeLocationAndStatus> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| VoltageProbeLocationAndStatus::from(raw))
    }

    /// Maximum value
    ///
    /// Maximum voltage level readable by this probe, in
    /// millivolts
    pub fn maximum_value(&self) -> Option<ProbeVoltage> {
        self.parts
            .get_field_word(0x06)
            .map(|raw| ProbeVoltage::from(raw))
    }

    /// Minimum value
    ///
    /// Minimum voltage level readable by this probe, in millivolts
    pub fn minimum_value(&self) -> Option<ProbeVoltage> {
        self.parts
            .get_field_word(0x08)
            .map(|raw| ProbeVoltage::from(raw))
    }

    /// Resolution
    ///
    /// Resolution for the probe’s reading, in tenths of millivolts
    pub fn resolution(&self) -> Option<VoltageProbeResolution> {
        self.parts
            .get_field_word(0x0A)
            .map(|raw| VoltageProbeResolution::from(raw))
    }

    /// Tolerance
    ///
    /// Tolerance for reading from this probe, in plus/minus
    /// millivolts
    pub fn tolerance(&self) -> Option<ProbeVoltage> {
        self.parts
            .get_field_word(0x0C)
            .map(|raw| ProbeVoltage::from(raw))
    }

    /// Accuracy
    ///
    /// Accuracy for reading from this probe, in plus/minus
    /// 1/100th of a percent
    pub fn accuracy(&self) -> Option<VoltageProbeAccuracy> {
        self.parts
            .get_field_word(0x0E)
            .map(|raw| VoltageProbeAccuracy::from(raw))
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
    /// This field is present in the structure only if the structure's
    /// length is larger than 14h.
    pub fn nominal_value(&self) -> Option<ProbeVoltage> {
        self.parts
            .get_field_word(0x14)
            .map(|raw| ProbeVoltage::from(raw))
    }
}

impl fmt::Debug for SMBiosVoltageProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosVoltageProbe<'_>>())
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

impl Serialize for SMBiosVoltageProbe<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosVoltageProbe", 10)?;
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

/// # Voltage Probe Location and Status
#[derive(PartialEq, Eq)]
pub struct VoltageProbeLocationAndStatus {
    /// Raw value
    pub raw: u8,
}

impl From<u8> for VoltageProbeLocationAndStatus {
    fn from(raw: u8) -> Self {
        VoltageProbeLocationAndStatus { raw }
    }
}

impl VoltageProbeLocationAndStatus {
    /// Voltage Probe Location
    pub fn location(&self) -> VoltageProbeLocation {
        VoltageProbeLocation::from(self.raw)
    }

    /// Voltage Probe Status
    pub fn status(&self) -> VoltageProbeStatus {
        VoltageProbeStatus::from(self.raw)
    }
}

impl fmt::Debug for VoltageProbeLocationAndStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<VoltageProbeLocationAndStatus>())
            .field("raw", &self.raw)
            .field("location", &self.location())
            .field("status", &self.status())
            .finish()
    }
}

impl Serialize for VoltageProbeLocationAndStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("VoltageProbeLocationAndStatus", 3)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("location", &self.location())?;
        state.serialize_field("status", &self.status())?;
        state.end()
    }
}

/// # Voltage Probe Status
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum VoltageProbeStatus {
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

impl From<u8> for VoltageProbeStatus {
    fn from(raw: u8) -> Self {
        match raw & 0b1110_0000 {
            0b0000_0000 => VoltageProbeStatus::None,
            0b0010_0000 => VoltageProbeStatus::Other,
            0b0100_0000 => VoltageProbeStatus::Unknown,
            0b0110_0000 => VoltageProbeStatus::OK,
            0b1000_0000 => VoltageProbeStatus::NonCritical,
            0b1010_0000 => VoltageProbeStatus::Critical,
            0b1100_0000 => VoltageProbeStatus::NonRecoverable,
            0b1110_0000 => VoltageProbeStatus::None,
            _ => panic!("impossible value"),
        }
    }
}

/// # Voltage Probe Location
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum VoltageProbeLocation {
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
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for VoltageProbeLocation {
    fn from(raw: u8) -> Self {
        match raw & 0b0001_1111 {
            0b0000_0001 => VoltageProbeLocation::Other,
            0b0000_0010 => VoltageProbeLocation::Unknown,
            0b0000_0011 => VoltageProbeLocation::Processor,
            0b0000_0100 => VoltageProbeLocation::Disk,
            0b0000_0101 => VoltageProbeLocation::PeripheralBay,
            0b0000_0110 => VoltageProbeLocation::SystemManagementModule,
            0b0000_0111 => VoltageProbeLocation::Motherboard,
            0b0000_1000 => VoltageProbeLocation::MemoryModule,
            0b0000_1001 => VoltageProbeLocation::ProcessorModule,
            0b0000_1010 => VoltageProbeLocation::PowerUnit,
            0b0000_1011 => VoltageProbeLocation::AddInCard,
            _ => VoltageProbeLocation::None,
        }
    }
}

/// # Probe Voltage
#[derive(Serialize, Debug)]
pub enum ProbeVoltage {
    /// Voltage in millivolts
    Millivolts(u16),
    /// Voltage is unknown
    Unknown,
}

impl From<u16> for ProbeVoltage {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => ProbeVoltage::Unknown,
            _ => ProbeVoltage::Millivolts(raw),
        }
    }
}

/// # Voltage Probe Resolution
#[derive(Serialize, Debug)]
pub enum VoltageProbeResolution {
    /// Resolution for the probe's reading in tenths of millivolts
    TenthsOfMillivolts(u16),
    /// Resolution is unknown
    Unknown,
}

impl From<u16> for VoltageProbeResolution {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => VoltageProbeResolution::Unknown,
            _ => VoltageProbeResolution::TenthsOfMillivolts(raw),
        }
    }
}

/// # Voltage Probe Accuracy
#[derive(Serialize, Debug)]
pub enum VoltageProbeAccuracy {
    /// Accuracy for the probe's reading in 1/100th of a percent
    OneOneHundredthPercent(u16),
    /// Accuracy is unknown
    Unknown,
}

impl From<u16> for VoltageProbeAccuracy {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => VoltageProbeAccuracy::Unknown,
            _ => VoltageProbeAccuracy::OneOneHundredthPercent(raw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type26 = vec![
            26, 0x16, 0x2A, 0x00, 0x01, 0x67, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x4C, 0x4D, 0x37, 0x38, 0x41, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type26);
        let test_struct = SMBiosVoltageProbe::new(&parts);

        assert_eq!(test_struct.description(), Some("LM78A".to_string()));
        assert_eq!(
            test_struct.location_and_status(),
            Some(VoltageProbeLocationAndStatus::from(103))
        );
        match test_struct.maximum_value().unwrap() {
            ProbeVoltage::Millivolts(_) => panic!("expected unknown"),
            ProbeVoltage::Unknown => (),
        }
        match test_struct.minimum_value().unwrap() {
            ProbeVoltage::Millivolts(_) => panic!("expected unknown"),
            ProbeVoltage::Unknown => (),
        }
        match test_struct.resolution().unwrap() {
            VoltageProbeResolution::TenthsOfMillivolts(_) => panic!("expected unknown"),
            VoltageProbeResolution::Unknown => (),
        }
        match test_struct.tolerance().unwrap() {
            ProbeVoltage::Millivolts(_) => panic!("expected unknown"),
            ProbeVoltage::Unknown => (),
        }
        match test_struct.accuracy().unwrap() {
            VoltageProbeAccuracy::OneOneHundredthPercent(_) => panic!("expected unknown"),
            VoltageProbeAccuracy::Unknown => (),
        }
        assert_eq!(test_struct.oem_defined(), Some(0));
        match test_struct.nominal_value().unwrap() {
            ProbeVoltage::Millivolts(_) => panic!("expected unknown"),
            ProbeVoltage::Unknown => (),
        }
    }
}
