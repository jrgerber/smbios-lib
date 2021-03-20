use crate::core::{Handle, UndefinedStruct};
use crate::SMBiosStruct;
use std::fmt;

/// # Memory Array Mapped Address (Type 19)
///
/// This structure provides the address mapping for a Physical Memory Array.
///
/// One structure is present for each contiguous address range described.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryArrayMappedAddress<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryArrayMappedAddress<'a> {
    const STRUCT_TYPE: u8 = 19u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosMemoryArrayMappedAddress<'a> {
    /// Physical address, in kilobytes, of a range of
    /// memory mapped to the specified Physical Memory
    /// Array
    /// When the field value is FFFF FFFFh, the actual
    /// address is stored in the Extended Starting
    /// Address field. When this field contains a valid
    /// address, Ending Address must also contain a valid
    /// address. When this field contains FFFF FFFFh,
    /// Ending Address must also contain FFFF FFFFh.
    pub fn starting_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x4)
    }

    /// Physical ending address of the last kilobyte of a
    /// range of addresses mapped to the specified
    /// Physical Memory Array
    /// When the field value is FFFF FFFFh and the
    /// Starting Address field also contains FFFF FFFFh,
    /// the actual address is stored in the Extended
    /// Ending Address field. When this field contains a
    /// valid address, Starting Address must also contain
    /// a valid address.
    pub fn ending_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x8)
    }

    /// Handle, or instance number, associated with the
    /// Physical Memory Array to which this address
    /// range is mapped
    /// Multiple address ranges can be mapped to a
    /// single Physical Memory Array.
    pub fn physical_memory_array_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0xC)
    }

    /// Number of Memory Devices that form a single row
    /// of memory for the address partition defined by this
    /// structure
    pub fn partition_width(&self) -> Option<u8> {
        self.parts.get_field_byte(0xE)
    }

    /// Physical address, in bytes, of a range of memory
    /// mapped to the specified Physical Memory Array
    /// This field is valid when Starting Address contains
    /// the value FFFF FFFFh. If Starting Address
    /// contains a value other than FFFF FFFFh, this field
    /// contains zeros. When this field contains a valid
    /// address, Extended Ending Address must also
    /// contain a valid address.
    pub fn extended_starting_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0xF)
    }

    /// Physical ending address, in bytes, of the last of a
    /// range of addresses mapped to the specified
    /// Physical Memory Array
    /// This field is valid when both Starting Address and
    /// Ending Address contain the value FFFF FFFFh. If
    /// Ending Address contains a value other than FFFF
    /// FFFFh, this field contains zeros. When this field
    /// contains a valid address, Extended Starting
    /// Address must also contain a valid address.
    pub fn extended_ending_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x17)
    }
}

impl fmt::Debug for SMBiosMemoryArrayMappedAddress<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryArrayMappedAddress<'_>>())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type19 = vec![
            0x13, 0x1F, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x3E, 0x00,
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type19);
        let test_struct = SMBiosMemoryArrayMappedAddress::new(&parts);

        assert_eq!(test_struct.starting_address(), Some(0));
        assert_eq!(test_struct.ending_address(), Some(16777215));
        assert_eq!(*test_struct.physical_memory_array_handle().unwrap(), 62);
        assert_eq!(test_struct.partition_width(), Some(4));
        assert_eq!(test_struct.extended_starting_address(), Some(0));
        assert_eq!(test_struct.extended_ending_address(), Some(0));
    }
}
