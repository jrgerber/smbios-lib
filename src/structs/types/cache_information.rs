use crate::core::{strings::*, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

/// # Cache Information (Type 7)
///
/// This structure defines the attributes of CPU cache device in the
/// system. One structure is specified for each such device, whether the device is internal to or external to
/// the CPU module. Cache modules can be associated with a processor structure in one or two ways
/// depending on the SMBIOS version
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosCacheInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosCacheInformation<'a> {
    const STRUCT_TYPE: u8 = 7u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosCacheInformation<'a> {
    /// String number for reference designation
    pub fn socket_designation(&self) -> SMBiosString {
        self.parts.get_field_string(0x04)
    }

    /// Bit fields describing the cache configuration
    pub fn cache_configuration(&self) -> Option<CacheConfiguaration> {
        self.parts
            .get_field_word(0x05)
            .map(|raw| CacheConfiguaration::from(raw))
    }

    /// Maximum size that can be installed
    pub fn maximum_cache_size(&self) -> Option<CacheMemorySize> {
        self.parts
            .get_field_word(0x07)
            .map(|raw| CacheMemorySize::from(raw))
            // When *_cache_size_2 is not present, SeeCacheSize2 is not considered sentinel
            // and thus should be converted to Kilobytes
            .map(|size| {
                if size == CacheMemorySize::SeeCacheSize2
                        && self.maximum_cache_size_2().is_none() {
                    CacheMemorySize::Kilobytes(0x7FFFu64 * 64)
                } else {
                    size
                }
            })
    }

    /// Same format as Max Cache Size field; set to 0 if no cache is installed
    pub fn installed_size(&self) -> Option<CacheMemorySize> {
        self.parts
            .get_field_word(0x09)
            .map(|raw| CacheMemorySize::from(raw))
            // When *_cache_size_2 is not present, SeeCacheSize2 is not considered sentinel
            // and thus should be converted to Kilobytes
            .map(|size| {
                if size == CacheMemorySize::SeeCacheSize2
                        && self.installed_cache_size_2().is_none() {
                    CacheMemorySize::Kilobytes(0x7FFFu64 * 64)
                } else {
                    size
                }
            })
    }

    /// Supported SRAM type
    pub fn supported_sram_type(&self) -> Option<SramTypes> {
        self.parts
            .get_field_word(0x0B)
            .map(|raw| SramTypes::from(raw))
    }

    /// Current SRAM type
    pub fn current_sram_type(&self) -> Option<SramTypes> {
        self.parts
            .get_field_word(0x0D)
            .map(|raw| SramTypes::from(raw))
    }

    /// Cache module speed, in nanoseconds.
    /// The value is 0 if the speed is unknown.
    pub fn cache_speed(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    /// Error-correction scheme supported by this cache component
    pub fn error_correction_type(&self) -> Option<ErrorCorrectionTypeData> {
        self.parts
            .get_field_byte(0x10)
            .map(|raw| ErrorCorrectionTypeData::from(raw))
    }

    /// Logical type of cache
    pub fn system_cache_type(&self) -> Option<SystemCacheTypeData> {
        self.parts
            .get_field_byte(0x11)
            .map(|raw| SystemCacheTypeData::from(raw))
    }

    /// Associativity of the cache
    pub fn associativity(&self) -> Option<CacheAssociativityData> {
        self.parts
            .get_field_byte(0x12)
            .map(|raw| CacheAssociativityData::from(raw))
    }

    /// Maximum cache size
    pub fn maximum_cache_size_2(&self) -> Option<CacheMemorySize> {
        self.parts
            .get_field_dword(0x13)
            .map(|raw| CacheMemorySize::from(raw))
    }

    /// Installed cache size
    pub fn installed_cache_size_2(&self) -> Option<CacheMemorySize> {
        self.parts
            .get_field_dword(0x17)
            .map(|raw| CacheMemorySize::from(raw))
    }
}

impl fmt::Debug for SMBiosCacheInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosCacheInformation<'_>>())
            .field("header", &self.parts.header)
            .field("socket_designation", &self.socket_designation())
            .field("cache_configuration", &self.cache_configuration())
            .field("maximum_cache_size", &self.maximum_cache_size())
            .field("installed_size", &self.installed_size())
            .field("supported_sram_type", &self.supported_sram_type())
            .field("current_sram_type", &self.current_sram_type())
            .field("cache_speed", &self.cache_speed())
            .field("error_correction_type", &self.error_correction_type())
            .field("system_cache_type", &self.system_cache_type())
            .field("associativity", &self.associativity())
            .field("maximum_cache_size_2", &self.maximum_cache_size_2())
            .field("installed_cache_size_2", &self.installed_cache_size_2())
            .finish()
    }
}

impl Serialize for SMBiosCacheInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosCacheInformation", 13)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("socket_designation", &self.socket_designation())?;
        state.serialize_field("cache_configuration", &self.cache_configuration())?;
        state.serialize_field("maximum_cache_size", &self.maximum_cache_size())?;
        state.serialize_field("installed_size", &self.installed_size())?;
        state.serialize_field("supported_sram_type", &self.supported_sram_type())?;
        state.serialize_field("current_sram_type", &self.current_sram_type())?;
        state.serialize_field("cache_speed", &self.cache_speed())?;
        state.serialize_field("error_correction_type", &self.error_correction_type())?;
        state.serialize_field("system_cache_type", &self.system_cache_type())?;
        state.serialize_field("associativity", &self.associativity())?;
        state.serialize_field("maximum_cache_size_2", &self.maximum_cache_size_2())?;
        state.serialize_field("installed_cache_size_2", &self.installed_cache_size_2())?;
        state.end()
    }
}

/// # Maximum memory capacity, in kilobytes, for this cache item
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum CacheMemorySize {
    /// Maximum memory capacity in Kilobytes
    Kilobytes(u64),
    /// Use `maximum_cache_size_2` to retrieve the maximum capacity or `installed_cache_size_2` to retrieve the installed size.
    SeeCacheSize2,
}

impl From<u16> for CacheMemorySize {
    fn from(raw: u16) -> Self {
        match raw {
            0xFFFF => CacheMemorySize::SeeCacheSize2,
            // When bit 15 is set, the size units of the remaining bits are 64k
            _ if raw & 0x8000 == 0x8000 => CacheMemorySize::Kilobytes((raw & 0x7FFF) as u64 * 64),
            _ => CacheMemorySize::Kilobytes(raw as u64),
        }
    }
}

impl From<u32> for CacheMemorySize {
    fn from(raw: u32) -> Self {
        match raw {
            // When bit 31 is set, the size units of the remaining bits are 64k
            _ if raw & 0x80000000 == 0x80000000 => {
                CacheMemorySize::Kilobytes((raw & 0x7FFFFFFF) as u64 * 64)
            }
            _ => CacheMemorySize::Kilobytes(raw as u64),
        }
    }
}

/// # Cache Associativity Data
pub struct CacheAssociativityData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [CacheAssociativity] value
    pub value: CacheAssociativity,
}

impl fmt::Debug for CacheAssociativityData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<CacheAssociativityData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for CacheAssociativityData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CacheAssociativityData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for CacheAssociativityData {
    type Target = CacheAssociativity;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Cache Associativity
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum CacheAssociativity {
    /// Other
    Other = 0x01,
    /// Unknown
    Unknown = 0x02,
    /// Direct Mapped
    DirectMapped = 0x03,
    /// 2-way Set-Associative
    SetAssociative2Way = 0x04,
    /// 4-way Set-Associative
    SetAssociative4Way = 0x05,
    /// Fully Associative
    FullyAssociative = 0x06,
    /// 8-way Set-Associative
    SetAssociative8Way = 0x07,
    /// 16-way Set-Associative
    SetAssociative16Way = 0x08,
    /// 12-way Set-Associative
    SetAssociative12Way = 0x09,
    /// 24-way Set-Associative
    SetAssociative24Way = 0x0A,
    /// 32-way Set-Associative
    SetAssociative32Way = 0x0B,
    /// 48-way Set-Associative
    SetAssociative48Way = 0x0C,
    /// 64-way Set-Associative
    SetAssociative64Way = 0x0D,
    /// 20-way Set-Associative
    SetAssociative20Way = 0x0E,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for CacheAssociativityData {
    fn from(raw: u8) -> Self {
        CacheAssociativityData {
            value: match raw {
                0x01 => CacheAssociativity::Other,
                0x02 => CacheAssociativity::Unknown,
                0x03 => CacheAssociativity::DirectMapped,
                0x04 => CacheAssociativity::SetAssociative2Way,
                0x05 => CacheAssociativity::SetAssociative4Way,
                0x06 => CacheAssociativity::FullyAssociative,
                0x07 => CacheAssociativity::SetAssociative8Way,
                0x08 => CacheAssociativity::SetAssociative16Way,
                0x09 => CacheAssociativity::SetAssociative12Way,
                0x0A => CacheAssociativity::SetAssociative24Way,
                0x0B => CacheAssociativity::SetAssociative32Way,
                0x0C => CacheAssociativity::SetAssociative48Way,
                0x0D => CacheAssociativity::SetAssociative64Way,
                0x0E => CacheAssociativity::SetAssociative20Way,
                _ => CacheAssociativity::None,
            },
            raw,
        }
    }
}

/// # System Cache Type Data
pub struct SystemCacheTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [SystemCacheType] value
    pub value: SystemCacheType,
}

impl fmt::Debug for SystemCacheTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SystemCacheTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for SystemCacheTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SystemCacheTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for SystemCacheTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            SystemCacheType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for SystemCacheTypeData {
    type Target = SystemCacheType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # System Cache Type
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum SystemCacheType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Instruction
    Instruction,
    /// Data
    Data,
    /// Unified
    Unified,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for SystemCacheTypeData {
    fn from(raw: u8) -> Self {
        SystemCacheTypeData {
            value: match raw {
                0x01 => SystemCacheType::Other,
                0x02 => SystemCacheType::Unknown,
                0x03 => SystemCacheType::Instruction,
                0x04 => SystemCacheType::Data,
                0x05 => SystemCacheType::Unified,
                _ => SystemCacheType::None,
            },
            raw,
        }
    }
}

/// # System Cache Error Correction Type Data
pub struct ErrorCorrectionTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ErrorCorrectionType] value
    pub value: ErrorCorrectionType,
}

impl fmt::Debug for ErrorCorrectionTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ErrorCorrectionTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ErrorCorrectionTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ErrorCorrectionTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ErrorCorrectionTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ErrorCorrectionType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ErrorCorrectionTypeData {
    type Target = ErrorCorrectionType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # System Cache Error Correction Type
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ErrorCorrectionType {
    /// Other
    Other = 0x01,
    /// Unknown
    Unknown = 0x02,
    /// None (No Correction)
    NoCorrection = 0x03,
    /// Parity
    Parity = 0x04,
    /// Single-bit ECC
    SingleBitEcc = 0x05,
    /// Multi-bit ECC
    MultiBitEcc = 0x06,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ErrorCorrectionTypeData {
    fn from(raw: u8) -> Self {
        ErrorCorrectionTypeData {
            value: match raw {
                0x01 => ErrorCorrectionType::Other,
                0x02 => ErrorCorrectionType::Unknown,
                0x03 => ErrorCorrectionType::NoCorrection,
                0x04 => ErrorCorrectionType::Parity,
                0x05 => ErrorCorrectionType::SingleBitEcc,
                0x06 => ErrorCorrectionType::MultiBitEcc,
                _ => ErrorCorrectionType::None,
            },
            raw,
        }
    }
}

/// # System Cache SRAM Types
#[derive(PartialEq, Eq)]
pub struct SramTypes {
    /// Raw value
    pub raw: u16,
}

impl Deref for SramTypes {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for SramTypes {
    fn from(raw: u16) -> Self {
        SramTypes { raw }
    }
}

impl SramTypes {
    /// Other
    pub fn other(&self) -> bool {
        self.raw & 0x0001 == 0x0001
    }

    /// Unknown
    pub fn unknown(&self) -> bool {
        self.raw & 0x0002 == 0x0002
    }

    /// Non-Burst
    pub fn non_burst(&self) -> bool {
        self.raw & 0x0004 == 0x0004
    }

    /// Burst
    pub fn burst(&self) -> bool {
        self.raw & 0x0008 == 0x0008
    }

    /// Pipeline Burst
    pub fn pipeline_burst(&self) -> bool {
        self.raw & 0x0010 == 0x0010
    }

    /// Synchronous
    pub fn synchronous(&self) -> bool {
        self.raw & 0x0020 == 0x0020
    }

    /// Asynchronous
    pub fn asynchronous(&self) -> bool {
        self.raw & 0x0040 == 0x0040
    }
}

impl fmt::Debug for SramTypes {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SramTypes>())
            .field("raw", &self.raw)
            .field("other", &self.other())
            .field("unknown", &self.unknown())
            .field("non_burst", &self.non_burst())
            .field("burst", &self.burst())
            .field("pipeline_burst", &self.pipeline_burst())
            .field("synchronous", &self.synchronous())
            .field("asynchronous", &self.asynchronous())
            .finish()
    }
}

impl Serialize for SramTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SramTypes", 8)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("other", &self.other())?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("non_burst", &self.non_burst())?;
        state.serialize_field("burst", &self.burst())?;
        state.serialize_field("pipeline_burst", &self.pipeline_burst())?;
        state.serialize_field("synchronous", &self.synchronous())?;
        state.serialize_field("asynchronous", &self.asynchronous())?;
        state.end()
    }
}

/// # System Cache Configuration
#[derive(PartialEq, Eq)]
pub struct CacheConfiguaration {
    /// Raw value
    pub raw: u16,
}

impl Deref for CacheConfiguaration {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for CacheConfiguaration {
    fn from(raw: u16) -> Self {
        CacheConfiguaration { raw }
    }
}

impl CacheConfiguaration {
    /// Cache Level (1 through 8)
    pub fn cache_level(&self) -> u8 {
        // [bits 0, 1, 2] Cache Level – 1 through 8
        // (For example, an L1 cache would use value
        // 000b and an L3 cache would use 010b.
        (self.raw & 0x0007) as u8 + 1
    }

    /// Cache Socketed (e.g. Cache on a Stick)
    ///
    /// true - socketed
    /// false - non-socketed
    pub fn cache_socketed(&self) -> bool {
        // [bit 3] Cache Socketed
        //     1b - Socketed
        //     0b - Not socketed
        self.raw & 0x0008 == 0x0008
    }

    /// Cache Location (relative to the CPU module)
    pub fn location(&self) -> CacheLocation {
        // [bits 5, 6] Location, relative to the CPU module:
        //     00b - Internal
        //     01b - External
        //     10b - Reserved
        //     11b - Unknown
        match self.raw & 0x0060 {
            0x0000 => CacheLocation::Internal,
            0x0020 => CacheLocation::External,
            0x0040 => CacheLocation::Reserved,
            0x0060 => CacheLocation::Unknown,
            _ => panic!("Impossible condition"),
        }
    }

    /// Cache Enabled (at boot time)
    pub fn enabled_at_boot(&self) -> bool {
        // [bit 7]
        self.raw & 0x0080 == 0x0080
    }

    /// Cache Operational Mode
    pub fn operational_mode(&self) -> CacheOperationalMode {
        // [bits 8, 9] Operational Mode
        //     00b - Write Through
        //     01b - Write Back
        //     10b - Varies with Memory Address
        //     11b - Unknown
        match self.raw & 0x0300 {
            0x0000 => CacheOperationalMode::WriteThrough,
            0x0100 => CacheOperationalMode::WriteBack,
            0x0200 => CacheOperationalMode::VariesWithMemoryAddress,
            0x0300 => CacheOperationalMode::Unknown,
            _ => panic!("Impossible condition"),
        }
    }
}

impl fmt::Debug for CacheConfiguaration {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<CacheConfiguaration>())
            .field("raw", &self.raw)
            .field("cache_level", &self.cache_level())
            .field("cache_socketed", &self.cache_socketed())
            .field("location", &self.location())
            .field("enabled_at_boot", &self.enabled_at_boot())
            .field("operational_mode", &self.operational_mode())
            .finish()
    }
}

impl Serialize for CacheConfiguaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CacheConfiguaration", 6)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("cache_level", &self.cache_level())?;
        state.serialize_field("cache_socketed", &self.cache_socketed())?;
        state.serialize_field("location", &self.location())?;
        state.serialize_field("enabled_at_boot", &self.enabled_at_boot())?;
        state.serialize_field("operational_mode", &self.operational_mode())?;
        state.end()
    }
}
/// # System Cache Location (relative to the CPU module)
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum CacheLocation {
    /// Internal to the CPU
    Internal,
    /// External to the CPU
    External,
    /// Reserved
    Reserved,
    /// Location Unknown
    Unknown,
}

/// # System Cache Operational Mode
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum CacheOperationalMode {
    /// Write Through
    WriteThrough = 0x00,
    /// Write Back
    WriteBack = 0x01,
    /// Varies with Memory Address
    VariesWithMemoryAddress = 0x02,
    /// Unknown Operational Mode
    Unknown = 0x03,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type7 = vec![
            0x07, 0x1B, 0x03, 0x00, 0x01, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0x10, 0x00, 0x10,
            0x00, 0x01, 0x06, 0x05, 0x07, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x4C,
            0x31, 0x20, 0x2D, 0x20, 0x43, 0x61, 0x63, 0x68, 0x65, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type7);
        let test_struct = SMBiosCacheInformation::new(&parts);

        let cache_configuration = test_struct.cache_configuration().unwrap();
        assert_eq!(cache_configuration.cache_level(), 1);
        assert!(!cache_configuration.cache_socketed());
        assert_eq!(cache_configuration.location(), CacheLocation::Internal);
        assert!(cache_configuration.enabled_at_boot());
        assert_eq!(
            cache_configuration.operational_mode(),
            CacheOperationalMode::WriteBack
        );

        let cache_size = test_struct.maximum_cache_size().unwrap();
        assert_eq!(cache_size, CacheMemorySize::Kilobytes(384));

        println!("{:?}", test_struct);
    }

    #[test]
    fn memory_size_parsing_test() {
        assert_eq!(
            CacheMemorySize::from(0x8200u16),
            CacheMemorySize::Kilobytes(32768)
        );
        assert_eq!(
            CacheMemorySize::from(0x00000200u32),
            CacheMemorySize::Kilobytes(512)
        );
        assert_eq!(
            CacheMemorySize::from(0x80000200u32),
            CacheMemorySize::Kilobytes(32768)
        );
        assert_eq!(
            CacheMemorySize::from(0xFFFFFFFFu32),
            CacheMemorySize::Kilobytes(2u64.pow(37) - 64)
        );
    }
}
