use super::*;

/// # Physical Memory Array (Type 16)
///
/// This structure describes a collection of memory devices that operate together to form a memory address space.
/// 
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosPhysicalMemoryArray<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosPhysicalMemoryArray<'a> {
    const STRUCT_TYPE: u8 = 16u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosPhysicalMemoryArray<'a> {
    /// Physical location of the Memory Array, whether on
    /// the system board or an add-in board
    pub fn location(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Function for which the array is used
    pub fn usage(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Primary hardware error correction or detection
    /// method supported by this memory array
    pub fn memory_error_correction(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Maximum memory capacity, in kilobytes, for this array
    /// If the capacity is not represented in this field, then
    /// this field contains 8000 0000h and the Extended
    /// Maximum Capacity field should be used. Values 2
    /// TB (8000 0000h) or greater must be represented
    /// in the Extended Maximum Capacity field.
    pub fn maximum_capacity(&self) -> Option<u32> {
        self.parts.get_field_dword(0x07)
    }

    /// Handle, or instance number, associated with any
    /// error that was previously detected for the array
    /// If the system does not provide the error
    /// information structure, the field contains FFFEh;
    /// otherwise, the field contains either FFFFh (if no
    /// error was detected) or the handle of the errorinformation structure.
    pub fn memory_error_information_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x0B)
    }

    /// Number of slots or sockets available for [SMBiosMemoryDevice]s in this array
    /// This value represents the number of [SMBiosMemoryDevice]
    /// structures that compose this Memory
    /// Array. Each [SMBiosMemoryDevice] has a reference to
    /// the “owning” Memory Array.
    pub fn number_of_memory_devices(&self) -> Option<u16> {
        self.parts.get_field_word(0x0D)
    }

    /// Maximum memory capacity, in bytes, for this array
    /// This field is only valid when the Maximum
    /// Capacity field contains 8000 0000h. When
    /// Maximum Capacity contains a value that is not
    /// 8000 0000h, Extended Maximum Capacity must
    /// contain zeros.
    pub fn extended_maximum_capacity(&self) -> Option<u64> {
        self.parts.get_field_qword(0x0F)
    }
}

impl fmt::Debug for SMBiosPhysicalMemoryArray<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPhysicalMemoryArray>())
            .field("header", &self.parts.header)
            .field("location", &self.location())
            .field("usage", &self.usage())
            .field("memory_error_correction", &self.memory_error_correction())
            .field("maximum_capacity", &self.maximum_capacity())
            .field(
                "memory_error_information_handle",
                &self.memory_error_information_handle(),
            )
            .field("number_of_memory_devices", &self.number_of_memory_devices())
            .field(
                "extended_maximum_capacity",
                &self.extended_maximum_capacity(),
            )
            .finish()
    }
}
