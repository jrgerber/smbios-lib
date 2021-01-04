use super::*;

pub struct SMBiosBaseboardInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosBaseboardInformation<'a> {
    const STRUCT_TYPE: u8 = 2u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosBaseboardInformation<'a> {
    fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn product(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    fn asset_tag(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    fn feature_flags(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    fn location_in_chassis(&self) -> Option<String> {
        self.parts.get_field_string(0x0A)
    }

    fn chassis_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x0B)
    }

    fn board_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0D)
    }

    fn number_of_contained_object_handles(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0E)
    }

    // fn contained_object_handles(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x0F)
    // }
}

impl fmt::Debug for SMBiosBaseboardInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosBaseboardInformation>())
        .field("header", &self.parts.header)
        .field("manufacturer", &self.manufacturer())
        .field("product", &self.product())
        .field("version", &self.version())
        .field("serial_number", &self.serial_number())
        .field("asset_tag", &self.asset_tag())
        .field("feature_flags", &self.feature_flags())
        .field("location_in_chassis", &self.location_in_chassis())
        .field("chassis_handle", &self.chassis_handle())
        .field("board_type", &self.board_type())
        .field("number_of_contained_object_handles", &self.number_of_contained_object_handles())
        // .field("contained_object_handles", &self.contained_object_handles())
        .finish()
    }
}

