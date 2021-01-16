use super::*;

/// # 32-Bit Memory Error Information (Type 18)
///
/// This structure identifies the specifics of an error that might be detected within a [SMBiosPhysicalMemoryArray].
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryErrorInformation32<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryErrorInformation32<'a> {
    const STRUCT_TYPE: u8 = 18u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryErrorInformation32<'a> {
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

    /// 32-bit physical address of the error based on the
    /// addressing of the bus to which the memory array
    /// is connected
    /// If the address is unknown, this field contains
    /// 8000 0000h.
    pub fn memory_array_error_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0B)
    }

    /// 32-bit physical address of the error relative to the
    /// start of the failing memory device, in bytes
    /// If the address is unknown, this field contains
    /// 8000 0000h.
    pub fn device_error_address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0F)
    }

    /// Range, in bytes, within which the error can be
    /// determined, when an error address is given
    /// If the range is unknown, this field contains 8000
    /// 0000h.
    pub fn error_resolution(&self) -> Option<u32> {
        self.parts.get_field_dword(0x13)
    }
}

impl fmt::Debug for SMBiosMemoryErrorInformation32<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryErrorInformation32>())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type18 = vec![
            0x12, 0x17, 0x50, 0x00, 0x03, 0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type18.as_slice());
        let test_struct = SMBiosMemoryErrorInformation32::new(&parts);

        assert_eq!(test_struct.error_type(), Some(3));
        assert_eq!(test_struct.error_granularity(), Some(2));
        assert_eq!(test_struct.error_operation(), Some(2));
        assert_eq!(test_struct.vendor_syndrome(), Some(0));
        assert_eq!(test_struct.memory_array_error_address(), Some(2147483648));
        assert_eq!(test_struct.device_error_address(), Some(2147483648));
        assert_eq!(test_struct.error_resolution(), Some(2147483648));
    }
}
