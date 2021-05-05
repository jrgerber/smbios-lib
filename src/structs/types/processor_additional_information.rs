use crate::core::{Handle, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

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
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosProcessorAdditionalInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosProcessorAdditionalInformation<'a> {
    const STRUCT_TYPE: u8 = 44u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosProcessorAdditionalInformation<'a> {
    /// Offset of the ProcessorSpecificBlock field.
    const PROCESSOR_SPECIFIC_BLOCK_OFFSET: usize = 0x06usize;

    /// Handle, or instance number, associated with the
    /// [SMBiosProcessorInformation] structure (SMBIOS type 4) which the
    /// Processor Additional Information structure describes.
    pub fn referenced_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x04)
    }

    /// Processor-Specific Block
    pub fn processor_specific_block(&self) -> Option<ProcessorSpecificBlock<'_>> {
        ProcessorSpecificBlock::new(self)
    }
}

impl fmt::Debug for SMBiosProcessorAdditionalInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<
            SMBiosProcessorAdditionalInformation<'_>,
        >())
        .field("header", &self.parts.header)
        .field("referenced_handle", &self.referenced_handle())
        .field("processor_specific_block", &self.processor_specific_block())
        .finish()
    }
}

impl Serialize for SMBiosProcessorAdditionalInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosProcessorAdditionalInformation", 3)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("referenced_handle", &self.referenced_handle())?;
        state.serialize_field("processor_specific_block", &self.processor_specific_block())?;
        state.end()
    }
}

/// # Processor Specific Block contained within [SMBiosProcessorAdditionalInformation]
pub struct ProcessorSpecificBlock<'a> {
    /// Raw byte slice for this processor specific block
    pub raw: &'a [u8],
}

impl<'a> ProcessorSpecificBlock<'a> {
    /// 'block_length' offset
    const BLOCK_LENGTH_OFFSET: usize = 0x00usize;
    /// 'processor_type' offset
    const PROCESSOR_TYPE_OFFSET: usize = 0x01usize;
    /// 'processor_specific_data' offset
    const PROCESSOR_SPECIFIC_DATA_OFFSET: usize = 0x02usize;

    fn new(additional_information: &'a SMBiosProcessorAdditionalInformation<'a>) -> Option<Self> {
        additional_information
            .parts()
            .get_field_byte(
                SMBiosProcessorAdditionalInformation::PROCESSOR_SPECIFIC_BLOCK_OFFSET
                    + Self::BLOCK_LENGTH_OFFSET,
            )
            .and_then(|block_length| {
                additional_information
                    .parts()
                    .get_field_data(
                        SMBiosProcessorAdditionalInformation::PROCESSOR_SPECIFIC_BLOCK_OFFSET
                            + Self::BLOCK_LENGTH_OFFSET,
                        block_length as usize
                            + SMBiosProcessorAdditionalInformation::PROCESSOR_SPECIFIC_BLOCK_OFFSET
                            + Self::PROCESSOR_SPECIFIC_DATA_OFFSET,
                    )
                    .map(|raw| Self { raw })
            })
    }

    /// Length of 'processor_specific_data'
    pub fn block_length(&self) -> u8 {
        self.raw[Self::BLOCK_LENGTH_OFFSET]
    }

    /// The processor architecture delineated by this 'ProcessorSpecificBlock'.
    pub fn processor_type(&self) -> ProcessorArchitectureTypeData {
        ProcessorArchitectureTypeData::from(self.raw[Self::PROCESSOR_TYPE_OFFSET])
    }

    /// Offset of the field within the structure referenced by the
    /// _Referenced Handle_ for which additional information is provided
    pub fn processor_specific_data(&self) -> &'a [u8] {
        &self.raw[Self::PROCESSOR_SPECIFIC_DATA_OFFSET..]
    }
}

impl fmt::Debug for ProcessorSpecificBlock<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorSpecificBlock<'_>>())
            .field("block_length", &self.block_length())
            .field("processor_type", &self.processor_type())
            .field("processor_specific_data", &self.processor_specific_data())
            .finish()
    }
}

impl Serialize for ProcessorSpecificBlock<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorSpecificBlock", 3)?;
        state.serialize_field("block_length", &self.block_length())?;
        state.serialize_field("processor_type", &self.processor_type())?;
        state.serialize_field("processor_specific_data", &self.processor_specific_data())?;
        state.end()
    }
}

/// # Processor Architecture Types Data
pub struct ProcessorArchitectureTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ProcessorArchitectureType] value
    pub value: ProcessorArchitectureType,
}

impl fmt::Debug for ProcessorArchitectureTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorArchitectureTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ProcessorArchitectureTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorArchitectureTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ProcessorArchitectureTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ProcessorArchitectureType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ProcessorArchitectureTypeData {
    type Target = ProcessorArchitectureType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Processor Architecture Types
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ProcessorArchitectureType {
    /// IA32 (x86)
    IA32,
    /// x64 (x86-64, Intel64, AMD64, EM64T)
    X64,
    /// Intel® Itanium® architecture
    IntelItanium,
    /// 32-bit ARM (Aarch32)
    Arm32Bit,
    /// 64-bit ARM (Aarch64)
    Arm64Bit,
    /// 32-bit RISC-V (RV32)
    RiscV32Bit,
    /// 64-bit RISC-V (RV64)
    RiscV64Bit,
    /// 128-bit RISC-V (RV128)
    RiscV128Bit,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ProcessorArchitectureTypeData {
    fn from(raw: u8) -> Self {
        ProcessorArchitectureTypeData {
            value: match raw {
                0x01 => ProcessorArchitectureType::IA32,
                0x02 => ProcessorArchitectureType::X64,
                0x03 => ProcessorArchitectureType::IntelItanium,
                0x04 => ProcessorArchitectureType::Arm32Bit,
                0x05 => ProcessorArchitectureType::Arm64Bit,
                0x06 => ProcessorArchitectureType::RiscV32Bit,
                0x07 => ProcessorArchitectureType::RiscV64Bit,
                0x08 => ProcessorArchitectureType::RiscV128Bit,
                _ => ProcessorArchitectureType::None,
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
        let block_length = 3u8;
        let processor_type = 0x07u8; // RiscV64Bit

        let struct_type44 = vec![
            44u8,
            6 + block_length + 2,
            0x2E,
            0x00, // header
            0x08,
            0x09, // referenced handle
            block_length,
            processor_type,
            0x03,
            0x02,
            0x01,
            0x00,
            0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type44);
        let test_struct = SMBiosProcessorAdditionalInformation::new(&parts);

        assert_eq!(*test_struct.referenced_handle().unwrap(), 0x0908);
        let processor_specific_block = test_struct.processor_specific_block().unwrap();
        assert_eq!(processor_specific_block.block_length(), 3);
        assert_eq!(
            *processor_specific_block.processor_type(),
            ProcessorArchitectureType::RiscV64Bit
        );
        assert_eq!(
            processor_specific_block.processor_specific_data(),
            &[0x03, 0x02, 0x01]
        );
    }
}
