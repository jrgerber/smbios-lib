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
    const LOG_TYPE_DESCRIPTORS_OFFSET: usize = 0x17usize;

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
    pub fn access_method(&self) -> Option<AccessMethodData> {
        self.parts
            .get_field_byte(0x0A)
            .and_then(|raw| Some(AccessMethodData::from(raw)))
    }

    /// Current status of the system event-log
    pub fn log_status(&self) -> Option<LogStatus> {
        self.parts
            .get_field_byte(0x0B)
            .and_then(|raw| Some(LogStatus::from(raw)))
    }

    /// Unique token that is reassigned every time
    /// the event log changes
    ///
    /// Can be used to determine if additional events
    /// have occurred since the last time the log was
    /// read.
    pub fn log_change_token(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0C)
    }

    /// Address associated with the access method
    ///
    /// The data present depends on the Access
    /// Method field value
    pub fn access_method_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    /// Format of the log header area
    pub fn log_header_format(&self) -> Option<HeaderFormatData> {
        self.parts
            .get_field_byte(0x14)
            .and_then(|raw| Some(HeaderFormatData::from(raw)))
    }

    /// Number of supported event log type
    /// descriptors that follow
    ///
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

    /// Type Descriptors
    pub fn type_descriptors(&self) -> Option<TypeDescriptors> {
        TypeDescriptors::new(self)
    }
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
            .field("type_descriptors", &self.type_descriptors())
            .finish()
    }
}

/// # System Event Log - Log Type Data
pub struct LogTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [LogType] value
    pub value: LogType,
}

impl fmt::Debug for LogTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<LogTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for LogTypeData {
    type Target = LogType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # System Event Log - Log Type
#[derive(Debug, PartialEq, Eq)]
pub enum LogType {
    /// Single-bit ECC memory error
    SingleBitEccMemoryError,
    /// Multi-bit ECC memory error
    MultiBitEccMemoryError,
    /// Parity memory error
    ParityMemoryError,
    /// Bus time-out
    BusTimeOut,
    /// I/O Channel Check
    IOChannelCheck,
    /// Software NMI
    SoftwareNmi,
    /// POST Memory Resize
    PostMemoryResize,
    /// POST Error
    PostError,
    /// PCI Parity Error
    PciParityError,
    /// PCI System Error
    PciSystemError,
    /// CPU Failure
    CpuFailure,
    /// EISA FailSafe Timer time-out
    EisaFailSafeTimerTimeout,
    /// Correctable memory log disabled
    CorrectableMemoryLogDisabled,
    /// Logging disabled for a specific Event Type — too many errors of the same type received in a short amount of time
    LoggingDisabledForSpecificEventType,
    /// Reserved
    Reserved0F,
    /// System Limit Exceeded (for example, voltage or temperature threshold exceeded)
    SystemLimitExceeded,
    /// Asynchronous hardware timer expired and issued a system reset
    AsyncHardwareTimerExpired,
    /// System configuration information
    SystemConfigurationInformation,
    /// Hard-disk information
    HardDiskInformation,
    /// System reconfigured
    SystemReconfigured,
    /// Uncorrectable CPU-complex error
    UncorrectableCpuComplexError,
    /// Log Area Reset/Cleared
    LogAreaReset,
    /// System boot. If implemented, this log entry is guaranteed to be the first one written on any system boot.
    SystemBoot,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for LogTypeData {
    fn from(raw: u8) -> Self {
        LogTypeData {
            value: match raw {
                0x01 => LogType::SingleBitEccMemoryError,
                0x02 => LogType::MultiBitEccMemoryError,
                0x03 => LogType::ParityMemoryError,
                0x04 => LogType::BusTimeOut,
                0x05 => LogType::IOChannelCheck,
                0x06 => LogType::SoftwareNmi,
                0x07 => LogType::PostMemoryResize,
                0x08 => LogType::PostError,
                0x09 => LogType::PciParityError,
                0x0A => LogType::PciSystemError,
                0x0B => LogType::CpuFailure,
                0x0C => LogType::EisaFailSafeTimerTimeout,
                0x0D => LogType::CorrectableMemoryLogDisabled,
                0x0E => LogType::LoggingDisabledForSpecificEventType,
                0x0F => LogType::Reserved0F,
                0x10 => LogType::SystemLimitExceeded,
                0x11 => LogType::AsyncHardwareTimerExpired,
                0x12 => LogType::SystemConfigurationInformation,
                0x13 => LogType::HardDiskInformation,
                0x14 => LogType::SystemReconfigured,
                0x15 => LogType::UncorrectableCpuComplexError,
                0x16 => LogType::LogAreaReset,
                0x17 => LogType::SystemBoot,
                _ => LogType::None,
            },
            raw,
        }
    }
}

/// # System Event Log - Variable Data Format Type Data
pub struct VariableDataFormatTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [VariableDataFormatType] value
    pub value: VariableDataFormatType,
}

impl fmt::Debug for VariableDataFormatTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<VariableDataFormatTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for VariableDataFormatTypeData {
    type Target = VariableDataFormatType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # System Event Log - Variable Data Format Type
#[derive(Debug, PartialEq, Eq)]
pub enum VariableDataFormatType {
    /// No standard format data is available; the first byte of the variable data (if present) contains OEM-specific unformatted information.
    NoStandardFormat,
    /// The first WORD of the variable data contains the handle of the SMBIOS structure associated with the hardware element that failed.
    Handle,
    /// The first DWORD of the variable data contains a multiple-event counter (see 7.16.6.3 for details).
    MultipleEvent,
    /// The first WORD of the variable data contains the handle of the SMBIOS structure associated with the hardware element that failed; it is followed by a DWORD containing a multiple-event counter (see 7.16.6.3 for details).
    MultipleEventHandle,
    /// The first two DWORDs of the variable data contain the POST Results Bitmap, as described in 7.16.6.4.
    PostResultsBitmap,
    /// The first DWORD of the variable data contains a value that identifies a system-management condition. See 7.16.6.5 for the enumerated values.
    SystemManagementType,
    /// The first DWORD of the variable data contains a value that identifies a system-management condition. (See 7.16.6.5 for the enumerated values.) This DWORD is directly followed by a DWORD that contains a multiple- event counter (see 7.16.6.3 for details).
    MultipleEventSystemManagementType,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for VariableDataFormatTypeData {
    fn from(raw: u8) -> Self {
        VariableDataFormatTypeData {
            value: match raw {
                0x00 => VariableDataFormatType::NoStandardFormat,
                0x01 => VariableDataFormatType::Handle,
                0x02 => VariableDataFormatType::MultipleEvent,
                0x03 => VariableDataFormatType::MultipleEventHandle,
                0x04 => VariableDataFormatType::PostResultsBitmap,
                0x05 => VariableDataFormatType::SystemManagementType,
                0x06 => VariableDataFormatType::MultipleEventSystemManagementType,
                _ => VariableDataFormatType::None,
            },
            raw,
        }
    }
}

/// # System Event Log - Access Method Data
pub struct AccessMethodData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [AccessMethod] value
    pub value: AccessMethod,
}

impl fmt::Debug for AccessMethodData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<AccessMethodData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for AccessMethodData {
    type Target = AccessMethod;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for AccessMethodData {
    fn from(raw: u8) -> Self {
        AccessMethodData {
            value: AccessMethod::from(raw),
            raw,
        }
    }
}
/// # System Event Log - Access Method
///
/// Defines the Location and Method used by higher-level software to access the log area.
#[derive(Debug, PartialEq, Eq)]
pub enum AccessMethod {
    /// 00h Indexed I/O
    ///
    /// 1 8-bit index port, 1 8-bit data port. The Access Method Address field contains the
    /// 16-bit I/O addresses for the index and data ports. See 7.16.2.1 for usage details.
    IndexedIO18Bit,
    /// 01h Indexed I/O
    ///
    /// 2 8-bit index ports, 1 8-bit data port. The Access Method Address field contains the
    /// 16-bit I/O address for the index and data ports. See 7.16.2.2 for usage details.
    IndexedIO28Bit,
    /// 02h Indexed I/O
    ///
    /// 1 16-bit index port, 1 8-bit data port. The Access Method Address field contains the
    /// 16-bit I/O address for the index and data ports. See 7.16.2.3 for usage details.
    IndexedIO116Bit,
    /// 03h Memory-mapped physical 32-bit address.
    ///
    /// The Access Method Address field contains the 4-byte (Intel DWORD format) starting physical address.
    MemoryMapped32Bit,
    /// 04h Available through General-Purpose NonVolatile Data functions.
    ///
    /// The Access Method Address field contains the 2-byte (Intel WORD format) GPNV handle.
    GeneralPurposeNonVolatile,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for AccessMethod {
    fn from(raw: u8) -> Self {
        match raw {
            0x00 => AccessMethod::IndexedIO18Bit,
            0x01 => AccessMethod::IndexedIO28Bit,
            0x02 => AccessMethod::IndexedIO116Bit,
            0x03 => AccessMethod::MemoryMapped32Bit,
            0x04 => AccessMethod::GeneralPurposeNonVolatile,
            _ => AccessMethod::None,
        }
    }
}

/// System Event Log Type Descriptor
///
/// Each entry consists of a 1-byte type field and a 1-byte data-format descriptor, as shown in Table 61. The
/// presence of an entry identifies that the Log Type is supported by the system and the format of any
/// variable data that accompanies the first bytes of the log’s variable data — a specific log record might
/// have more variable data than specified by its Variable Data Format Type.
pub struct EventLogTypeDescriptor<'a> {
    /// Raw byte slice for this event log type descriptor
    pub raw: &'a [u8],
}

impl<'a> EventLogTypeDescriptor<'a> {
    const MINIMUM_RAW_SIZE: usize = 2usize;
    const LOG_TYPE_OFFSET: usize = 0usize;
    const VARIABLE_DATA_FORMAT_TYPE_OFFSET: usize = 1usize;

    fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < Self::MINIMUM_RAW_SIZE {
            None
        } else {
            Some(Self { raw })
        }
    }

    /// Event Log Type
    pub fn log_type(&self) -> LogTypeData {
        LogTypeData::from(self.raw[Self::LOG_TYPE_OFFSET])
    }

    /// Event Log Variable Data Format Type
    ///
    /// The Variable Data Format Type, specified in the Event Log structure’s Supported Event Type fields,
    /// identifies the standard format that application software can apply to the first n bytes of the associated.
    /// Log Type’s variable data.Additional OEM-specific data might follow in the log’s variable data field.
    pub fn variable_data_format_type(&self) -> VariableDataFormatTypeData {
        VariableDataFormatTypeData::from(self.raw[Self::VARIABLE_DATA_FORMAT_TYPE_OFFSET])
    }
}

impl<'a> fmt::Debug for EventLogTypeDescriptor<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<EventLogTypeDescriptor>())
            .field("raw", &self.raw)
            .field("log_type", &self.log_type())
            .field(
                "variable_data_format_type",
                &self.variable_data_format_type(),
            )
            .finish()
    }
}

/// # System Event Log Type Descriptors within [SMBiosSystemEventLog]
pub struct TypeDescriptors<'a> {
    raw: &'a [u8],
    record_count: usize,
    record_length: usize,
}

impl<'a> TypeDescriptors<'a> {
    fn new(system_event_log: &'a SMBiosSystemEventLog<'a>) -> Option<Self> {
        system_event_log
            .length_of_each_log_type_descriptor()
            .and_then(|record_length| {
                system_event_log
                    .number_of_supported_log_type_descriptors()
                    .and_then(|record_count| {
                        system_event_log
                            .parts()
                            .get_field_data(
                                SMBiosSystemEventLog::LOG_TYPE_DESCRIPTORS_OFFSET,
                                SMBiosSystemEventLog::LOG_TYPE_DESCRIPTORS_OFFSET
                                    + (record_length as usize * record_count as usize),
                            )
                            .and_then(|raw| {
                                Some(Self {
                                    raw,
                                    record_count: record_count as usize,
                                    record_length: record_length as usize,
                                })
                            })
                    })
            })
    }
}

impl<'a> fmt::Debug for TypeDescriptors<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<TypeDescriptors>())
            .field("descriptors", &self.into_iter())
            .finish()
    }
}

/// # Iterates over the [EventLogTypeDescriptor] entries within [TypeDescriptors]
pub struct TypeDescriptorsIterator<'a> {
    descriptors: &'a TypeDescriptors<'a>,
    current_index: usize,
    current_entry: usize,
}

impl<'a> TypeDescriptorsIterator<'a> {
    fn reset(&mut self) {
        self.current_index = 0;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a TypeDescriptors<'a> {
    type Item = EventLogTypeDescriptor<'a>;
    type IntoIter = TypeDescriptorsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TypeDescriptorsIterator {
            descriptors: self,
            current_index: 0,
            current_entry: 0,
        }
    }
}

impl<'a> Iterator for TypeDescriptorsIterator<'a> {
    type Item = EventLogTypeDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.descriptors.record_count {
            self.reset();
            return None;
        }

        let next_index = self.current_index + self.descriptors.record_length;
        match EventLogTypeDescriptor::new(&self.descriptors.raw[self.current_index..next_index]) {
            Some(event_log_type_descriptor) => {
                self.current_index = next_index;
                self.current_entry += 1;
                Some(event_log_type_descriptor)
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for TypeDescriptorsIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list()
            .entries(self.descriptors.into_iter())
            .finish()
    }
}

/// # System Event Log - Log Status
#[derive(PartialEq, Eq)]
pub struct LogStatus {
    /// Raw value
    pub raw: u8,
}

impl Deref for LogStatus {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for LogStatus {
    fn from(raw: u8) -> Self {
        LogStatus { raw }
    }
}

impl LogStatus {
    /// If true, log area valid; otherwise false
    pub fn log_area_valid(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// If true log area full; otherwise false
    pub fn log_area_full(&self) -> bool {
        self.raw & 0x02 == 0x02
    }
}

impl fmt::Debug for LogStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<LogStatus>())
            .field("raw", &self.raw)
            .field("log_area_valid", &self.log_area_valid())
            .field("log_area_full", &self.log_area_full())
            .finish()
    }
}

/// # System Event Log - Header Format Data
pub struct HeaderFormatData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [HeaderFormat] value
    pub value: HeaderFormat,
}

impl fmt::Debug for HeaderFormatData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<HeaderFormatData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for HeaderFormatData {
    type Target = HeaderFormat;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # System Event Log - Header Format
#[derive(Debug, PartialEq, Eq)]
pub enum HeaderFormat {
    /// No header (for example, the header is 0 bytes in length)
    NoHeader,
    /// Type 1 log header
    Type1LogHeader,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for HeaderFormatData {
    fn from(raw: u8) -> Self {
        HeaderFormatData {
            value: match raw {
                0x00 => HeaderFormat::NoHeader,
                0x01 => HeaderFormat::Type1LogHeader,
                _ => HeaderFormat::None,
            },
            raw,
        }
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

        println!("{:?}", test_struct);
        assert_eq!(test_struct.log_area_length(), Some(4096));
        assert_eq!(test_struct.log_header_start_offset(), Some(0));
        assert_eq!(test_struct.log_data_start_offset(), Some(16));
        assert_eq!(
            *test_struct.access_method().unwrap(),
            AccessMethod::MemoryMapped32Bit
        );
        assert_eq!(
            test_struct.log_status().unwrap(),
            LogStatus::from(0b0000_0001)
        );
        assert_eq!(test_struct.log_change_token(), Some(5));
        assert_eq!(test_struct.access_method_address(), Some(1789796376));
        assert_eq!(
            *test_struct.log_header_format().unwrap(),
            HeaderFormat::Type1LogHeader
        );
        assert_eq!(
            test_struct.number_of_supported_log_type_descriptors(),
            Some(25)
        );
        assert_eq!(test_struct.length_of_each_log_type_descriptor(), Some(2));

        let type_descriptors = test_struct.type_descriptors().unwrap();
        let mut iterator = type_descriptors.into_iter();
        let first = iterator.next().unwrap();
        assert_eq!(*first.log_type(), LogType::SingleBitEccMemoryError);
    }
}
