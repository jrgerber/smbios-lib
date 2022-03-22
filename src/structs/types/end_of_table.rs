use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};

/// # End-of-Table (Type 127)
///
/// This structure type identifies the end of the structure table that might be earlier than the last byte within the buffer specified by the structure.
///
/// To ensure backward compatibility with management software written to previous versions of this
/// specification, a system implementation should use the end-of-table indicator in a manner similar to the
/// [super::SMBiosInactive] (Type 126) structure type; the structure table is still reported as a fixed-length, and the entire
/// length of the table is still indexable. If the end-of-table indicator is used in the last physical structure in a
/// table, the field’s length is encoded as 4.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosEndOfTable<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosEndOfTable<'a> {
    const STRUCT_TYPE: u8 = 127u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosEndOfTable<'a> {}

impl fmt::Debug for SMBiosEndOfTable<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosEndOfTable<'_>>())
            .field("header", &self.parts.header)
            .finish()
    }
}

impl Serialize for SMBiosEndOfTable<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosEndOfTable", 1)?;
        state.serialize_field("header", &self.parts.header)?;
        state.end()
    }
}
