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
    fn new(parts: &'a SMBiosStructParts<'a>) -> Self;

    /// Contains the standard parts/sections of the implementing SMBIOS type.
    fn parts(&self) -> &'a SMBiosStructParts<'a>;
}

/// # Structure Handle
///
/// Each SMBIOS structure has a handle or instance value associated with it.
/// Some structures will reference other structures by using this value.
///
/// Dereference a handle (*handle) to access its u16 value.
pub struct Handle(pub u16);

impl Deref for Handle {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Handle>())
            .field("handle", &self.0)
            .finish()
    }
}
