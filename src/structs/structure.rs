use crate::*;

/// # SMBIOS Structure
///
/// A type implementing this trait provides a representation of an SMBIOS type.
pub trait SMBiosStruct<'a> {
    /// The SMBIOS structure type
    ///
    /// Example: System Information (Type 1) this is set to 1.
    const STRUCT_TYPE: u8;

    /// Creates a new instance of the implementing SMBIOS type
    fn new(parts: &'a UndefinedStruct) -> Self;

    /// Contains the standard parts/sections of the implementing SMBIOS type.
    fn parts(&self) -> &'a UndefinedStruct;
}

/// # Structure Handle
///
/// Each SMBIOS structure has a handle or instance value associated with it.
/// Some structures will reference other structures by using this value.
///
/// Dereference a handle (*handle) to access its u16 value.
#[derive(PartialEq, Eq)]
pub struct Handle(pub u16);

impl Handle {
    /// Handle Size (2 bytes)
    pub const SIZE: usize = 2usize;
}

impl Deref for Handle {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Handle>())
            .field("handle", &self.0)
            .finish()
    }
}

/// # SMBIOS Structure Type
///
/// Each SMBIOS structure has a type number associated with it.
///
/// Dereference a structure type (*struct_type) to access its u8 value.
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
