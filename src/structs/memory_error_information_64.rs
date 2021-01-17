use super::*;

/// # 64-Bit Memory Error Information (Type 33)
///
/// This structure describes an error within a [SMBiosPhysicalMemoryArray], when the error address is above 4G (0xFFFFFFFF).
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryErrorInformation64<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryErrorInformation64<'a> {
    const STRUCT_TYPE: u8 = 33u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryErrorInformation64<'a> {
    /// Type of error that is associated with the current
    /// status reported for the memory array or device
    pub fn error_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Granularity (for example, device versus Partition)
    /// to which the error can be resolved
    pub fn error_granularity(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Memory access operation that caused the error
    pub fn error_operation(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Vendor-specific ECC syndrome or CRC data
    /// associated with the erroneous access
    /// If the value is unknown, this field contains 0000
    /// 0000h.
    pub fn vendor_syndrome(&self) -> Option<u32> {
        self.parts.get_field_dword(0x07)
    }

    /// 64-bit physical address of the error based on the
    /// addressing of the bus to which the memory array is
    /// connected
    /// If the address is unknown, this field contains 8000 0000
    /// 0000 0000h.
    pub fn memory_array_error_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x0B)
    }

    /// 64-bit physical address of the error relative to the start of
    /// the failing memory device, in bytes
    /// If the address is unknown, this field contains 8000 0000
    /// 0000 0000h.
    pub fn device_error_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x13)
    }

    /// Range, in bytes, within which the error can be determined,
    /// when an error address is given
    /// If the range is unknown, this field contains 8000 0000h.
    pub fn error_resolution(&self) -> Option<u32> {
        self.parts.get_field_dword(0x1B)
    }
}

impl fmt::Debug for SMBiosMemoryErrorInformation64<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryErrorInformation64>())
            .field("header", &self.parts.header)
            .field("error_type", &self.error_type())
            .field("error_granularity", &self.error_granularity())
            .field("error_operation", &self.error_operation())
            .field("vendor_syndrome", &self.vendor_syndrome())
            .field(
                "memory_array_error_address",
                &self.memory_array_error_address(),
            )
            .field("device_error_address", &self.device_error_address())
            .field("error_resolution", &self.error_resolution())
            .finish()
    }
}
