use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use alloc::string::String;

/// # Electrical Current Probe (Type 29)
///
/// This structure describes the attributes for an electrical current probe in the system. Each structure describes a single electrical current probe.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosElectricalCurrentProbe<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosElectricalCurrentProbe<'a> {
    const STRUCT_TYPE: u8 = 29u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosElectricalCurrentProbe<'a> {
    ///  A string that contains additional descriptive information about the probe or its location
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Probe’s physical location and status of the current monitored by this current probe
    pub fn location_and_status(&self) -> Option<CurrentProbeLocationAndStatus> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| CurrentProbeLocationAndStatus::from(raw))
    }

    /// Maximum current level readable by this probe, in milliamps
    pub fn maximum_value(&self) -> Option<ProbeAmperage> {
        self.parts
            .get_field_word(0x06)
            .map(|raw| ProbeAmperage::from(raw))
    }

    /// Minimum current level readable by this probe, in milliamps
    pub fn minimum_value(&self) -> Option<ProbeAmperage> {
        self.parts
            .get_field_word(0x08)
            .map(|raw| ProbeAmperage::from(raw))
    }

    /// Resolution for the probe’s reading, in tenths of milliamps
    pub fn resolution(&self) -> Option<CurrentProbeResolution> {
        self.parts
            .get_field_word(0x0A)
            .map(|raw| CurrentProbeResolution::from(raw))
    }

    /// Tolerance for reading from this probe, in plus/minus milliamps
    pub fn tolerance(&self) -> Option<ProbeAmperage> {
        self.parts
            .get_field_word(0x0C)
            .map(|raw| ProbeAmperage::from(raw))
    }

    /// Accuracy for reading from this probe, in plus/minus 1/100th of a percent
    pub fn accuracy(&self) -> Option<CurrentProbeAccuracy> {
        self.parts
            .get_field_word(0x0E)
            .map(|raw| CurrentProbeAccuracy::from(raw))
    }

    /// OEM- or BIOS vendor-specific information.
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    /// Nominal value for the probe’s reading in milliamps
    pub fn nominal_value(&self) -> Option<ProbeAmperage> {
        self.parts
            .get_field_word(0x14)
            .map(|raw| ProbeAmperage::from(raw))
    }
}

impl fmt::Debug for SMBiosElectricalCurrentProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosElectricalCurrentProbe<'_>>())
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

impl Serialize for SMBiosElectricalCurrentProbe<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosElectricalCurrentProbe", 10)?;
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

/// # Electrical Current Probe Location and Status
#[derive(PartialEq, Eq)]
pub struct CurrentProbeLocationAndStatus {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The [CurrentProbeStatus]
    pub status: CurrentProbeStatus,
    /// The [CurrentProbeLocation]
    pub location: CurrentProbeLocation,
}

impl fmt::Debug for CurrentProbeLocationAndStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<CurrentProbeLocationAndStatus>())
            .field("raw", &self.raw)
            .field("status", &self.status)
            .field("location", &self.location)
            .finish()
    }
}

impl Serialize for CurrentProbeLocationAndStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CurrentProbeLocationAndStatus", 3)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("location", &self.location)?;
        state.end()
    }
}

/// # Electrical Current Probe Status
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum CurrentProbeStatus {
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

/// # Electrical Current Probe Location
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum CurrentProbeLocation {
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
    /// System Management Module
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

impl From<u8> for CurrentProbeLocationAndStatus {
    fn from(raw: u8) -> Self {
        CurrentProbeLocationAndStatus {
            status: match raw & 0b111_00000 {
                0b001_00000 => CurrentProbeStatus::Other,
                0b010_00000 => CurrentProbeStatus::Unknown,
                0b011_00000 => CurrentProbeStatus::OK,
                0b100_00000 => CurrentProbeStatus::NonCritical,
                0b101_00000 => CurrentProbeStatus::Critical,
                0b110_00000 => CurrentProbeStatus::NonRecoverable,
                _ => CurrentProbeStatus::None,
            },
            location: match raw & 0b000_11111 {
                0b000_00001 => CurrentProbeLocation::Other,
                0b000_00010 => CurrentProbeLocation::Unknown,
                0b000_00011 => CurrentProbeLocation::Processor,
                0b000_00100 => CurrentProbeLocation::Disk,
                0b000_00101 => CurrentProbeLocation::PeripheralBay,
                0b000_00110 => CurrentProbeLocation::SystemManagementModule,
                0b000_00111 => CurrentProbeLocation::Motherboard,
                0b000_01000 => CurrentProbeLocation::MemoryModule,
                0b000_01001 => CurrentProbeLocation::ProcessorModule,
                0b000_01010 => CurrentProbeLocation::PowerUnit,
                0b000_01011 => CurrentProbeLocation::AddInCard,
                _ => CurrentProbeLocation::None,
            },
            raw,
        }
    }
}

/// # Probe Amperage
#[derive(Serialize, Debug)]
pub enum ProbeAmperage {
    /// Amperage in milliamps
    Milliamps(u16),
    /// Amperage is unknown
    Unknown,
}

impl From<u16> for ProbeAmperage {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => ProbeAmperage::Unknown,
            _ => ProbeAmperage::Milliamps(raw),
        }
    }
}

/// # Current Probe Resolution
#[derive(Serialize, Debug)]
pub enum CurrentProbeResolution {
    /// Resolution for the probe's reading in tenths of milliamps
    TenthsOfMilliamps(u16),
    /// Resolution is unknown
    Unknown,
}

impl From<u16> for CurrentProbeResolution {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => CurrentProbeResolution::Unknown,
            _ => CurrentProbeResolution::TenthsOfMilliamps(raw),
        }
    }
}

/// # Current Probe Accuracy
#[derive(Serialize, Debug)]
pub enum CurrentProbeAccuracy {
    /// Accuracy for the probe's reading in 1/100th of a percent
    OneOneHundredthPercent(u16),
    /// Accuracy is unknown
    Unknown,
}

impl From<u16> for CurrentProbeAccuracy {
    fn from(raw: u16) -> Self {
        match raw {
            0x8000 => CurrentProbeAccuracy::Unknown,
            _ => CurrentProbeAccuracy::OneOneHundredthPercent(raw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type29 = vec![
            0x1D, 0x16, 0x33, 0x00, 0x01, 0x67, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
            0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, b'A', b'B', b'C', 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type29);
        let test_struct = SMBiosElectricalCurrentProbe::new(&parts);

        assert_eq!(test_struct.description(), Some("ABC".to_string()));
        let location_and_status = test_struct.location_and_status().unwrap();
        assert_eq!(location_and_status.status, CurrentProbeStatus::OK);
        assert_eq!(
            location_and_status.location,
            CurrentProbeLocation::Motherboard
        );
        assert_eq!(
            test_struct.location_and_status(),
            Some(CurrentProbeLocationAndStatus::from(103))
        );
        match test_struct.maximum_value().unwrap() {
            ProbeAmperage::Milliamps(_) => panic!("expected unknown"),
            ProbeAmperage::Unknown => (),
        }
        match test_struct.minimum_value().unwrap() {
            ProbeAmperage::Milliamps(_) => panic!("expected unknown"),
            ProbeAmperage::Unknown => (),
        }
        match test_struct.resolution().unwrap() {
            CurrentProbeResolution::TenthsOfMilliamps(_) => panic!("expected unknown"),
            CurrentProbeResolution::Unknown => (),
        }
        match test_struct.tolerance().unwrap() {
            ProbeAmperage::Milliamps(_) => panic!("expected unknown"),
            ProbeAmperage::Unknown => (),
        }
        match test_struct.accuracy().unwrap() {
            CurrentProbeAccuracy::OneOneHundredthPercent(_) => panic!("expected unknown"),
            CurrentProbeAccuracy::Unknown => (),
        }
        assert_eq!(test_struct.oem_defined(), Some(0));
        match test_struct.nominal_value().unwrap() {
            ProbeAmperage::Milliamps(_) => panic!("expected unknown"),
            ProbeAmperage::Unknown => (),
        }
    }
}
