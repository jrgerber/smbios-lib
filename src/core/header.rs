use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{convert::TryInto, fmt, ops::Deref, str::FromStr};

/// # Structure Handle
///
/// Each SMBIOS structure has a handle or instance value associated with it.
/// Some structures will reference other structures by using this value.
///
/// Dereference a handle (*handle) to access its u16 value.
#[derive(Serialize, PartialEq, Eq)]
pub struct Handle(pub u16);

impl Handle {
    /// Handle Size (2 bytes)
    pub const SIZE: usize = 2usize;
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Deref for Handle {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Handle {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Handle(if s.starts_with("0x") && s.len() > 2 {
            u16::from_str_radix(&s[2..], 16)?
        } else {
            u16::from_str(s)?
        }))
    }
}

/// # SMBIOS Structure Type
///
/// Each SMBIOS structure has a type number associated with it.
///
/// Dereference a structure type (*struct_type) to access its u8 value.
#[derive(Serialize)]
pub struct SMBiosType(pub u8);

impl Deref for SMBiosType {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for SMBiosType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosType>())
            .field("type", &self.0)
            .finish()
    }
}

/// # SMBIOS Header
///
/// The header part/section of a structure
pub struct Header([u8; 4]);

impl fmt::Debug for Header {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Header>())
            .field("struct_type", &self.struct_type())
            .field("length", &self.length())
            .field("handle", &self.handle())
            .finish()
    }
}

impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Header", 3)?;
        state.serialize_field("struct_type", &self.struct_type())?;
        state.serialize_field("length", &self.length())?;
        state.serialize_field("handle", &self.handle())?;
        state.end()
    }
}

impl Header {
    /// Total size of a Header (4)
    ///
    /// A header has a byte for the _struct_type_ at offset 0,
    /// a byte for the _length_ at offset 1,
    /// and a word for the _handle_ at offset 2 for a total of
    /// 4 bytes.
    pub const SIZE: usize = 4;

    /// StructType offset (offset 0 and 1 bytes)
    pub const STRUCT_TYPE_OFFSET: usize = 0;

    /// Length offset (offset 1 and 1 bytes)
    pub const LENGTH_OFFSET: usize = 1;

    /// Handle offset (offset 2 and 2 bytes)
    pub const HANDLE_OFFSET: usize = 2;

    /// Creates a new [Header] struct
    pub fn new(data: [u8; 4]) -> Self {
        Header(data)
    }

    /// The type of SMBIOS structure
    pub fn struct_type(&self) -> u8 {
        self.0[Self::STRUCT_TYPE_OFFSET] // struct_type is 1 byte at offset 0
    }

    /// The length of the structure not including the strings part/section
    pub fn length(&self) -> u8 {
        self.0[Self::LENGTH_OFFSET] // length is 1 byte at offset 1
    }

    /// The handle of this structure instance
    pub fn handle(&self) -> Handle {
        // handle is 2 bytes at offset 2
        Handle(u16::from_le_bytes(
            self.0[Self::HANDLE_OFFSET..Self::HANDLE_OFFSET + 2]
                .try_into()
                .expect("u16 is 2 bytes"),
        ))
    }

    /// Byte iterator of the header
    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.0.iter()
    }
}

impl From<[u8; 4]> for Header {
    fn from(data: [u8; 4]) -> Self {
        Header(data)
    }
}

impl Deref for Header {
    type Target = [u8; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
