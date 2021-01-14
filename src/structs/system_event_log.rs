use super::*;

pub struct SMBiosSystemEventLog<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemEventLog<'a> {
    const STRUCT_TYPE: u8 = 15u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemEventLog<'a> {
    fn log_area_length(&self) -> Option<u16> {
        self.parts.get_field_word(0x04)
    }

    fn log_header_start_offset(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    fn log_data_start_offset(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    fn access_method(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0A)
    }

    fn log_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0B)
    }

    fn log_change_token(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0C)
    }

    fn access_method_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    fn log_header_format(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    fn number_of_supported_log_type_descriptors(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    fn length_of_each_log_type_descriptor(&self) -> Option<u8> {
        self.parts.get_field_byte(0x16)
    }

    // fn list_of_supported_event_log_type_descriptors(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x17)
    // }
}

impl fmt::Debug for SMBiosSystemEventLog<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemEventLog>())
            .field("header", &self.parts.header)
            .field("log_area_length", &self.log_area_length())
            .field("log_header_start_offset", &self.log_header_start_offset())
            .field("log_data_start_offset", &self.log_data_start_offset())
            .field("access_method", &self.access_method())
            .field("log_status", &self.log_status())
            .field("log_change_token", &self.log_change_token())
            .field("access_method_address", &self.access_method_address())
            .field("log_header_format", &self.log_header_format())
            .field(
                "number_of_supported_log_type_descriptors",
                &self.number_of_supported_log_type_descriptors(),
            )
            .field(
                "length_of_each_log_type_descriptor",
                &self.length_of_each_log_type_descriptor(),
            )
            // .field("list_of_supported_event_log_type_descriptors", &self.list_of_supported_event_log_type_descriptors())
            .finish()
    }
}
