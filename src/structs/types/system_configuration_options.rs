use crate::{SMBiosStruct, Strings, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;

/// # System Configuration Options (Type 12)
///
/// This structure contains information required to configure the baseboard’s Jumpers and Switches.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemConfigurationOptions<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemConfigurationOptions<'a> {
    const STRUCT_TYPE: u8 = 12u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosSystemConfigurationOptions<'a> {
    /// Number of strings
    pub fn count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Iterable collection of OEM strings
    ///
    /// EXAMPLES:
    /// "JP2: 1-2 Cache Size is 256K, 2-3 Cache Size is 512K"
    /// "SW1-1: Close to Disable On Board Video"
    pub fn configuration_strings(&self) -> &Strings {
        &self.parts.strings
    }
}

impl fmt::Debug for SMBiosSystemConfigurationOptions<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemConfigurationOptions<'_>>())
            .field("header", &self.parts.header)
            .field("count", &self.count())
            .field("configuration_strings", &self.configuration_strings())
            .finish()
    }
}

impl Serialize for SMBiosSystemConfigurationOptions<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosSystemConfigurationOptions", 3)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("count", &self.count())?;
        state.serialize_field("configuration_strings", &self.configuration_strings())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type12 = vec![
            0x0C, 0x05, 0x23, 0x00, 0x01, b's', b'c', b'r', b'e', b'+', b'+', 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type12);
        let test_struct = SMBiosSystemConfigurationOptions::new(&parts);

        assert_eq!(test_struct.count(), Some(1));
        assert_eq!(
            test_struct
                .configuration_strings()
                .into_iter()
                .next()
                .unwrap()
                .unwrap(),
            "scre++".to_string()
        );
    }
}
