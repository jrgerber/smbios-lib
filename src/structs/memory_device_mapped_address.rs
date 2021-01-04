use super::*;

pub struct SMBiosMemoryDeviceMappedAddress<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryDeviceMappedAddress<'a> {
    const STRUCT_TYPE: u8 = 20u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryDeviceMappedAddress<'a> {
    fn starting_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x4)
    }

    fn ending_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x8)
    }

    fn memory_device_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xC)
    }

    fn memory_array_mapped_address_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xE)
    }

    fn partition_row_position(&self) -> Option<u8> {
        self.parts.get_field_byte(0x10)
    }

    fn interleave_position(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    fn interleaved_data_depth(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    fn extended_starting_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x13)
    }

    fn extended_ending_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x1B)
    }
}

impl fmt::Debug for SMBiosMemoryDeviceMappedAddress<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryDeviceMappedAddress>())
        .field("header", &self.parts.header)
        .field("starting_address", &self.starting_address())
        .field("ending_address", &self.ending_address())
        .field("memory_device_handle", &self.memory_device_handle())
        .field("memory_array_mapped_address_handle", &self.memory_array_mapped_address_handle())
        .field("partition_row_position", &self.partition_row_position())
        .field("interleave_position", &self.interleave_position())
        .field("interleaved_data_depth", &self.interleaved_data_depth())
        .field("extended_starting_address", &self.extended_starting_address())
        .field("extended_ending_address", &self.extended_ending_address())
        .finish()
    }
}

