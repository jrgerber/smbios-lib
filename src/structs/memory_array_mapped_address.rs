use super::*;

pub struct SMBiosMemoryArrayMappedAddress<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryArrayMappedAddress<'a> {
    const STRUCT_TYPE: u8 = 19u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryArrayMappedAddress<'a> {
    fn starting_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x4)
    }

    fn ending_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x8)
    }

    fn physical_memory_array_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xC)
    }

    fn partition_width(&self) -> Option<u8> {
        self.parts.get_field_byte(0xE)
    }

    fn extended_starting_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0xF)
    }

    fn extended_ending_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x17)
    }
}

impl fmt::Debug for SMBiosMemoryArrayMappedAddress<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryArrayMappedAddress>())
            .field("header", &self.parts.header)
            .field("starting_address", &self.starting_address())
            .field("ending_address", &self.ending_address())
            .field(
                "physical_memory_array_handle",
                &self.physical_memory_array_handle(),
            )
            .field("partition_width", &self.partition_width())
            .field(
                "extended_starting_address",
                &self.extended_starting_address(),
            )
            .field("extended_ending_address", &self.extended_ending_address())
            .finish()
    }
}
