use super::*;

/// # Group Associations (Type 14)
///
/// The Group Associations structure is provided for OEMs who want to specify the arrangement or hierarchy
/// of certain components (including other Group Associations) within the system. For example, you can use
/// the Group Associations structure to indicate that two CPUs share a common external cache system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosGroupAssociations<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosGroupAssociations<'a> {
    const STRUCT_TYPE: u8 = 14u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosGroupAssociations<'a> {
    /// A string describing the group
    pub fn group_name(&self) -> Option<u8> {
        self.parts.get_field_byte(0x4)
    }

    /// Item (Structure) Type of this member
    pub fn item_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x5)
    }

    /// Handle corresponding to this structure
    pub fn item_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x6)
    }

    // fn minimum_ending_offset(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x8)
    // }
}

impl fmt::Debug for SMBiosGroupAssociations<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosGroupAssociations>())
            .field("header", &self.parts.header)
            .field("group_name", &self.group_name())
            .field("item_type", &self.item_type())
            .field("item_handle", &self.item_handle())
            // .field("minimum_ending_offset", &self.minimum_ending_offset())
            .finish()
    }
}
