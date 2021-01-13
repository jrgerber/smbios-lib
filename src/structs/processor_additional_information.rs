use super::*;

/// # Processor Additional Information (Type 44)
/// 
/// The information in this structure defines the processor additional information in case SMBIOS type 4 [SMBiosProcessorInformation] is
/// not sufficient to describe processor characteristics. The SMBIOS type 44 structure has a reference
/// handle field to link back to the related SMBIOS type 4 structure. There may be multiple SMBIOS type 44
/// structures linked to the same SMBIOS type 4 structure. For example, when cores are not identical in a
/// processor, SMBIOS type 44 structures describe different core-specific information.
/// 
/// SMBIOS type 44 defines the standard header for the processor-specific block, while the
/// contents of processor-specific data are maintained by processor architecture workgroups or vendors in
/// separate documents.
pub struct SMBiosProcessorAdditionalInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosProcessorAdditionalInformation<'a> {
    const STRUCT_TYPE: u8 = 44u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosProcessorAdditionalInformation<'a> {
    /// Handle, or instance number, associated with the
    /// [SMBiosProcessorInformation] structure (SMBIOS type 4) which the
    /// Processor Additional Information structure describes.
    fn referenced_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x04)
    }

    // TODO: This is an array that must be implemented

    // fn block_length(&self) -> Option<u8> {
    //     self.parts.get_field_byte(0x06)
    // }

    // fn processor_type(&self) -> Option<u8> {
    //     self.parts.get_field_byte(0x07)
    // }

    // fn processor_specific_data(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x08)
    // }
}

impl fmt::Debug for SMBiosProcessorAdditionalInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosProcessorAdditionalInformation>())
        .field("header", &self.parts.header)
        .field("referenced_handle", &self.referenced_handle())
        // .field("block_length", &self.block_length())
        // .field("processor_type", &self.processor_type())
        // .field("processor_specific_data", &self.processor_specific_data())
        .finish()
    }
}

