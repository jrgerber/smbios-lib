use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{fmt, ops::Deref};
/// # Physical Memory Array (Type 16)
///
/// This structure describes a collection of memory devices that operate together to form a memory address space.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosPhysicalMemoryArray<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosPhysicalMemoryArray<'a> {
    const STRUCT_TYPE: u8 = 16u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosPhysicalMemoryArray<'a> {
    /// Physical location of the Memory Array, whether on
    /// the system board or an add-in board
    pub fn location(&self) -> Option<MemoryArrayLocationData> {
        self.parts
            .get_field_byte(0x04)
            .map(|raw| MemoryArrayLocationData::from(raw))
    }

    /// Function for which the array is used
    pub fn usage(&self) -> Option<MemoryArrayUseData> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| MemoryArrayUseData::from(raw))
    }

    /// Primary hardware error correction or detection
    /// method supported by this memory array
    pub fn memory_error_correction(&self) -> Option<MemoryArrayErrorCorrectionData> {
        self.parts
            .get_field_byte(0x06)
            .map(|raw| MemoryArrayErrorCorrectionData::from(raw))
    }

    /// Maximum memory capacity, in kilobytes, for this array
    ///
    /// If the capacity is not represented in this field, then
    /// the 'extended_maximum_capacity' field should be used.
    ///
    /// Values 2 TB (8000 0000h) or greater must be represented
    /// in the Extended Maximum Capacity field.
    pub fn maximum_capacity(&self) -> Option<MaximumMemoryCapacity> {
        self.parts
            .get_field_dword(0x07)
            .map(|raw| MaximumMemoryCapacity::from(raw))
    }

    /// Handle, or instance number, associated with any
    /// error that was previously detected for the array
    ///
    /// If the system does not provide the error
    /// information structure, the field contains FFFEh;
    /// otherwise, the field contains either FFFFh (if no
    /// error was detected) or the handle of the errorinformation structure.
    pub fn memory_error_information_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x0B)
    }

    /// Number of slots or sockets available for [SMBiosMemoryDevice]s in this array
    ///
    /// This value represents the number of [SMBiosMemoryDevice]
    /// structures that compose this Memory
    /// Array. Each [SMBiosMemoryDevice] has a reference to
    /// the "owning" Memory Array.
    pub fn number_of_memory_devices(&self) -> Option<u16> {
        self.parts.get_field_word(0x0D)
    }

    /// Maximum memory capacity, in bytes, for this array
    ///
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPhysicalMemoryArray<'_>>())
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

impl Serialize for SMBiosPhysicalMemoryArray<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosPhysicalMemoryArray", 8)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("location", &self.location())?;
        state.serialize_field("usage", &self.usage())?;
        state.serialize_field("memory_error_correction", &self.memory_error_correction())?;
        state.serialize_field("maximum_capacity", &self.maximum_capacity())?;
        state.serialize_field(
            "memory_error_information_handle",
            &self.memory_error_information_handle(),
        )?;
        state.serialize_field("number_of_memory_devices", &self.number_of_memory_devices())?;
        state.serialize_field(
            "extended_maximum_capacity",
            &self.extended_maximum_capacity(),
        )?;
        state.end()
    }
}

/// # Memory Array - Location Data
pub struct MemoryArrayLocationData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryArrayLocation] value
    pub value: MemoryArrayLocation,
}

impl fmt::Debug for MemoryArrayLocationData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<MemoryArrayLocationData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for MemoryArrayLocationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryArrayLocationData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for MemoryArrayLocationData {
    type Target = MemoryArrayLocation;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Array - Location
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemoryArrayLocation {
    /// Other
    Other = 0x01,
    /// Unknown
    Unknown = 0x02,
    /// System board or motherboard
    SystemBoardOrMotherboard = 0x03,
    /// ISA add-on card
    IsaAddOnCard = 0x04,
    /// EISA add-on card
    EisaAddOnCard = 0x05,
    /// PCI add-on card
    PciAddOnCard = 0x06,
    /// MCA add-on card
    McaAddOnCard = 0x07,
    /// PCMCIA add-on card
    PcmciaAddOnCard = 0x08,
    /// Proprietary add-on card
    ProprietaryAddOnCard = 0x09,
    /// NuBus
    NuBus = 0x0A,
    /// PC-98/C20 add-on card
    PC98C20AddOnCard = 0xA0,
    /// PC-98/C24 add-on card
    PC98C24AddOnCard = 0xA1,
    /// PC-98/E add-on card
    PC98EAddOnCard = 0xA2,
    /// PC-98/Local bus add-on card
    PC98LocalBusAddOnCard = 0xA3,
    /// CXL add-on card
    CxlFlexbus10AddOnCard = 0xA4,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryArrayLocationData {
    fn from(raw: u8) -> Self {
        MemoryArrayLocationData {
            value: match raw {
                0x01 => MemoryArrayLocation::Other,
                0x02 => MemoryArrayLocation::Unknown,
                0x03 => MemoryArrayLocation::SystemBoardOrMotherboard,
                0x04 => MemoryArrayLocation::IsaAddOnCard,
                0x05 => MemoryArrayLocation::EisaAddOnCard,
                0x06 => MemoryArrayLocation::PciAddOnCard,
                0x07 => MemoryArrayLocation::McaAddOnCard,
                0x08 => MemoryArrayLocation::PcmciaAddOnCard,
                0x09 => MemoryArrayLocation::ProprietaryAddOnCard,
                0x0A => MemoryArrayLocation::NuBus,
                0xA0 => MemoryArrayLocation::PC98C20AddOnCard,
                0xA1 => MemoryArrayLocation::PC98C24AddOnCard,
                0xA2 => MemoryArrayLocation::PC98EAddOnCard,
                0xA3 => MemoryArrayLocation::PC98LocalBusAddOnCard,
                0xA4 => MemoryArrayLocation::CxlFlexbus10AddOnCard,
                _ => MemoryArrayLocation::None,
            },
            raw,
        }
    }
}

/// # Memory Array - Use Data
pub struct MemoryArrayUseData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryArrayUse] value
    pub value: MemoryArrayUse,
}

impl fmt::Debug for MemoryArrayUseData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<MemoryArrayUseData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for MemoryArrayUseData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryArrayUseData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for MemoryArrayUseData {
    type Target = MemoryArrayUse;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Array - Use
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemoryArrayUse {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// System memory
    SystemMemory,
    /// Video memory
    VideoMemory,
    /// Flash memory
    FlashMemory,
    /// Non-volatile RAM
    NonVolatileRam,
    /// Cache memory
    CacheMemory,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryArrayUseData {
    fn from(raw: u8) -> Self {
        MemoryArrayUseData {
            value: match raw {
                0x01 => MemoryArrayUse::Other,
                0x02 => MemoryArrayUse::Unknown,
                0x03 => MemoryArrayUse::SystemMemory,
                0x04 => MemoryArrayUse::VideoMemory,
                0x05 => MemoryArrayUse::FlashMemory,
                0x06 => MemoryArrayUse::NonVolatileRam,
                0x07 => MemoryArrayUse::CacheMemory,
                _ => MemoryArrayUse::None,
            },
            raw,
        }
    }
}

/// # Memory Array - Error Correction Types Data
pub struct MemoryArrayErrorCorrectionData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryArrayErrorCorrection] value
    pub value: MemoryArrayErrorCorrection,
}

impl fmt::Debug for MemoryArrayErrorCorrectionData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<MemoryArrayErrorCorrectionData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for MemoryArrayErrorCorrectionData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryArrayErrorCorrectionData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for MemoryArrayErrorCorrectionData {
    type Target = MemoryArrayErrorCorrection;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Array - Error Correction Types
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemoryArrayErrorCorrection {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// No Error Correction
    NoCorrection,
    /// Parity
    Parity,
    /// Single-bit ECC
    SingleBitEcc,
    /// Multi-bit ECC
    MultiBitEcc,
    /// CRC
    Crc,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryArrayErrorCorrectionData {
    fn from(raw: u8) -> Self {
        MemoryArrayErrorCorrectionData {
            value: match raw {
                0x01 => MemoryArrayErrorCorrection::Other,
                0x02 => MemoryArrayErrorCorrection::Unknown,
                0x03 => MemoryArrayErrorCorrection::NoCorrection,
                0x04 => MemoryArrayErrorCorrection::Parity,
                0x05 => MemoryArrayErrorCorrection::SingleBitEcc,
                0x06 => MemoryArrayErrorCorrection::MultiBitEcc,
                0x07 => MemoryArrayErrorCorrection::Crc,
                _ => MemoryArrayErrorCorrection::None,
            },
            raw,
        }
    }
}

/// # Maximum memory capacity, in kilobytes, for this array
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MaximumMemoryCapacity {
    /// Maximum memory capacity in Kilobytes
    Kilobytes(u32),
    /// Use `extended_maximum_capacity` to retrieve the maximum capacity
    SeeExtendedMaximumCapacity,
}

impl From<u32> for MaximumMemoryCapacity {
    fn from(raw: u32) -> Self {
        match raw {
            0x8000_0000 => MaximumMemoryCapacity::SeeExtendedMaximumCapacity,
            _ => MaximumMemoryCapacity::Kilobytes(raw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type16 = vec![
            0x10, 0x17, 0x3E, 0x00, 0x03, 0x03, 0x05, 0x00, 0x00, 0x00, 0x60, 0xFE, 0xFF, 0x04,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type16);
        let test_struct = SMBiosPhysicalMemoryArray::new(&parts);

        assert_eq!(
            *test_struct.location().unwrap(),
            MemoryArrayLocation::SystemBoardOrMotherboard
        );
        assert_eq!(*test_struct.usage().unwrap(), MemoryArrayUse::SystemMemory);
        assert_eq!(
            *test_struct.memory_error_correction().unwrap(),
            MemoryArrayErrorCorrection::SingleBitEcc
        );
        match test_struct.maximum_capacity().unwrap() {
            MaximumMemoryCapacity::Kilobytes(kb) => assert_eq!(kb, 0x6000_0000),
            MaximumMemoryCapacity::SeeExtendedMaximumCapacity => panic!("expected kb"),
        }
        assert_eq!(test_struct.memory_error_information_handle(), Some(65534));
        assert_eq!(test_struct.number_of_memory_devices(), Some(4));
        assert_eq!(test_struct.extended_maximum_capacity(), Some(0));
    }
}
