use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;

/// # Management Device Threshold Data (Type 36)
///
/// The information in this structure defines threshold information for a component (probe or cooling-unit) contained within a Management Device
///
/// For each threshold field present in the structure:
/// - The threshold units (millivolts, milliamps, 1/10th degrees C, or RPMs) are as defined by the associated probe or cooling-unit component structure.
/// - If the value is unavailable, the field is set to 0x8000.
///
/// NOTE This structure type was added in version 2.3 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosManagementDeviceThresholdData<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementDeviceThresholdData<'a> {
    const STRUCT_TYPE: u8 = 36u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosManagementDeviceThresholdData<'a> {
    /// Lower non-critical threshold for this component
    pub fn lower_threshold_non_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x04)
    }

    /// Upper non-critical threshold for this component
    pub fn upper_threshold_non_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    /// Lower critical threshold for this component
    pub fn lower_threshold_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Upper critical threshold for this component
    pub fn upper_threshold_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Lower non-recoverable threshold for this component
    pub fn lower_threshold_non_recoverable(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Upper non-recoverable threshold for this component
    pub fn upper_threshold_non_recoverable(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }
}

impl fmt::Debug for SMBiosManagementDeviceThresholdData<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosManagementDeviceThresholdData<'_>>())
            .field("header", &self.parts.header)
            .field(
                "lower_threshold_non_critical",
                &self.lower_threshold_non_critical(),
            )
            .field(
                "upper_threshold_non_critical",
                &self.upper_threshold_non_critical(),
            )
            .field("lower_threshold_critical", &self.lower_threshold_critical())
            .field("upper_threshold_critical", &self.upper_threshold_critical())
            .field(
                "lower_threshold_non_recoverable",
                &self.lower_threshold_non_recoverable(),
            )
            .field(
                "upper_threshold_non_recoverable",
                &self.upper_threshold_non_recoverable(),
            )
            .finish()
    }
}

impl Serialize for SMBiosManagementDeviceThresholdData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosManagementDeviceThresholdData", 7)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field(
            "lower_threshold_non_critical",
            &self.lower_threshold_non_critical(),
        )?;
        state.serialize_field(
            "upper_threshold_non_critical",
            &self.upper_threshold_non_critical(),
        )?;
        state.serialize_field("lower_threshold_critical", &self.lower_threshold_critical())?;
        state.serialize_field("upper_threshold_critical", &self.upper_threshold_critical())?;
        state.serialize_field(
            "lower_threshold_non_recoverable",
            &self.lower_threshold_non_recoverable(),
        )?;
        state.serialize_field(
            "upper_threshold_non_recoverable",
            &self.upper_threshold_non_recoverable(),
        )?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type36 = vec![
            0x24, 0x10, 0x28, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00,
            0x06, 0x00, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type36);
        let test_struct = SMBiosManagementDeviceThresholdData::new(&parts);

        assert_eq!(test_struct.lower_threshold_non_critical(), Some(1));
        assert_eq!(test_struct.upper_threshold_non_critical(), Some(2));
        assert_eq!(test_struct.lower_threshold_critical(), Some(3));
        assert_eq!(test_struct.upper_threshold_critical(), Some(4));
        assert_eq!(test_struct.lower_threshold_non_recoverable(), Some(5));
        assert_eq!(test_struct.upper_threshold_non_recoverable(), Some(6));
    }
}
