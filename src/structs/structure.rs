use crate::core::UndefinedStruct;

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
