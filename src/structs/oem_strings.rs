use super::*;

/// # OEM Strings (Type 11)
/// 
/// This structure contains free-form strings defined by the OEM. Examples of this are
/// part numbers for system reference documents, contact information for the manufacturer, etc.
/// 
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOemStrings<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOemStrings<'a> {
    const STRUCT_TYPE: u8 = 11u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOemStrings>())
        .field("header", &self.parts.header)
        .field("count", &self.count())
        .field("oem_strings", &self.oem_strings())
        .finish()
    }
}

