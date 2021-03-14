use crate::{SMBiosStruct, UndefinedStruct};
use std::fmt;
use std::ops::Deref;

/// # 32-Bit Memory Error Information (Type 18)
///
/// This structure identifies the specifics of an error that might be detected within a [SMBiosPhysicalMemoryArray].
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryErrorInformation32<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryErrorInformation32<'a> {
    const STRUCT_TYPE: u8 = 18u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosMemoryErrorInformation32<'a> {
    /// Type of error that is associated with the current
    /// status reported for the memory array or device
    pub fn error_type(&self) -> Option<MemoryErrorTypeData> {
        self.parts
            .get_field_byte(0x04)
            .map(|raw| MemoryErrorTypeData::from(raw))
    }

    /// Granularity (for example, device versus Partition)
    /// to which the error can be resolved
    pub fn error_granularity(&self) -> Option<MemoryErrorGranularityData> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| MemoryErrorGranularityData::from(raw))
    }

    /// Memory access operation that caused the error
    pub fn error_operation(&self) -> Option<MemoryErrorOperationData> {
        self.parts
            .get_field_byte(0x06)
            .map(|raw| MemoryErrorOperationData::from(raw))
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryErrorInformation32<'_>>())
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

/// # Memory Error - Error Type Data
pub struct MemoryErrorTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryErrorType] value
    pub value: MemoryErrorType,
}

impl fmt::Debug for MemoryErrorTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<MemoryErrorTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for MemoryErrorTypeData {
    type Target = MemoryErrorType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Error - Error Type
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryErrorType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// OK
    OK,
    /// Bad read
    BadRead,
    /// Parity error
    ParityError,
    /// Single-bit error
    SingleBitError,
    /// Double-bit error
    DoubleBitError,
    /// Multi-bit error
    MultiBitError,
    /// Nibble error
    NibbleError,
    /// Checksum error
    ChecksumError,
    /// CRC error
    CrcError,
    /// Corrected single-bit error
    CorrectedSingleBitError,
    /// Corrected error
    CorrectedError,
    /// Uncorrectable error
    UncorrectableError,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryErrorTypeData {
    fn from(raw: u8) -> Self {
        MemoryErrorTypeData {
            value: match raw {
                0x01 => MemoryErrorType::Other,
                0x02 => MemoryErrorType::Unknown,
                0x03 => MemoryErrorType::OK,
                0x04 => MemoryErrorType::BadRead,
                0x05 => MemoryErrorType::ParityError,
                0x06 => MemoryErrorType::SingleBitError,
                0x07 => MemoryErrorType::DoubleBitError,
                0x08 => MemoryErrorType::MultiBitError,
                0x09 => MemoryErrorType::NibbleError,
                0x0A => MemoryErrorType::ChecksumError,
                0x0B => MemoryErrorType::CrcError,
                0x0C => MemoryErrorType::CorrectedSingleBitError,
                0x0D => MemoryErrorType::CorrectedError,
                0x0E => MemoryErrorType::UncorrectableError,
                _ => MemoryErrorType::None,
            },
            raw,
        }
    }
}

/// # Memory Error - Error Granularity Data
pub struct MemoryErrorGranularityData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryErrorGranularity] value
    pub value: MemoryErrorGranularity,
}

impl fmt::Debug for MemoryErrorGranularityData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<MemoryErrorGranularityData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for MemoryErrorGranularityData {
    type Target = MemoryErrorGranularity;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Error - Error Granularity
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryErrorGranularity {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Device level
    DeviceLevel,
    /// Memory partition level
    MemoryPartitionLevel,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryErrorGranularityData {
    fn from(raw: u8) -> Self {
        MemoryErrorGranularityData {
            value: match raw {
                0x01 => MemoryErrorGranularity::Other,
                0x02 => MemoryErrorGranularity::Unknown,
                0x03 => MemoryErrorGranularity::DeviceLevel,
                0x04 => MemoryErrorGranularity::MemoryPartitionLevel,
                _ => MemoryErrorGranularity::None,
            },
            raw,
        }
    }
}

/// # Memory Error - Error Operation Data
pub struct MemoryErrorOperationData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryErrorOperation] value
    pub value: MemoryErrorOperation,
}

impl fmt::Debug for MemoryErrorOperationData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<MemoryErrorOperationData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for MemoryErrorOperationData {
    type Target = MemoryErrorOperation;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Error - Error Operation
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryErrorOperation {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Read
    Read,
    /// Write
    Write,
    /// Partial write
    PartialWrite,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryErrorOperationData {
    fn from(raw: u8) -> Self {
        MemoryErrorOperationData {
            value: match raw {
                0x01 => MemoryErrorOperation::Other,
                0x02 => MemoryErrorOperation::Unknown,
                0x03 => MemoryErrorOperation::Read,
                0x04 => MemoryErrorOperation::Write,
                0x05 => MemoryErrorOperation::PartialWrite,
                _ => MemoryErrorOperation::None,
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
        let struct_type18 = vec![
            0x12, 0x17, 0x50, 0x00, 0x03, 0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type18);
        let test_struct = SMBiosMemoryErrorInformation32::new(&parts);

        assert_eq!(*test_struct.error_type().unwrap(), MemoryErrorType::OK);
        assert_eq!(
            *test_struct.error_granularity().unwrap(),
            MemoryErrorGranularity::Unknown
        );
        assert_eq!(
            *test_struct.error_operation().unwrap(),
            MemoryErrorOperation::Unknown
        );
        assert_eq!(test_struct.vendor_syndrome(), Some(0));
        assert_eq!(test_struct.memory_array_error_address(), Some(0x8000_0000));
        assert_eq!(test_struct.device_error_address(), Some(0x8000_0000));
        assert_eq!(test_struct.error_resolution(), Some(0x8000_0000));
    }
}
