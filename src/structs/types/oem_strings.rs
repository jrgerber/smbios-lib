use crate::*;

/// # OEM Strings (Type 11)
///
/// This structure contains free-form strings defined by the OEM. Examples of this are
/// part numbers for system reference documents, contact information for the manufacturer, etc.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOemStrings<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosOemStrings<'a> {
    const STRUCT_TYPE: u8 = 11u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosOemStrings<'a> {
    /// Number of strings
    pub fn count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Iterable collection of OEM strings
    pub fn oem_strings(&self) -> &Strings {
        &self.parts.strings
    }
}

impl fmt::Debug for SMBiosOemStrings<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOemStrings<'_>>())
            .field("header", &self.parts.header)
            .field("count", &self.count())
            .field("oem_strings", &self.oem_strings())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type6 = vec![
            0x0B, 0x05, 0x04, 0x00, 0x03, 0x41, 0x42, 0x53, 0x20, 0x37, 0x30, 0x2F, 0x37, 0x31,
            0x20, 0x36, 0x30, 0x20, 0x36, 0x31, 0x20, 0x36, 0x32, 0x20, 0x36, 0x33, 0x3B, 0x00,
            0x46, 0x42, 0x59, 0x54, 0x45, 0x23, 0x32, 0x55, 0x33, 0x45, 0x33, 0x58, 0x34, 0x37,
            0x36, 0x4A, 0x36, 0x53, 0x36, 0x62, 0x37, 0x42, 0x37, 0x48, 0x37, 0x4D, 0x37, 0x51,
            0x37, 0x54, 0x37, 0x57, 0x37, 0x61, 0x37, 0x6A, 0x37, 0x6D, 0x61, 0x33, 0x61, 0x70,
            0x61, 0x71, 0x61, 0x75, 0x62, 0x37, 0x2E, 0x51, 0x33, 0x3B, 0x00, 0x42, 0x55, 0x49,
            0x4C, 0x44, 0x49, 0x44, 0x23, 0x31, 0x33, 0x57, 0x57, 0x43, 0x44, 0x43, 0x38, 0x36,
            0x30, 0x31, 0x23, 0x53, 0x41, 0x42, 0x41, 0x23, 0x44, 0x41, 0x42, 0x41, 0x3B, 0x00,
            0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type6);
        let test_struct = SMBiosOemStrings::new(&parts);

        assert_eq!(test_struct.count(), Some(0x03));

        let mut iter = test_struct.oem_strings().into_iter();
        assert_eq!(iter.next(), Some("ABS 70/71 60 61 62 63;".to_string()));
        assert_eq!(
            iter.next(),
            Some("FBYTE#2U3E3X476J6S6b7B7H7M7Q7T7W7a7j7ma3apaqaub7.Q3;".to_string())
        );
        assert_eq!(
            iter.next(),
            Some("BUILDID#13WWCDC8601#SABA#DABA;".to_string())
        );
    }
}
