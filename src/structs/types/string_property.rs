use crate::core::Handle;
use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

/// # String Property (Type 46)
///
/// This structure defines a string property for another structure. This allows adding string properties that are
/// common to several structures without having to modify the definitions of these structures. Multiple type 46
/// structures can add string properties to the same parent structure.
///
/// NOTE: This structure type was added in version 3.5 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.5.0 (DSP0134)
/// Document Date: 2021-09-15
pub struct SMBiosStringProperty<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosStringProperty<'a> {
    const STRUCT_TYPE: u8 = 46u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosStringProperty<'a> {
    /// String Property Id
    pub fn string_property_id(&self) -> Option<StringPropertyIdData> {
        self.parts
            .get_field_word(0x04)
            .map(|raw| StringPropertyIdData::from(raw))
    }

    /// String Property Value
    pub fn string_property_value(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// Parent Handle
    ///
    /// Handle corresponding to the structure this string property applies to
    pub fn parent_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x07)
    }
}

impl fmt::Debug for SMBiosStringProperty<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosStringProperty<'_>>())
            .field("header", &self.parts.header)
            .field("string_property_id", &self.string_property_id())
            .field("string_property_value", &self.string_property_value())
            .field("parent_handle", &self.parent_handle())
            .finish()
    }
}

impl Serialize for SMBiosStringProperty<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosStringProperty", 3)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("string_property_id", &self.string_property_id())?;
        state.serialize_field("string_property_value", &self.string_property_value())?;
        state.serialize_field("parent_handle", &self.parent_handle())?;
        state.end()
    }
}

/// # String Property Id Data of [SMBiosStringProperty].
pub struct StringPropertyIdData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u16,
    /// The contained [StringPropertyId] value
    pub value: StringPropertyId,
}

impl fmt::Debug for StringPropertyIdData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<StringPropertyIdData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for StringPropertyIdData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("StringPropertyIdData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for StringPropertyIdData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            StringPropertyId::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for StringPropertyIdData {
    type Target = StringPropertyId;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u16> for StringPropertyIdData {
    fn from(raw: u16) -> Self {
        StringPropertyIdData {
            value: match raw {
                0x0001 => StringPropertyId::UefiDevicePath,
                _ => match raw & 0b1000_0000_0000_0000 {
                    // >= 32768 ?
                    0b1000_0000_0000_0000 => match raw & 0b1100_0000_0000_0000 {
                        // >= 49152 ?
                        0b1100_0000_0000_0000 => StringPropertyId::OemSpecific, // 49152-65535
                        _ => StringPropertyId::VendorSpecific,                  // 32768-49151
                    },
                    _ => StringPropertyId::None,
                },
            },
            raw,
        }
    }
}

/// # String Property Id of [SMBiosStringProperty]
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum StringPropertyId {
    /// UEFI Device Path
    ///
    /// String representation of a UEFI device path, as converted by
    /// EFI_DEVICE_PATH_TO_TEXT_PROTOCOL. ConvertDevicePathToText() and then converted to UTF-8
    UefiDevicePath,
    /// Vendor Specific
    VendorSpecific,
    /// OEM Specific
    OemSpecific,
    /// A value unknown to this standard, check the raw value
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type46 = vec![
            // struct_type(46), length(0x09), handle(0x10)
            0x2E, 0x09, 0x10, 0x00,
            // string_property_id: (0x0001 - StringPropertyId::UefiDevicePath), string_property_value(1), parent_handle(0x0008)
            0x01, 0x00, 0x01, 0x08, 0x00, //string_property_value: "Abcd"
            b'A', b'b', b'c', b'd', 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type46);
        let test_struct = SMBiosStringProperty::new(&parts);

        assert_eq!(
            test_struct.string_property_id().unwrap().value,
            StringPropertyId::UefiDevicePath
        );

        assert_eq!(test_struct.string_property_value().unwrap(), "Abcd");

        assert_eq!(*test_struct.parent_handle().unwrap(), 8u16);
    }
}
