use super::*;

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
    fn referenced_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x04)
    }

    fn block_length(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn processor_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    // fn processor_specific_data(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x08)
    // }
}

impl fmt::Debug for SMBiosProcessorAdditionalInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosProcessorAdditionalInformation>())
            .field("header", &self.parts.header)
            .field("referenced_handle", &self.referenced_handle())
            .field("block_length", &self.block_length())
            .field("processor_type", &self.processor_type())
            // .field("processor_specific_data", &self.processor_specific_data())
            .finish()
    }
}
