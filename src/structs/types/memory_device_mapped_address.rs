use crate::core::{Handle, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};

/// # Memory Device Mapped Address (Type 20)
///
/// This structure maps memory address space usually to a device-level granularity.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryDeviceMappedAddress<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryDeviceMappedAddress<'a> {
    const STRUCT_TYPE: u8 = 20u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosMemoryDeviceMappedAddress<'a> {
    /// Physical address, in kilobytes, of a range of
    /// memory mapped to the referenced [super::SMBiosMemoryDevice]
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
    /// [super::SMBiosMemoryDevice]
    /// When the field value is FFFF FFFFh the actual
    /// address is stored in the Extended Ending Address
    /// field. When this field contains a valid address,
    /// Starting Address must also contain a valid
    /// address.
    pub fn ending_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x8)
    }

    /// Handle, or instance number, associated with the
    /// [super::SMBiosMemoryDevice] structure to which this address
    /// range is mapped
    /// Multiple address ranges can be mapped to a
    /// single [super::SMBiosMemoryDevice]
    pub fn memory_device_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xC)
    }

    /// Handle, or instance number, associated with the
    /// Memory Array Mapped Address structure to which
    /// this device address range is mapped
    /// Multiple address ranges can be mapped to a
    /// single [super::SMBiosMemoryArrayMappedAddress].
    pub fn memory_array_mapped_address_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xE)
    }

    /// Position of the referenced [super::SMBiosMemoryDevice] in a row
    /// of the address partition
    /// For example, if two 8-bit devices form a 16-bit row,
    /// this field’s value is either 1 or 2.
    /// The value 0 is reserved. If the position is
    /// unknown, the field contains FFh.
    pub fn partition_row_position(&self) -> Option<u8> {
        self.parts.get_field_byte(0x10)
    }

    /// Position of the referenced [super::SMBiosMemoryDevice] in an
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
    /// referenced [super::SMBiosMemoryDevice] that are accessed in a
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
    /// mapped to the referenced [super::SMBiosMemoryDevice]
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
    /// [super::SMBiosMemoryDevice]
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosMemoryDeviceMappedAddress<'_>>())
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

impl Serialize for SMBiosMemoryDeviceMappedAddress<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosMemoryDeviceMappedAddress", 10)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("starting_address", &self.starting_address())?;
        state.serialize_field("ending_address", &self.ending_address())?;
        state.serialize_field("memory_device_handle", &self.memory_device_handle())?;
        state.serialize_field(
            "memory_array_mapped_address_handle",
            &self.memory_array_mapped_address_handle(),
        )?;
        state.serialize_field("partition_row_position", &self.partition_row_position())?;
        state.serialize_field("interleave_position", &self.interleave_position())?;
        state.serialize_field("interleaved_data_depth", &self.interleaved_data_depth())?;
        state.serialize_field(
            "extended_starting_address",
            &self.extended_starting_address(),
        )?;
        state.serialize_field("extended_ending_address", &self.extended_ending_address())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type20 = vec![
            0x14, 0x23, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x7F, 0x00, 0x40, 0x00,
            0x3F, 0x00, 0x01, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type20);
        let test_struct = SMBiosMemoryDeviceMappedAddress::new(&parts);

        assert_eq!(test_struct.starting_address(), Some(0));
        assert_eq!(test_struct.ending_address(), Some(8388607));
        assert_eq!(*test_struct.memory_device_handle().unwrap(), 64);
        assert_eq!(
            *test_struct.memory_array_mapped_address_handle().unwrap(),
            63
        );
        assert_eq!(test_struct.partition_row_position(), Some(1));
        assert_eq!(test_struct.interleave_position(), Some(1));
        assert_eq!(test_struct.interleaved_data_depth(), Some(2));
        assert_eq!(test_struct.extended_starting_address(), Some(0));
        assert_eq!(test_struct.extended_ending_address(), Some(0));
    }
}
