use super::*;

/// # System Boot Information (Type 32)
///
/// The client system firmware (for example, BIOS) communicates the 
/// System Boot Status to the client’s Pre1864 boot Execution Environment
/// (PXE) boot image or OS-present management application through this
/// structure.
/// 
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemBootInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemBootInformation<'a> {
    const STRUCT_TYPE: u8 = 32u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemBootInformation<'a> {
    // TODO:
    
    // fn reserved(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x04)
    // }

    // fn boot_status(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x0A)
    // }
}

impl fmt::Debug for SMBiosSystemBootInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemBootInformation>())
        .field("header", &self.parts.header)
        // .field("reserved", &self.reserved())
        // .field("boot_status", &self.boot_status())
        .finish()
    }
}

