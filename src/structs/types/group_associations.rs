use crate::core::{strings::*, Handle, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeSeq, ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};

/// # Group Associations (Type 14)
///
/// The Group Associations structure is provided for OEMs who want to specify the arrangement or hierarchy
/// of certain components (including other Group Associations) within the system. For example, you can use
/// the Group Associations structure to indicate that two CPUs share a common external cache system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosGroupAssociations<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosGroupAssociations<'a> {
    const STRUCT_TYPE: u8 = 14u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosGroupAssociations<'a> {
    /// A string describing the group
    pub fn group_name(&self) -> SMBiosString {
        self.parts.get_field_string(0x4)
    }

    /// Number of [GroupAssociationItem] entries
    pub fn number_of_items(&self) -> Option<usize> {
        let length = self.parts.header.length() as usize;

        if length < GroupAssociationItemIterator::ITEMS_OFFSET {
            return None;
        }

        let byte_count = length - GroupAssociationItemIterator::ITEMS_OFFSET;

        if byte_count % GroupAssociationItem::SIZE != 0 {
            return None;
        }

        Some(byte_count / GroupAssociationItem::SIZE)
    }

    /// Iterates over the [GroupAssociationItem] entries
    pub fn item_iterator(&'a self) -> GroupAssociationItemIterator<'a> {
        GroupAssociationItemIterator::new(self)
    }
}

impl fmt::Debug for SMBiosGroupAssociations<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosGroupAssociations<'_>>())
            .field("header", &self.parts.header)
            .field("group_name", &self.group_name())
            .field("number_of_items", &self.number_of_items())
            .field("item_iterator", &self.item_iterator())
            .finish()
    }
}

impl Serialize for SMBiosGroupAssociations<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosGroupAssociations", 4)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("group_name", &self.group_name())?;
        state.serialize_field("number_of_items", &self.number_of_items())?;
        state.serialize_field("item_iterator", &self.item_iterator())?;
        state.end()
    }
}

/// # Group Association Item contained within [SMBiosGroupAssociations]
pub struct GroupAssociationItem<'a> {
    group_associations: &'a SMBiosGroupAssociations<'a>,
    entry_offset: usize,
}

impl<'a> GroupAssociationItem<'a> {
    /// Size in bytes of a GroupAssociationItem
    const SIZE: usize = 3usize;

    fn new(group_associations: &'a SMBiosGroupAssociations<'a>, entry_offset: usize) -> Self {
        Self {
            group_associations,
            entry_offset,
        }
    }

    /// Item Type
    ///
    /// Item (Structure) Type of this member
    pub fn struct_type(&self) -> Option<u8> {
        self.group_associations
            .parts()
            .get_field_byte(self.entry_offset)
    }

    /// Item Handle
    ///
    /// Handle corresponding to this structure
    pub fn item_handle(&self) -> Option<Handle> {
        self.group_associations
            .parts()
            .get_field_handle(self.entry_offset + 1)
    }
}

impl fmt::Debug for GroupAssociationItem<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<GroupAssociationItem<'_>>())
            .field("struct_type", &self.struct_type())
            .field("item_handle", &self.item_handle())
            .finish()
    }
}

impl Serialize for GroupAssociationItem<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GroupAssociationItem", 2)?;
        state.serialize_field("struct_type", &self.struct_type())?;
        state.serialize_field("item_handle", &self.item_handle())?;
        state.end()
    }
}

/// Iterates over the [GroupAssociationItem] entries contained within [SMBiosGroupAssociations]
pub struct GroupAssociationItemIterator<'a> {
    data: &'a SMBiosGroupAssociations<'a>,
    current_index: usize,
    current_entry: usize,
    number_of_entries: usize,
}

impl<'a> GroupAssociationItemIterator<'a> {
    const ITEMS_OFFSET: usize = 5usize;

    fn new(data: &'a SMBiosGroupAssociations<'a>) -> Self {
        GroupAssociationItemIterator {
            data: data,
            current_index: Self::ITEMS_OFFSET,
            current_entry: 0,
            number_of_entries: data.number_of_items().unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::ITEMS_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a GroupAssociationItemIterator<'a> {
    type Item = GroupAssociationItem<'a>;
    type IntoIter = GroupAssociationItemIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GroupAssociationItemIterator {
            data: self.data,
            current_index: GroupAssociationItemIterator::ITEMS_OFFSET,
            current_entry: 0,
            number_of_entries: self.data.number_of_items().unwrap_or(0),
        }
    }
}

impl<'a> Iterator for GroupAssociationItemIterator<'a> {
    type Item = GroupAssociationItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_entries {
            self.reset();
            return None;
        }

        let next_index = self.current_index + GroupAssociationItem::SIZE;
        match self
            .data
            .parts()
            .get_field_data(self.current_index, next_index)
        {
            Some(_entry_block) => {
                let result = GroupAssociationItem::new(self.data, self.current_index);
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
}

impl<'a> fmt::Debug for GroupAssociationItemIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

impl<'a> Serialize for GroupAssociationItemIterator<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let items: Vec<GroupAssociationItem<'_>> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(items.len()))?;
        for e in items {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type14 = vec![
            0x0E, 0x08, 0x5F, 0x00, 0x01, 0xDD, 0x5B, 0x00, 0x46, 0x69, 0x72, 0x6D, 0x77, 0x61,
            0x72, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x49, 0x6E, 0x66,
            0x6F, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type14);
        let test_struct = SMBiosGroupAssociations::new(&parts);

        println!("{:?}", test_struct);

        assert_eq!(
            test_struct.group_name().to_string(),
            "Firmware Version Info".to_string()
        );
        let mut iterator = test_struct.item_iterator().into_iter();
        let first_item = iterator.next().unwrap();
        assert_eq!(first_item.struct_type(), Some(221));
        assert_eq!(*first_item.item_handle().unwrap(), 91);
    }
}
