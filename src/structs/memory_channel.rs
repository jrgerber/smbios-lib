use super::*;

/// # Memory Channel (Type 37)
///
/// The information in this structure provides the correlation between a Memory Channel and its associated [SMBiosMemoryDevice]s.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryChannel<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryChannel<'a> {
    const STRUCT_TYPE: u8 = 37u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryChannel<'a> {
    /// Type of memory associated with the channel
    pub fn channel_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Maximum load supported by the channel; the sum of all
    /// device loads cannot exceed this value
    pub fn maximum_channel_load(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Number of [SMBiosMemoryDevice]s (Type 11h) that are
    /// associated with this channel
    /// This value also defines the number of Load/Handle pairs
    /// that follow.
    pub fn memory_device_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    // TODO: this is an array

    // ///
    // pub fn memory_device_load(&self) -> Option<u8> {
    //     self.parts.get_field_byte(0x07)
    // }

    // ///
    // pub fn memory_device_handle(&self) -> Option<u16> {
    //     self.parts.get_field_word(0x08)
    // }
}

impl fmt::Debug for SMBiosMemoryChannel<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryChannel>())
            .field("header", &self.parts.header)
            .field("channel_type", &self.channel_type())
            .field("maximum_channel_load", &self.maximum_channel_load())
            .field("memory_device_count", &self.memory_device_count())
            // .field("memory_device_load", &self.memory_device_load())
            // .field("memory_device_handle", &self.memory_device_handle())
            .finish()
    }
}
