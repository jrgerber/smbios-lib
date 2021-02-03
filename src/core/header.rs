use std::convert::TryInto;

use crate::*;

/// # SMBIOS Header
///
/// The header part/section of a structure
pub struct Header<'a> {
    data: &'a [u8],
}

impl fmt::Debug for Header<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Header>())
            .field("struct_type", &self.struct_type())
            .field("length", &self.length())
            .field("handle", &self.handle())
            .finish()
    }
}

impl<'a> Header<'a> {
    /// Total size of a Header (4)
    ///
    /// A header has a byte for the _struct_type_ at offset 0,
    /// a byte for the _length_ at offset 1,
    /// and a word for the _handle_ at offset 2 for a total of
    /// 4 bytes.
    pub const SIZE: usize = 4;
    /// Length offset
    pub const LENGTH_OFFSET: usize = 1;

    /// Creates a new [Header] struct
    pub fn new(data: &'a [u8]) -> Self {
        assert!(
            data.len() == Self::SIZE,
            "Header must be 4 bytes in length, 1 for struct_type, 1 for length, and 2 for handle."
        );
        Header { data }
    }

    /// The type of SMBIOS structure
    pub fn struct_type(&self) -> u8 {
        self.data[0] // struct_type is 1 byte at offset 0
    }

    /// The length of the structure not including the strings part/section
    pub fn length(&self) -> u8 {
        self.data[Self::LENGTH_OFFSET] // length is 1 byte at offset 1
    }

    /// The handle of this structure instance
    pub fn handle(&self) -> Handle {
        // handle is 2 bytes at offset 2
        Handle(u16::from_le_bytes(
            self.data[2..4]
                .try_into()
                .expect("array length does not match type width"),
        ))
    }
}
