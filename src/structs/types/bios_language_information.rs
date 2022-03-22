use crate::{SMBiosStruct, Strings, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use core::ops::Deref;
use alloc::string::String;

/// # BIOS Language Information (Type 13)
///
/// The information in this structure defines the installable language attributes of the BIOS.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosBiosLanguageInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosBiosLanguageInformation<'a> {
    const STRUCT_TYPE: u8 = 13u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosBiosLanguageInformation<'a> {
    /// Number of languages available
    /// Each available language has a description
    /// string. This field contains the number of strings
    /// that follow the formatted area of the structure.
    pub fn number_of_installable_languages(&self) -> Option<u8> {
        self.parts.get_field_byte(0x4)
    }

    /// Bit field indicating the format of the languages.
    pub fn flags(&self) -> Option<BiosLanguageFlags> {
        self.parts
            .get_field_byte(0x5)
            .map(|raw| BiosLanguageFlags::from(raw))
    }

    /// The currently installed language.
    pub fn current_language(&self) -> Option<String> {
        self.parts.get_field_string(0x15)
    }

    /// Iterable collection of the installable languages.
    pub fn installable_langauges(&self) -> &Strings {
        &self.parts.strings
    }
}

impl fmt::Debug for SMBiosBiosLanguageInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosBiosLanguageInformation<'_>>())
            .field("header", &self.parts.header)
            .field(
                "number_of_installable_languages",
                &self.number_of_installable_languages(),
            )
            .field("flags", &self.flags())
            .field("current_language", &self.current_language())
            .field("installable_languages", &self.installable_langauges())
            .finish()
    }
}

impl Serialize for SMBiosBiosLanguageInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosBiosLanguageInformation", 5)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field(
            "number_of_installable_languages",
            &self.number_of_installable_languages(),
        )?;
        state.serialize_field("flags", &self.flags())?;
        state.serialize_field("current_language", &self.current_language())?;
        state.serialize_field("installable_languages", &self.installable_langauges())?;
        state.end()
    }
}

/// # Language Format
#[derive(Serialize, Debug)]
pub enum LanguageFormat {
    /// Language strings use the abbreviated format.
    ///
    /// Example: "frCA"
    Abbreviated,
    /// Language strings use the long format.
    ///
    /// Example: "fr|CA|iso8859-1"
    Long,
}

/// # BIOS Language Flags
#[derive(PartialEq, Eq)]
pub struct BiosLanguageFlags {
    /// Raw value
    pub raw: u8,
}

impl Deref for BiosLanguageFlags {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for BiosLanguageFlags {
    fn from(raw: u8) -> Self {
        BiosLanguageFlags { raw }
    }
}

impl BiosLanguageFlags {
    /// If set to 1, the Current Language strings use the abbreviated format. Otherwise, the strings use the long format.
    pub fn language_format(&self) -> LanguageFormat {
        if self.raw & 0x01 == 0x01 {
            LanguageFormat::Abbreviated
        } else {
            LanguageFormat::Long
        }
    }
}

impl fmt::Debug for BiosLanguageFlags {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<BiosLanguageFlags>())
            .field("raw", &self.raw)
            .field("language_format", &self.language_format())
            .finish()
    }
}

impl Serialize for BiosLanguageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BiosLanguageFlags", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("language_format", &self.language_format())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bios_language_information() {
        let bios_language_information_bytes = vec![
            0x0Du8, 0x16, 0x21, 0x00,
            // number_of_installable_languages: Some(3), flags: Some(0), current_language: Some("en|US|iso8859-1")
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, // "en|US|iso8859-1"
            0x65, 0x6E, 0x7C, 0x55, 0x53, 0x7C, 0x69, 0x73, 0x6F, 0x38, 0x38, 0x35, 0x39, 0x2D,
            0x31, 0x00, // "hr|HR|iso8859-2"
            0x68, 0x72, 0x7C, 0x48, 0x52, 0x7C, 0x69, 0x73, 0x6F, 0x38, 0x38, 0x35, 0x39, 0x2D,
            0x32, 0x00, // "ja|JP|unicode"
            0x6A, 0x61, 0x7C, 0x4A, 0x50, 0x7C, 0x75, 0x6E, 0x69, 0x63, 0x6F, 0x64, 0x65, 0x00,
            // end of structure
            0x00,
        ];

        let parts = UndefinedStruct::new(&bios_language_information_bytes);
        let bios_language_information = SMBiosBiosLanguageInformation::new(&parts);

        // header tests
        assert_eq!(*bios_language_information.parts().header.handle(), 0x0021);
        assert_eq!(bios_language_information.parts().header.length(), 0x16);

        // basic field tests
        assert_eq!(
            bios_language_information
                .current_language()
                .expect("current_language field exists"),
            "en|US|iso8859-1".to_string()
        );
        assert_eq!(
            bios_language_information
                .number_of_installable_languages()
                .expect("number_of_installable_languages field exists"),
            3
        );
        assert_eq!(
            bios_language_information.flags().unwrap(),
            BiosLanguageFlags::from(0)
        );

        // installable_languages tests
        let mut string_iterator = bios_language_information
            .installable_langauges()
            .into_iter();
        let first_string = string_iterator.next().expect("has a first string");
        assert_eq!(first_string, "en|US|iso8859-1".to_string());
        let second_string = string_iterator.next().expect("has a second string");
        assert_eq!(second_string, "hr|HR|iso8859-2".to_string());
        let third_string = string_iterator.next().expect("has a third string");
        assert_eq!(third_string, "ja|JP|unicode".to_string());
        assert!(string_iterator.next().is_none());

        // debug print test
        println!("bios_language_information: {:?}", bios_language_information);
    }
}
