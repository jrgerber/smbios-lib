use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::any;
use std::fmt;

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
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosInactive<'a> {
    const STRUCT_TYPE: u8 = 126u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosInactive<'a> {}

impl fmt::Debug for SMBiosInactive<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosInactive<'_>>())
            .field("header", &self.parts.header)
            .finish()
    }
}

impl Serialize for SMBiosInactive<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosInactive", 1)?;
        state.serialize_field("header", &self.parts.header)?;
        state.end()
    }
}
