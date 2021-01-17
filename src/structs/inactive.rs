use super::*;

/// # Inactive (Type 126)
///
/// This structure definition supports a system implementation where the SMBIOS structure-table is a
/// superset of all supported system attributes and provides a standard mechanism for the system BIOS to
/// signal that a structure is currently inactive and should not be interpreted by the upper-level software.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosInactive<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosInactive<'a> {
    const STRUCT_TYPE: u8 = 126u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosInactive<'a> {}

impl fmt::Debug for SMBiosInactive<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosInactive>())
            .field("header", &self.parts.header)
            .finish()
    }
}
