use super::*;

/// # Memory Device Mapped Address (Type 20)
///
/// This structure maps memory address space usually to a device-level granularity.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
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
    /// Physical address, in kilobytes, of a range of
    /// memory mapped to the referenced [SMBiosMemoryDevice]
    /// When the field value is FFFF FFFFh the actual
    /// address is stored in the Extended Starting
    /// Address field. When this field contains a valid
    /// address, Ending Address must also contain a
    /// valid address. When this field contains FFFF
    /// FFFFh, Ending Address must also contain FFFF
    /// FFFFh.
    pub fn starting_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x4)
    }

    /// Physical ending address of the last kilobyte of a
    /// range of addresses mapped to the referenced
    /// [SMBiosMemoryDevice]
    /// When the field value is FFFF FFFFh the actual
    /// address is stored in the Extended Ending Address
    /// field. When this field contains a valid address,
    /// Starting Address must also contain a valid
    /// address.
    pub fn ending_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x8)
    }

    /// Handle, or instance number, associated with the
    /// [SMBiosMemoryDevice] structure to which this address
    /// range is mapped
    /// Multiple address ranges can be mapped to a
    /// single [SMBiosMemoryDevice]
    pub fn memory_device_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xC)
    }

    /// Handle, or instance number, associated with the
    /// Memory Array Mapped Address structure to which
    /// this device address range is mapped
    /// Multiple address ranges can be mapped to a
    /// single [SMBiosMemoryArrayMappedAddress].
    pub fn memory_array_mapped_address_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xE)
    }

    /// Position of the referenced [SMBiosMemoryDevice] in a row
    /// of the address partition
    /// For example, if two 8-bit devices form a 16-bit row,
    /// this field’s value is either 1 or 2.
    /// The value 0 is reserved. If the position is
    /// unknown, the field contains FFh.
    pub fn partition_row_position(&self) -> Option<u8> {
        self.parts.get_field_byte(0x10)
    }

    /// Position of the referenced [SMBiosMemoryDevice] in an
    /// interleave
    /// The value 0 indicates non-interleaved, 1 indicates
    /// first interleave position, 2 the second interleave
    /// position, and so on. If the position is unknown, the
    /// field contains FFh.
    /// EXAMPLES: In a 2:1 interleave, the value 1 indicates
    /// the device in the ”even” position. In a 4:1 interleave, the
    /// value 1 indicates the first of four possible positions.
    pub fn interleave_position(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    /// Maximum number of consecutive rows from the
    /// referenced [SMBiosMemoryDevice] that are accessed in a
    /// single interleaved transfer
    /// If the device is not part of an interleave, the field
    /// contains 0; if the interleave configuration is
    /// unknown, the value is FFh.
    /// EXAMPLES: If a device transfers two rows each time it
    /// is read, its Interleaved Data Depth is set to 2. If that
    /// device is 2:1 interleaved and in Interleave Position 1, the
    /// rows mapped to that device are 1, 2, 5, 6, 9, 10, etc.
    pub fn interleaved_data_depth(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    /// Physical address, in bytes, of a range of memory
    /// mapped to the referenced [SMBiosMemoryDevice]
    /// This field is valid when Starting Address contains
    /// the value FFFF FFFFh. If Starting Address
    /// contains a value other than FFFF FFFFh, this field
    /// contains zeros. When this field contains a valid
    /// address, Extended Ending Address must also
    /// contain a valid address.
    pub fn extended_starting_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x13)
    }

    /// Physical ending address, in bytes, of the last of a
    /// range of addresses mapped to the referenced
    /// [SMBiosMemoryDevice]
    /// This field is valid when both Starting Address and
    /// Ending Address contain the value FFFF FFFFh. If
    /// Ending Address contains a value other than FFFF
    /// FFFFh, this field contains zeros. When this field
    /// contains a valid address, Extended Starting
    /// Address must also contain a valid address
    pub fn extended_ending_address(&self) -> Option<u64> {
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
            .field(
                "memory_array_mapped_address_handle",
                &self.memory_array_mapped_address_handle(),
            )
            .field("partition_row_position", &self.partition_row_position())
            .field("interleave_position", &self.interleave_position())
            .field("interleaved_data_depth", &self.interleaved_data_depth())
            .field(
                "extended_starting_address",
                &self.extended_starting_address(),
            )
            .field("extended_ending_address", &self.extended_ending_address())
            .finish()
    }
}
