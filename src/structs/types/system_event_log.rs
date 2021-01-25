use crate::*;

/// # System Event Log (Type 15)
///
/// The presence of this structure within the SMBIOS data returned for a system indicates that the system
/// supports an event log. An event log is a fixed-length area within a non-volatile
/// storage element, starting with a fixed-length (and vendor-specific) header record, followed by one or more
/// variable-length log records.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
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
    /// Length, in bytes, of the overall event log area,
    /// from the first byte of header to the last byte of data
    pub fn log_area_length(&self) -> Option<u16> {
        self.parts.get_field_word(0x04)
    }

    /// Defines the starting offset (or index) within the
    /// nonvolatile storage of the event-log’s header,
    /// from the Access Method Address
    /// For single-byte indexed I/O accesses, the
    /// most-significant byte of the start offset is set
    /// to 00h.
    pub fn log_header_start_offset(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    /// Defines the starting offset (or index) within the
    /// nonvolatile storage of the event-log’s first
    /// data byte, from the Access Method Address
    /// For single-byte indexed I/O accesses, the
    /// most-significant byte of the start offset is set
    /// to 00h.
    ///
    /// NOTE: The data directly follows any header
    /// information. Therefore, the header length
    /// can be determined by subtracting the
    /// Header Start Offset from the Data Start
    /// Offset.
    pub fn log_data_start_offset(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Defines the Location and Method used by higher-level software to access the log area
    pub fn access_method(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0A)
    }

    /// Current status of the system event-log
    pub fn log_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0B)
    }

    /// Unique token that is reassigned every time
    /// the event log changes
    /// Can be used to determine if additional events
    /// have occurred since the last time the log was
    /// read.
    pub fn log_change_token(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0C)
    }

    /// Address associated with the access method;
    /// the data present depends on the Access
    /// Method field value
    pub fn access_method_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    /// Format of the log header area
    pub fn log_header_format(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    /// Number of supported event log type
    /// descriptors that follow
    /// If the value is 0, the list that starts at offset
    /// 17h is not present.
    pub fn number_of_supported_log_type_descriptors(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    /// Number of bytes associated with each type
    /// entry in the list below
    /// The value is currently “hard-coded” as 2,
    /// because each entry consists of two bytes.
    /// This field’s presence allows future additions
    /// to the type list. Software that interprets the
    /// following list should not assume a list entry’s
    /// length.
    pub fn length_of_each_log_type_descriptor(&self) -> Option<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type15 = vec![
            0x0F, 0x49, 0x3D, 0x00, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x03, 0x01, 0x05, 0x00,
            0x00, 0x00, 0x18, 0x20, 0xAE, 0x6A, 0x01, 0x19, 0x02, 0x01, 0x03, 0x02, 0x03, 0x03,
            0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x04, 0x09, 0x03, 0x0A,
            0x03, 0x0B, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x10, 0x00, 0x11, 0x00, 0x12,
            0x00, 0x13, 0x00, 0x14, 0x00, 0x15, 0x00, 0x16, 0x00, 0x17, 0x00, 0xFF, 0x00, 0xE0,
            0xE0, 0xE1, 0xE1, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type15.as_slice());
        let test_struct = SMBiosSystemEventLog::new(&parts);

        assert_eq!(test_struct.log_area_length(), Some(4096));
        assert_eq!(test_struct.log_header_start_offset(), Some(0));
        assert_eq!(test_struct.log_data_start_offset(), Some(16));
        assert_eq!(test_struct.access_method(), Some(3));
        assert_eq!(test_struct.log_status(), Some(1));
        assert_eq!(test_struct.log_change_token(), Some(5));
        assert_eq!(test_struct.access_method_address(), Some(1789796376));
        assert_eq!(test_struct.log_header_format(), Some(1));
        assert_eq!(
            test_struct.number_of_supported_log_type_descriptors(),
            Some(25)
        );
        assert_eq!(test_struct.length_of_each_log_type_descriptor(), Some(2));
    }
}
