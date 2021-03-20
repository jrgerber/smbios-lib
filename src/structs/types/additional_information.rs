use crate::core::{Handle, UndefinedStruct};
use crate::structs::SMBiosStruct;
use std::fmt;

/// # Additional Information Entry contained within [SMBiosAdditionalInformation]
pub struct AdditionalInformationEntry<'a> {
    additional_information: &'a SMBiosAdditionalInformation<'a>,
    entry_offset: usize,
}

impl<'a> AdditionalInformationEntry<'a> {
    fn new(
        additional_information: &'a SMBiosAdditionalInformation<'a>,
        entry_offset: usize,
    ) -> Self {
        Self {
            additional_information,
            entry_offset,
        }
    }

    /// Length of this Additional Information Entry instance; a minimum of 6
    pub fn entry_length(&self) -> Option<u8> {
        self.additional_information
            .parts()
            .get_field_byte(self.entry_offset)
    }

    /// Handle, or instance number, associated with the structure for which additional information is provided
    pub fn referenced_handle(&self) -> Option<Handle> {
        self.additional_information
            .parts()
            .get_field_handle(self.entry_offset + 1)
    }

    /// Offset of the field within the structure referenced by the
    /// _Referenced Handle_ for which additional information is provided
    pub fn referenced_offset(&self) -> Option<u8> {
        self.additional_information
            .parts()
            .get_field_byte(self.entry_offset + 3)
    }

    /// Number of the optional string to be associated with the field referenced by the _Referenced Offset_
    pub fn string(&self) -> Option<String> {
        self.additional_information
            .parts()
            .get_field_string(self.entry_offset + 4)
    }

    /// Enumerated value or updated field content that has not yet been
    /// approved for publication in this specification and therefore could
    /// not be used in the field referenced by _Referenced Offset_
    ///
    /// NOTE: This field is the same type and size as the field being referenced
    /// by this Additional Information Entry.
    pub fn value(&self) -> Option<&[u8]> {
        const VALUE_RELATIVE_OFFSET: usize = 5usize;
        let value_offset = self.entry_offset + VALUE_RELATIVE_OFFSET;

        match self.entry_length() {
            Some(entry_length) => {
                let value_size = entry_length as usize - VALUE_RELATIVE_OFFSET;
                self.additional_information
                    .parts()
                    .get_field_data(value_offset, value_offset + value_size)
            }
            None => None,
        }
    }
}

impl fmt::Debug for AdditionalInformationEntry<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosAdditionalInformation<'_>>())
            .field("entry_length", &self.entry_length())
            .field("referenced_handle", &self.referenced_handle())
            .field("referenced_offset", &self.referenced_offset())
            .field("string", &self.string())
            .field("value", &self.value())
            .finish()
    }
}

/// # Iterates over the [AdditionalInformationEntry] entries contained within [SMBiosAdditionalInformation]
pub struct AdditionalInformationEntryIterator<'a> {
    data: &'a SMBiosAdditionalInformation<'a>,
    current_index: usize,
    current_entry: u8,
    number_of_entries: u8,
}

impl<'a> AdditionalInformationEntryIterator<'a> {
    const ENTRIES_OFFSET: usize = 5usize;

    fn new(data: &'a SMBiosAdditionalInformation<'a>) -> Self {
        AdditionalInformationEntryIterator {
            data: data,
            current_index: Self::ENTRIES_OFFSET,
            current_entry: 0,
            number_of_entries: data.number_of_entries().unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::ENTRIES_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a AdditionalInformationEntryIterator<'a> {
    type Item = AdditionalInformationEntry<'a>;
    type IntoIter = AdditionalInformationEntryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AdditionalInformationEntryIterator {
            data: self.data,
            current_index: AdditionalInformationEntryIterator::ENTRIES_OFFSET,
            current_entry: 0,
            number_of_entries: self.data.number_of_entries().unwrap_or(0),
        }
    }
}

impl<'a> Iterator for AdditionalInformationEntryIterator<'a> {
    type Item = AdditionalInformationEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_entries {
            self.reset();
            return None;
        }

        match self.data.parts().get_field_byte(self.current_index) {
            Some(entry_length) => {
                // Length of 0 would result in an endless loop because we would never advance to the next entry
                if entry_length == 0 {
                    self.reset();
                    return None;
                }

                let next_index = self.current_index + entry_length as usize;
                match self
                    .data
                    .parts()
                    .get_field_data(self.current_index, next_index)
                {
                    Some(_entry_block) => {
                        let result = AdditionalInformationEntry::new(self.data, self.current_index);
                        self.current_index = next_index;
                        self.current_entry += 1;
                        Some(result)
                    }
                    None => {
                        self.reset();
                        None
                    }
                }
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for AdditionalInformationEntryIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

/// # Additional Information (Type 40)
///
/// This structure is intended to provide additional information for handling unspecified enumerated values
/// and interim field updates in another structure.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosAdditionalInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosAdditionalInformation<'a> {
    const STRUCT_TYPE: u8 = 40u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosAdditionalInformation<'a> {
    /// Number of [AdditionalInformationEntry] entries
    pub fn number_of_entries(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Iterates over the [AdditionalInformationEntry] entries
    pub fn entry_iterator(&'a self) -> AdditionalInformationEntryIterator<'a> {
        AdditionalInformationEntryIterator::new(self)
    }
}

impl fmt::Debug for SMBiosAdditionalInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosAdditionalInformation<'_>>())
            .field("header", &self.parts.header)
            .field("number_of_entries", &self.number_of_entries())
            .field("entry_iterator", &self.entry_iterator())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additional_information() {
        let additional_information_bytes = vec![
            0x28u8, 0x0B, 0x02, 0x01, // header (offsets 0-3)
            0x01, // 1 additional entry (offset 4)
            0x6, 0x04, 0x03, 0x22, 0x01, 0x67, // entry (offsets 5-0x0A)
            0x58, 0x00, // entry string "X" (offsets 0x0B-0x0C)
            0x00,
        ]; // end of structure (offset 0x0D)

        let parts = UndefinedStruct::new(&additional_information_bytes);
        let additional_information = SMBiosAdditionalInformation::new(&parts);

        assert_eq!(*additional_information.parts().header.handle(), 0x0102);
        assert_eq!(additional_information.parts().header.length(), 0x0B);
        assert_eq!(
            additional_information
                .number_of_entries()
                .expect("must be 1 entry"),
            1
        );

        let mut iterator = additional_information.entry_iterator();
        let first_entry = iterator.next().expect("must have a first entry");
        assert_eq!(
            first_entry
                .entry_length()
                .expect("must be entry length of 6"),
            6
        );
        assert_eq!(
            first_entry.string().expect("must be entry string of \"X\""),
            "X".to_string()
        );

        assert!(iterator.next().is_none());

        let additional_information_bytes = vec![
            0x28u8, 0x11, 0x02, 0x01, // header (offsets 0-3)
            0x02, // 2 additional entries (offset 4)
            0x6, 0x04, 0x03, 0x22, 0x00, 0x67, // entry (offsets 5-0x0A)
            0x6, 0x06, 0x05, 0x33, 0x00, 0x89, // entry (offsets 0x0B-0x10)
            0x00, // null string (offsets 0x0B-0x0C)
            0x00,
        ]; // end of structure (offset 0x0D)
        let parts = UndefinedStruct::new(&additional_information_bytes);
        let additional_information = SMBiosAdditionalInformation::new(&parts);

        let mut counter = 0;

        for _entry in additional_information.entry_iterator() {
            counter = counter + 1;
        }

        assert_eq!(counter, 2);
        assert_eq!(
            additional_information
                .number_of_entries()
                .expect("must be 2 entries"),
            counter
        );

        println!("additional_information: {:?}", additional_information);
    }
}
