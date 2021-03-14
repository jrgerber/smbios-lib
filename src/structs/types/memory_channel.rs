use crate::{Handle, SMBiosStruct, UndefinedStruct};
use std::fmt;
use std::ops::Deref;

/// # Memory Channel (Type 37)
///
/// The information in this structure provides the correlation between a Memory Channel and its associated [SMBiosMemoryDevice]s.
///
/// Each device presents one or more loads to the channel; the sum of all device loads cannot exceed the channel’s defined maximum.
///
/// NOTE This structure type was added in version 2.3 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryChannel<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryChannel<'a> {
    const STRUCT_TYPE: u8 = 37u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosMemoryChannel<'a> {
    /// Type of memory associated with the channel
    pub fn channel_type(&self) -> Option<MemoryChannelTypeData> {
        self.parts
            .get_field_byte(0x04)
            .map(|raw| MemoryChannelTypeData::from(raw))
    }

    /// Maximum load supported by the channel; the sum of all
    /// device loads cannot exceed this value
    pub fn maximum_channel_load(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Number of [SMBiosMemoryDevice]s (Type 11h) that are
    /// associated with this channel
    ///
    /// This value also defines the number of Load/Handle pairs
    /// that follow.
    pub fn memory_device_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Load/Handle pairs defining the [SMBiosMemoryDevice]s
    /// associated with this memory channel.
    pub fn load_handle_pairs_iterator(&self) -> LoadHandlePairIterator<'_> {
        LoadHandlePairIterator::new(self)
    }
}

impl fmt::Debug for SMBiosMemoryChannel<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryChannel<'_>>())
            .field("header", &self.parts.header)
            .field("channel_type", &self.channel_type())
            .field("maximum_channel_load", &self.maximum_channel_load())
            .field("memory_device_count", &self.memory_device_count())
            .field(
                "load_handle_pairs_iterator",
                &self.load_handle_pairs_iterator(),
            )
            .finish()
    }
}

/// # Memory Channel — Channel Type Data
pub struct MemoryChannelTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryChannelType] value
    pub value: MemoryChannelType,
}

impl fmt::Debug for MemoryChannelTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<MemoryChannelTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for MemoryChannelTypeData {
    type Target = MemoryChannelType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Channel — Channel Type
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryChannelType {
    /// Other,
    Other,
    /// Unknown,
    Unknown,
    /// RamBus,
    RamBus,
    /// SyncLink,
    SyncLink,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryChannelTypeData {
    fn from(raw: u8) -> Self {
        MemoryChannelTypeData {
            value: match raw {
                0x01 => MemoryChannelType::Other,
                0x02 => MemoryChannelType::Unknown,
                0x03 => MemoryChannelType::RamBus,
                0x04 => MemoryChannelType::SyncLink,
                _ => MemoryChannelType::None,
            },
            raw,
        }
    }
}

/// # Load/Handle Pair contained within [SMBiosMemoryChannel]
pub struct LoadHandlePair<'a> {
    memory_channel: &'a SMBiosMemoryChannel<'a>,
    entry_offset: usize,
}

impl<'a> LoadHandlePair<'a> {
    /// Size in bytes of a LoadHandlePair
    const SIZE: usize = 3usize;

    fn new(memory_channel: &'a SMBiosMemoryChannel<'a>, entry_offset: usize) -> Self {
        Self {
            memory_channel,
            entry_offset,
        }
    }

    /// Channel load provided by the [SMBiosMemoryDevice] associated with this channel
    pub fn load(&self) -> Option<u8> {
        self.memory_channel
            .parts()
            .get_field_byte(self.entry_offset)
    }

    /// Structure handle that identifies the [SMBiosMemoryDevice] associated with this channel
    pub fn handle(&self) -> Option<Handle> {
        self.memory_channel
            .parts()
            .get_field_handle(self.entry_offset + 1)
    }
}

impl fmt::Debug for LoadHandlePair<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<LoadHandlePair<'_>>())
            .field("load", &self.load())
            .field("handle", &self.handle())
            .finish()
    }
}

/// Iterates over the [LoadHandlePair] entries contained within [SMBiosMemoryChannel]
pub struct LoadHandlePairIterator<'a> {
    data: &'a SMBiosMemoryChannel<'a>,
    current_index: usize,
    current_entry: u8,
    number_of_entries: u8,
}

impl<'a> LoadHandlePairIterator<'a> {
    const LOAD_HANDLE_PAIRS_OFFSET: usize = 7usize;

    fn new(data: &'a SMBiosMemoryChannel<'a>) -> Self {
        LoadHandlePairIterator {
            data: data,
            current_index: Self::LOAD_HANDLE_PAIRS_OFFSET,
            current_entry: 0,
            number_of_entries: data.memory_device_count().unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::LOAD_HANDLE_PAIRS_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a LoadHandlePairIterator<'a> {
    type Item = LoadHandlePair<'a>;
    type IntoIter = LoadHandlePairIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LoadHandlePairIterator {
            data: self.data,
            current_index: LoadHandlePairIterator::LOAD_HANDLE_PAIRS_OFFSET,
            current_entry: 0,
            number_of_entries: self.data.memory_device_count().unwrap_or(0),
        }
    }
}

impl<'a> Iterator for LoadHandlePairIterator<'a> {
    type Item = LoadHandlePair<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_entries {
            self.reset();
            return None;
        }

        let next_index = self.current_index + LoadHandlePair::SIZE;
        match self
            .data
            .parts()
            .get_field_data(self.current_index, next_index)
        {
            Some(_entry_block) => {
                let result = LoadHandlePair::new(self.data, self.current_index);
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

impl<'a> fmt::Debug for LoadHandlePairIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type37 = vec![
            37u8, 0x0D, 0x3F, 0x00, 0x03, 0x30, 0x02, 0x01, 0x02, 0x00, 0x03, 0x04, 0x00, 0x00,
            0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type37);
        let test_struct = SMBiosMemoryChannel::new(&parts);

        assert_eq!(
            *test_struct.channel_type().unwrap(),
            MemoryChannelType::RamBus
        );
        assert_eq!(test_struct.maximum_channel_load(), Some(0x30));
        assert_eq!(test_struct.memory_device_count(), Some(2));

        let mut iterator = test_struct.load_handle_pairs_iterator().into_iter();
        let first = iterator.next().unwrap();
        assert_eq!(first.load(), Some(1));
        assert_eq!(*first.handle().unwrap(), 2);
        let second = iterator.next().unwrap();
        assert_eq!(second.load(), Some(3));
        assert_eq!(*second.handle().unwrap(), 4);
        assert!(iterator.next().is_none());
    }
}
