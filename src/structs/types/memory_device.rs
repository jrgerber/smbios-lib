use crate::core::{strings::*, Handle, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use core::ops::Deref;

/// # Memory Device (Type 17)
///
/// This structure describes a single memory device that is part of a larger [super::SMBiosPhysicalMemoryArray] (Type 16) structure.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryDevice<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryDevice<'a> {
    const STRUCT_TYPE: u8 = 17u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosMemoryDevice<'a> {
    /// Handle, or instance number, associated with the
    /// [super::SMBiosPhysicalMemoryArray] to which this device belongs
    pub fn physical_memory_array_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x04)
    }

    /// Handle, or instance number, associated with any
    /// error that was previously detected for the device
    /// If the system does not provide the error information
    /// structure, the field contains FFFEh; otherwise, the
    /// field contains either FFFFh (if no error was
    /// detected) or the handle of the error-information
    /// structure ([super::SMBiosMemoryErrorInformation32] or
    /// [super::SMBiosMemoryErrorInformation64]).
    pub fn memory_error_information_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x06)
    }

    /// Total width, in bits, of this memory device, including
    /// any check or error-correction bits
    /// If there are no error-correction bits, this value
    /// should be equal to Data Width. If the width is
    /// unknown, the field is set to FFFFh.
    pub fn total_width(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Data width, in bits, of this memory device
    /// A Data Width of 0 and a Total Width of 8 indicates
    /// that the device is being used solely to provide 8
    /// error-correction bits. If the width is unknown, the
    /// field is set to FFFFh.
    pub fn data_width(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Size of the memory device
    pub fn size(&self) -> Option<MemorySize> {
        self.parts
            .get_field_word(0x0C)
            .map(|raw| MemorySize::from(raw))
    }

    /// Implementation form factor for this memory device
    pub fn form_factor(&self) -> Option<MemoryFormFactorData> {
        self.parts
            .get_field_byte(0x0E)
            .map(|raw| MemoryFormFactorData::from(raw))
    }

    /// Identifies when the Memory Device is one of a set
    /// of Memory Devices that must be populated with all
    /// devices of the same type and size, and the set to
    /// which this device belongs
    /// A value of 0 indicates that the device is not part of a
    /// set; a value of FFh indicates that the attribute is
    /// unknown.
    /// NOTE: A Device Set number must be unique within the
    /// context of the Memory Array containing this Memory
    /// Device.
    pub fn device_set(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    /// Identifies the physically-labeled socket or board position where
    /// the memory device is located
    /// EXAMPLE: “SIMM 3”
    pub fn device_locator(&self) -> SMBiosString {
        self.parts.get_field_string(0x10)
    }

    /// Identifies the physically labeled bank where the memory device is located
    /// EXAMPLE: “Bank 0” or “A”
    pub fn bank_locator(&self) -> SMBiosString {
        self.parts.get_field_string(0x11)
    }

    /// Type of memory used in this device
    pub fn memory_type(&self) -> Option<MemoryDeviceTypeData> {
        self.parts
            .get_field_byte(0x12)
            .map(|raw| MemoryDeviceTypeData::from(raw))
    }

    /// Additional detail on the memory device type
    pub fn type_detail(&self) -> Option<MemoryTypeDetails> {
        self.parts
            .get_field_word(0x13)
            .map(|raw| MemoryTypeDetails::from(raw))
    }

    /// The maximum capable speed of the
    /// device, in megatransfers per second (MT/s).
    pub fn speed(&self) -> Option<MemorySpeed> {
        self.parts
            .get_field_word(0x15)
            .map(|raw| MemorySpeed::from(raw))
    }

    /// The manufacturer of this memory device
    pub fn manufacturer(&self) -> SMBiosString {
        self.parts.get_field_string(0x17)
    }

    /// The serial number of this memory device.
    /// This value is set by the manufacturer and normally
    /// is not changeable.
    pub fn serial_number(&self) -> SMBiosString {
        self.parts.get_field_string(0x18)
    }

    /// The asset tag of this memory device
    pub fn asset_tag(&self) -> SMBiosString {
        self.parts.get_field_string(0x19)
    }

    /// The part number of this memory device.
    /// This value is set by the manufacturer and normally
    /// is not changeable.
    pub fn part_number(&self) -> SMBiosString {
        self.parts.get_field_string(0x1A)
    }

    /// Bits 7-4: reserved
    /// Bits 3-0: rank
    /// Value=0 for unknown rank information
    pub fn attributes(&self) -> Option<u8> {
        self.parts.get_field_byte(0x1B)
    }

    /// Extended size of the memory device in MB (complements
    /// the Size field at offset 0Ch)
    pub fn extended_size(&self) -> Option<MemorySizeExtended> {
        self.parts
            .get_field_dword(0x1C)
            .map(|raw| MemorySizeExtended::from(raw))
    }

    /// Identifies the configured speed of the memory
    /// device, in megatransfers per second (MT/s). See
    /// 7.18.4 for details.
    /// 0000h = the speed is unknown
    /// FFFFh = the speed is 65,535 MT/s or greater,
    /// and the actual speed is stored in the Extended
    /// Configured Memory Speed field
    pub fn configured_memory_speed(&self) -> Option<MemorySpeed> {
        self.parts
            .get_field_word(0x20)
            .map(|raw| MemorySpeed::from(raw))
    }

    /// Minimum operating voltage for this device, in
    /// millivolts
    /// If the value is 0, the voltage is unknown.
    pub fn minimum_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x22)
    }

    /// Maximum operating voltage for this device, in
    /// millivolts
    /// If the value is 0, the voltage is unknown.
    pub fn maximum_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x24)
    }

    /// Configured voltage for this device, in millivolts
    /// If the value is 0, the voltage is unknown.
    pub fn configured_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x26)
    }

    /// Memory technology type for this memory device.
    pub fn memory_technology(&self) -> Option<MemoryDeviceTechnologyData> {
        self.parts
            .get_field_byte(0x28)
            .map(|raw| MemoryDeviceTechnologyData::from(raw))
    }

    /// The operating modes supported by this memory device.
    pub fn memory_operating_mode_capability(&self) -> Option<MemoryOperatingModeCapabilities> {
        self.parts
            .get_field_word(0x29)
            .map(|raw| MemoryOperatingModeCapabilities::from(raw))
    }

    /// The firmware version of this memory device.
    pub fn firmware_version(&self) -> SMBiosString {
        self.parts.get_field_string(0x2B)
    }

    /// The two-byte module manufacturer ID found in the SPD of this memory device; LSB first.
    pub fn module_manufacturer_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x2C)
    }

    /// The two-byte module product ID found in the SPD of this memory device; LSB first.
    pub fn module_product_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x2E)
    }

    /// The two-byte memory subsystem controller manufacturer ID found in the SPD of this memory device; LSB first.
    pub fn memory_subsystem_controller_manufacturer_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x30)
    }

    /// The two-byte memory subsystem controller product ID found in the SPD of this memory device; LSB first.
    pub fn memory_subsystem_controller_product_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x32)
    }

    /// Size of the Non-volatile portion of the memory
    /// device in Bytes, if any.
    ///
    /// If the value is 0, there is no
    /// non-volatile portion.
    pub fn non_volatile_size(&self) -> Option<MemoryIndicatedSize> {
        self.parts
            .get_field_qword(0x34)
            .map(|raw| MemoryIndicatedSize::from(raw))
    }

    /// Size of the Volatile portion of the memory device in
    /// Bytes, if any.
    ///
    /// If the value is 0, there is no Volatile
    /// portion.
    pub fn volatile_size(&self) -> Option<MemoryIndicatedSize> {
        self.parts
            .get_field_qword(0x3C)
            .map(|raw| MemoryIndicatedSize::from(raw))
    }

    /// Size of the Cache portion of the memory device in
    /// Bytes, if any.
    ///
    /// If the value is 0, there is no Cache
    /// portion.
    pub fn cache_size(&self) -> Option<MemoryIndicatedSize> {
        self.parts
            .get_field_qword(0x44)
            .map(|raw| MemoryIndicatedSize::from(raw))
    }

    /// Size of the Logical memory device in Bytes.
    pub fn logical_size(&self) -> Option<MemoryIndicatedSize> {
        self.parts
            .get_field_qword(0x4C)
            .map(|raw| MemoryIndicatedSize::from(raw))
    }

    /// Extended speed of the memory device
    /// (complements the Speed field at offset 15h).
    /// Identifies the maximum capable speed of the
    /// device, in megatransfers per second (MT/s).
    pub fn extended_speed(&self) -> Option<MemorySpeedExtended> {
        self.parts
            .get_field_dword(0x54)
            .map(|raw| MemorySpeedExtended::from(raw))
    }

    /// Extended configured memory speed of the memory
    /// device (complements the 'configured_memory_speed'
    /// field at offset 20h).
    ///
    /// Identifies the configured
    /// speed of the memory device, in megatransfers per
    /// second (MT/s)
    ///
    /// The 'extended_speed' and 'extended_configured_memory_speed' fields are intended to represent
    /// memory devices that operate faster than 65,535 MT/s, which cannot be described using the
    /// Speed or Configured Memory Speed fields. These fields are only meaningful if the value
    /// in the Speed or Configured Memory Speed fields are FFFFh. For compatibility with older
    /// SMBIOS parsers, memory devices slower than 65,535 MT/s should represent their speed
    /// using the Speed and Configured Memory Speed fields, leaving the Extended Speed and
    /// Extended Configured Memory Speed fields set to 0.
    ///
    /// Bit 31 is reserved for future use and must be set to 0
    /// Bits 30:0 represent the speed or configured memory speed of the device in MT/s.
    pub fn extended_configured_memory_speed(&self) -> Option<MemorySpeedExtended> {
        self.parts
            .get_field_dword(0x58)
            .map(|raw| MemorySpeedExtended::from(raw))
    }
}

impl fmt::Debug for SMBiosMemoryDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosMemoryDevice<'_>>())
            .field("header", &self.parts.header)
            .field(
                "physical_memory_array_handle",
                &self.physical_memory_array_handle(),
            )
            .field(
                "memory_error_information_handle",
                &self.memory_error_information_handle(),
            )
            .field("total_width", &self.total_width())
            .field("data_width", &self.data_width())
            .field("size", &self.size())
            .field("form_factor", &self.form_factor())
            .field("device_set", &self.device_set())
            .field("device_locator", &self.device_locator())
            .field("bank_locator", &self.bank_locator())
            .field("memory_type", &self.memory_type())
            .field("type_detail", &self.type_detail())
            .field("speed", &self.speed())
            .field("manufacturer", &self.manufacturer())
            .field("serial_number", &self.serial_number())
            .field("asset_tag", &self.asset_tag())
            .field("part_number", &self.part_number())
            .field("attributes", &self.attributes())
            .field("extended_size", &self.extended_size())
            .field("configured_memory_speed", &self.configured_memory_speed())
            .field("minimum_voltage", &self.minimum_voltage())
            .field("maximum_voltage", &self.maximum_voltage())
            .field("configured_voltage", &self.configured_voltage())
            .field("memory_technology", &self.memory_technology())
            .field(
                "memory_operating_mode_capability",
                &self.memory_operating_mode_capability(),
            )
            .field("firmware_version", &self.firmware_version())
            .field("module_manufacturer_id", &self.module_manufacturer_id())
            .field("module_product_id", &self.module_product_id())
            .field(
                "memory_subsystem_controller_manufacturer_id",
                &self.memory_subsystem_controller_manufacturer_id(),
            )
            .field(
                "memory_subsystem_controller_product_id",
                &self.memory_subsystem_controller_product_id(),
            )
            .field("non_volatile_size", &self.non_volatile_size())
            .field("volatile_size", &self.volatile_size())
            .field("cache_size", &self.cache_size())
            .field("logical_size", &self.logical_size())
            .field("extended_speed", &self.extended_speed())
            .field(
                "extended_configured_memory_speed",
                &self.extended_configured_memory_speed(),
            )
            .finish()
    }
}

impl Serialize for SMBiosMemoryDevice<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosMemoryDevice", 36)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field(
            "physical_memory_array_handle",
            &self.physical_memory_array_handle(),
        )?;
        state.serialize_field(
            "memory_error_information_handle",
            &self.memory_error_information_handle(),
        )?;
        state.serialize_field("total_width", &self.total_width())?;
        state.serialize_field("data_width", &self.data_width())?;
        state.serialize_field("size", &self.size())?;
        state.serialize_field("form_factor", &self.form_factor())?;
        state.serialize_field("device_set", &self.device_set())?;
        state.serialize_field("device_locator", &self.device_locator())?;
        state.serialize_field("bank_locator", &self.bank_locator())?;
        state.serialize_field("memory_type", &self.memory_type())?;
        state.serialize_field("type_detail", &self.type_detail())?;
        state.serialize_field("speed", &self.speed())?;
        state.serialize_field("manufacturer", &self.manufacturer())?;
        state.serialize_field("serial_number", &self.serial_number())?;
        state.serialize_field("asset_tag", &self.asset_tag())?;
        state.serialize_field("part_number", &self.part_number())?;
        state.serialize_field("attributes", &self.attributes())?;
        state.serialize_field("extended_size", &self.extended_size())?;
        state.serialize_field("configured_memory_speed", &self.configured_memory_speed())?;
        state.serialize_field("minimum_voltage", &self.minimum_voltage())?;
        state.serialize_field("maximum_voltage", &self.maximum_voltage())?;
        state.serialize_field("configured_voltage", &self.configured_voltage())?;
        state.serialize_field("memory_technology", &self.memory_technology())?;
        state.serialize_field(
            "memory_operating_mode_capability",
            &self.memory_operating_mode_capability(),
        )?;
        state.serialize_field("firmware_version", &self.firmware_version())?;
        state.serialize_field("module_manufacturer_id", &self.module_manufacturer_id())?;
        state.serialize_field("module_product_id", &self.module_product_id())?;
        state.serialize_field(
            "memory_subsystem_controller_manufacturer_id",
            &self.memory_subsystem_controller_manufacturer_id(),
        )?;
        state.serialize_field(
            "memory_subsystem_controller_product_id",
            &self.memory_subsystem_controller_product_id(),
        )?;
        state.serialize_field("non_volatile_size", &self.non_volatile_size())?;
        state.serialize_field("volatile_size", &self.volatile_size())?;
        state.serialize_field("cache_size", &self.cache_size())?;
        state.serialize_field("logical_size", &self.logical_size())?;
        state.serialize_field("extended_speed", &self.extended_speed())?;
        state.serialize_field(
            "extended_configured_memory_speed",
            &self.extended_configured_memory_speed(),
        )?;
        state.end()
    }
}

/// # Memory Device - Type Data
#[derive(PartialEq, Eq)]
pub struct MemoryDeviceTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryDeviceType] value
    pub value: MemoryDeviceType,
}

impl fmt::Debug for MemoryDeviceTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<MemoryDeviceTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for MemoryDeviceTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryDeviceTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for MemoryDeviceTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            MemoryDeviceType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for MemoryDeviceTypeData {
    type Target = MemoryDeviceType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Device -Type
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemoryDeviceType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// DRAM
    Dram,
    /// EDRAM
    Edram,
    /// VRAM
    Vram,
    /// SRAM
    Sram,
    /// RAM
    Ram,
    /// ROM
    Rom,
    /// FLASH
    Flash,
    /// EEPROM
    Eeprom,
    /// FEPROM
    Feprom,
    /// EPROM
    Eprom,
    /// CDRAM
    Cdram,
    /// 3DRAM
    ThreeDram,
    /// SDRAM
    Sdram,
    /// SGRAM
    Sgram,
    /// RDRAM
    Rdram,
    /// DDR
    Ddr,
    /// DDR2
    Ddr2,
    /// DDR2 FB-DIMM
    Ddr2Fbdimm,
    /// DDR3
    Ddr3,
    /// FBD2
    Fbd2,
    /// DDR4
    Ddr4,
    /// LPDDR
    Lpddr,
    /// LPDDR2
    Lpddr2,
    /// LPDDR3
    Lpddr3,
    /// LPDDR4
    Lpddr4,
    /// Logical non-volatile device
    LogicalNonVolatileDevice,
    /// HBM (High Bandwidth Memory)
    Hbm,
    /// HBM2 (High Bandwidth Memory Generation 2)
    Hbm2,
    /// DDR5
    Ddr5,
    /// LPDDR5
    Lpddr5,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryDeviceTypeData {
    fn from(raw: u8) -> Self {
        MemoryDeviceTypeData {
            value: match raw {
                0x01 => MemoryDeviceType::Other,
                0x02 => MemoryDeviceType::Unknown,
                0x03 => MemoryDeviceType::Dram,
                0x04 => MemoryDeviceType::Edram,
                0x05 => MemoryDeviceType::Vram,
                0x06 => MemoryDeviceType::Sram,
                0x07 => MemoryDeviceType::Ram,
                0x08 => MemoryDeviceType::Rom,
                0x09 => MemoryDeviceType::Flash,
                0x0A => MemoryDeviceType::Eeprom,
                0x0B => MemoryDeviceType::Feprom,
                0x0C => MemoryDeviceType::Eprom,
                0x0D => MemoryDeviceType::Cdram,
                0x0E => MemoryDeviceType::ThreeDram,
                0x0F => MemoryDeviceType::Sdram,
                0x10 => MemoryDeviceType::Sgram,
                0x11 => MemoryDeviceType::Rdram,
                0x12 => MemoryDeviceType::Ddr,
                0x13 => MemoryDeviceType::Ddr2,
                0x14 => MemoryDeviceType::Ddr2Fbdimm,
                0x18 => MemoryDeviceType::Ddr3,
                0x19 => MemoryDeviceType::Fbd2,
                0x1A => MemoryDeviceType::Ddr4,
                0x1B => MemoryDeviceType::Lpddr,
                0x1C => MemoryDeviceType::Lpddr2,
                0x1D => MemoryDeviceType::Lpddr3,
                0x1E => MemoryDeviceType::Lpddr4,
                0x1F => MemoryDeviceType::LogicalNonVolatileDevice,
                0x20 => MemoryDeviceType::Hbm,
                0x21 => MemoryDeviceType::Hbm2,
                0x22 => MemoryDeviceType::Ddr5,
                0x23 => MemoryDeviceType::Lpddr5,
                _ => MemoryDeviceType::None,
            },
            raw,
        }
    }
}

/// # Memory Device — Form Factor Data
pub struct MemoryFormFactorData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryFormFactor] value
    pub value: MemoryFormFactor,
}

impl fmt::Debug for MemoryFormFactorData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<MemoryFormFactorData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for MemoryFormFactorData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryFormFactorData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for MemoryFormFactorData {
    type Target = MemoryFormFactor;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Device — Form Factor
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemoryFormFactor {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// SIMM
    Simm,
    /// SIP
    Sip,
    /// Chip
    Chip,
    /// DIP
    Dip,
    /// ZIP
    Zip,
    /// Proprietary Card
    ProprietaryCard,
    /// DIMM
    Dimm,
    /// TSOP
    Tsop,
    /// Row of chips
    RowOfChips,
    /// RIMM
    Rimm,
    /// SODIMM
    Sodimm,
    /// SRIMM
    Srimm,
    /// FB-DIMM
    Fbdimm,
    /// Die
    Die,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryFormFactorData {
    fn from(raw: u8) -> Self {
        MemoryFormFactorData {
            value: match raw {
                0x01 => MemoryFormFactor::Other,
                0x02 => MemoryFormFactor::Unknown,
                0x03 => MemoryFormFactor::Simm,
                0x04 => MemoryFormFactor::Sip,
                0x05 => MemoryFormFactor::Chip,
                0x06 => MemoryFormFactor::Dip,
                0x07 => MemoryFormFactor::Zip,
                0x08 => MemoryFormFactor::ProprietaryCard,
                0x09 => MemoryFormFactor::Dimm,
                0x0A => MemoryFormFactor::Tsop,
                0x0B => MemoryFormFactor::RowOfChips,
                0x0C => MemoryFormFactor::Rimm,
                0x0D => MemoryFormFactor::Sodimm,
                0x0E => MemoryFormFactor::Srimm,
                0x0F => MemoryFormFactor::Fbdimm,
                0x10 => MemoryFormFactor::Die,
                _ => MemoryFormFactor::None,
            },
            raw,
        }
    }
}

/// # Memory Device — Type Detail
#[derive(PartialEq, Eq)]
pub struct MemoryTypeDetails {
    /// Raw value
    pub raw: u16,
}

impl Deref for MemoryTypeDetails {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for MemoryTypeDetails {
    fn from(raw: u16) -> Self {
        MemoryTypeDetails { raw }
    }
}

impl MemoryTypeDetails {
    /// Bit 1 Other
    pub fn other(&self) -> bool {
        self.raw & 0x0002 == 0x0002
    }

    /// Bit 2 Unknown
    pub fn unknown(&self) -> bool {
        self.raw & 0x0004 == 0x0004
    }

    /// Bit 3 Fast-paged
    pub fn fast_paged(&self) -> bool {
        self.raw & 0x0008 == 0x0008
    }

    /// Bit 4 Static column
    pub fn static_column(&self) -> bool {
        self.raw & 0x0010 == 0x0010
    }

    /// Bit 5 Pseudo-static
    pub fn pseudo_static(&self) -> bool {
        self.raw & 0x0020 == 0x0020
    }

    /// Bit 6 RAMBUS
    pub fn ram_bus(&self) -> bool {
        self.raw & 0x0040 == 0x0040
    }

    /// Bit 7 Synchronous
    pub fn synchronous(&self) -> bool {
        self.raw & 0x0080 == 0x0080
    }

    /// Bit 8 CMOS
    pub fn cmos(&self) -> bool {
        self.raw & 0x0100 == 0x0100
    }

    /// Bit 9 EDO
    pub fn edo(&self) -> bool {
        self.raw & 0x0200 == 0x0200
    }

    /// Bit 10 Window DRAM
    pub fn window_dram(&self) -> bool {
        self.raw & 0x0400 == 0x0400
    }

    /// Bit 11 Cache DRAM
    pub fn cache_dram(&self) -> bool {
        self.raw & 0x0800 == 0x0800
    }

    /// Bit 12 Non-volatile
    pub fn non_volatile(&self) -> bool {
        self.raw & 0x1000 == 0x1000
    }

    /// Bit 13 Registered (Buffered)
    pub fn registered(&self) -> bool {
        self.raw & 0x2000 == 0x2000
    }

    /// Bit 14 Unbuffered (Unregistered)
    pub fn unbuffered(&self) -> bool {
        self.raw & 0x4000 == 0x4000
    }

    /// Bit 15 LRDIMM
    pub fn lrdimm(&self) -> bool {
        self.raw & 0x8000 == 0x8000
    }
}

impl fmt::Debug for MemoryTypeDetails {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<MemoryTypeDetails>())
            .field("raw", &self.raw)
            .field("other", &self.other())
            .field("unknown", &self.unknown())
            .field("fast_paged", &self.fast_paged())
            .field("static_column", &self.static_column())
            .field("pseudo_static", &self.pseudo_static())
            .field("ram_bus", &self.ram_bus())
            .field("synchronous", &self.synchronous())
            .field("cmos", &self.cmos())
            .field("edo", &self.edo())
            .field("window_dram", &self.window_dram())
            .field("cache_dram", &self.cache_dram())
            .field("non_volatile", &self.non_volatile())
            .field("registered", &self.registered())
            .field("unbuffered", &self.unbuffered())
            .field("lrdimm", &self.lrdimm())
            .finish()
    }
}

impl Serialize for MemoryTypeDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryTypeDetails", 16)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("other", &self.other())?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("fast_paged", &self.fast_paged())?;
        state.serialize_field("static_column", &self.static_column())?;
        state.serialize_field("pseudo_static", &self.pseudo_static())?;
        state.serialize_field("ram_bus", &self.ram_bus())?;
        state.serialize_field("synchronous", &self.synchronous())?;
        state.serialize_field("cmos", &self.cmos())?;
        state.serialize_field("edo", &self.edo())?;
        state.serialize_field("window_dram", &self.window_dram())?;
        state.serialize_field("cache_dram", &self.cache_dram())?;
        state.serialize_field("non_volatile", &self.non_volatile())?;
        state.serialize_field("registered", &self.registered())?;
        state.serialize_field("unbuffered", &self.unbuffered())?;
        state.serialize_field("lrdimm", &self.lrdimm())?;
        state.end()
    }
}

/// # Memory Device — Memory Technology Data
pub struct MemoryDeviceTechnologyData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [MemoryDeviceTechnology] value
    pub value: MemoryDeviceTechnology,
}

impl fmt::Debug for MemoryDeviceTechnologyData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<MemoryDeviceTechnologyData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for MemoryDeviceTechnologyData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryDeviceTechnologyData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for MemoryDeviceTechnologyData {
    type Target = MemoryDeviceTechnology;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Device — Memory Technology
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemoryDeviceTechnology {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// DRAM
    Dram,
    /// NVDIMM-N
    NvdimmN,
    /// NVDIMM-F
    NvdimmF,
    /// NVDIMM-P
    NvdimmP,
    /// Intel® Optane™ persistent memory
    IntelOptaneDcPersistentMemory,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for MemoryDeviceTechnologyData {
    fn from(raw: u8) -> Self {
        MemoryDeviceTechnologyData {
            value: match raw {
                0x01 => MemoryDeviceTechnology::Other,
                0x02 => MemoryDeviceTechnology::Unknown,
                0x03 => MemoryDeviceTechnology::Dram,
                0x04 => MemoryDeviceTechnology::NvdimmN,
                0x05 => MemoryDeviceTechnology::NvdimmF,
                0x06 => MemoryDeviceTechnology::NvdimmP,
                0x07 => MemoryDeviceTechnology::IntelOptaneDcPersistentMemory,
                _ => MemoryDeviceTechnology::None,
            },
            raw,
        }
    }
}

/// # Memory Device — Memory Operating Mode Capability
#[derive(PartialEq, Eq)]
pub struct MemoryOperatingModeCapabilities {
    /// Raw value
    pub raw: u16,
}

impl Deref for MemoryOperatingModeCapabilities {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for MemoryOperatingModeCapabilities {
    fn from(raw: u16) -> Self {
        MemoryOperatingModeCapabilities { raw }
    }
}

impl MemoryOperatingModeCapabilities {
    /// Other
    pub fn other(&self) -> bool {
        self.raw & 0x0002 == 0x0002
    }

    /// Unknown
    pub fn unknown(&self) -> bool {
        self.raw & 0x0004 == 0x0004
    }

    /// Volatile memory
    pub fn volatile_memory(&self) -> bool {
        self.raw & 0x0008 == 0x0008
    }

    /// Byte-accessible persistent memory
    pub fn byte_accessible_persistent_memory(&self) -> bool {
        self.raw & 0x0010 == 0x0010
    }

    /// Block-accessible persistent memory
    pub fn block_accessible_persistent_memory(&self) -> bool {
        self.raw & 0x0020 == 0x0020
    }
}

impl fmt::Debug for MemoryOperatingModeCapabilities {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<MemoryOperatingModeCapabilities>())
            .field("raw", &self.raw)
            .field("other", &self.other())
            .field("unknown", &self.unknown())
            .field("volatile_memory", &self.volatile_memory())
            .field(
                "byte_accessible_persistent_memory",
                &self.byte_accessible_persistent_memory(),
            )
            .field(
                "block_accessible_persistent_memory",
                &self.block_accessible_persistent_memory(),
            )
            .finish()
    }
}

impl Serialize for MemoryOperatingModeCapabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryOperatingModeCapabilities", 6)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("other", &self.other())?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("volatile_memory", &self.volatile_memory())?;
        state.serialize_field(
            "byte_accessible_persistent_memory",
            &self.byte_accessible_persistent_memory(),
        )?;
        state.serialize_field(
            "block_accessible_persistent_memory",
            &self.block_accessible_persistent_memory(),
        )?;
        state.end()
    }
}

/// # Speed of Memory
#[derive(Serialize, Debug)]
pub enum MemorySpeed {
    /// Memory Speed is Unknown
    Unknown,
    /// The speed is 65,535 MT/s or greater, and the actual speed is stored in the 'extended_speed' field
    SeeExtendedSpeed,
    /// The maximum capable speed of the device, in megatransfers per second (MT/s)
    MTs(u16),
}

impl From<u16> for MemorySpeed {
    fn from(raw: u16) -> Self {
        match raw {
            0 => MemorySpeed::Unknown,
            0xFFFF => MemorySpeed::SeeExtendedSpeed,
            _ => MemorySpeed::MTs(raw),
        }
    }
}

/// # Extended Speed of Memory
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemorySpeedExtended {
    /// Speed of memory in megatransfers per second (MT/s)
    MTs(u32),
    /// See the _speed_ field.
    ///
    /// For compatibility with older SMBIOS parsers, memory devices slower than 65,535 MT/s should represent their speed
    /// using the Speed and Configured Memory Speed fields, leaving the Extended Speed and
    /// Extended Configured Memory Speed fields set to 0.
    SeeSpeed,
}

/// Bit 31 is reserved for future use and must be set to 0
/// Bits 30:0 represent the speed or configured memory speed of the device in MT/s.
impl From<u32> for MemorySpeedExtended {
    fn from(raw: u32) -> Self {
        let bits_30_to_0 = raw & 0x7FFF_FFFF;
        match bits_30_to_0 {
            0 => MemorySpeedExtended::SeeSpeed,
            _ => MemorySpeedExtended::MTs(bits_30_to_0),
        }
    }
}

/// # Size of Memory
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemorySize {
    /// No Memory Device Installed in the Socket
    NotInstalled,
    /// Memory Size is Unknown
    Unknown,
    /// The actual size is stored in the Extended Size field
    SeeExtendedSize,
    /// Size of Memory (KB)
    Kilobytes(u16),
    /// Size of Memory (MB)
    Megabytes(u16),
}

/// If the value is 0, no memory device is installed in the
/// socket; if the size is unknown, the field value is
/// FFFFh. If the size is 32 GB-1 MB or greater, the
/// field value is 7FFFh and the actual size is stored in
/// the Extended Size field.
/// The granularity in which the value is specified
/// depends on the setting of the most-significant bit (bit
/// 15). If the bit is 0, the value is specified in megabyte
/// units; if the bit is 1, the value is specified in kilobyte
/// units. For example, the value 8100h identifies a
/// 256 KB memory device and 0100h identifies a
/// 256 MB memory device.
impl From<u16> for MemorySize {
    fn from(raw: u16) -> Self {
        match raw {
            0 => MemorySize::NotInstalled,
            0xFFFF => MemorySize::Unknown,
            0x7FFF => MemorySize::SeeExtendedSize,
            _ => match raw & 0x8000 {
                0x8000 => MemorySize::Kilobytes(raw & 0x7FFF),
                _ => MemorySize::Megabytes(raw),
            },
        }
    }
}

/// # Extended Size of Memory
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MemorySizeExtended {
    /// Size of Memory (MB)
    Megabytes(u32),
    /// See the _size_ field.
    ///
    /// For compatibility with older SMBIOS parsers, memory devices smaller than (32 GB - 1 MB)
    /// should be represented using their size in the Size field, leaving the Extended Size field set to 0.
    SeeSize,
}

/// Bit 31 is reserved for future use and must be set to 0.
/// Bits 30:0 represent the size of the memory device in megabytes.
impl From<u32> for MemorySizeExtended {
    fn from(raw: u32) -> Self {
        let bits_30_to_0 = raw & 0x7FFF_FFFF;
        match bits_30_to_0 {
            0 => MemorySizeExtended::SeeSize,
            _ => MemorySizeExtended::Megabytes(bits_30_to_0),
        }
    }
}

/// # Size of Memory in Bytes
#[derive(Serialize, Debug)]
pub enum MemoryIndicatedSize {
    /// Memory Size is Unknown
    Unknown,
    /// Size of Memory (bytes)
    Bytes(u64),
}

impl From<u64> for MemoryIndicatedSize {
    fn from(raw: u64) -> Self {
        match raw {
            0xFFFFFFFFFFFFFFFF => MemoryIndicatedSize::Unknown,
            _ => MemoryIndicatedSize::Bytes(raw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Memory Device structure is sensitive to BIOS specification versions
        // and prone to bugs.  Therefore, it is important to test different
        // structure versions.
        //
        // Length of the structure:
        // 15h for version 2.1,
        // 1Bh for version 2.3,
        // 1Ch for version 2.6,
        // 22h for version 2.7,
        // 28h for version 2.8,
        // 54h for version 3.2,
        // 5Ch for version 3.3 and later

        // Memory Device structure version 2.8 (does not contain _memory_technology()_ field and beyond)
        let struct_type17 = vec![
            0x11, 0x28, 0x40, 0x00, 0x3E, 0x00, 0xFE, 0xFF, 0x48, 0x00, 0x40, 0x00, 0x00, 0x20,
            0x09, 0x00, 0x01, 0x02, 0x1A, 0x80, 0x00, 0x6A, 0x0A, 0x03, 0x04, 0x05, 0x06, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x6A, 0x0A, 0xB0, 0x04, 0xB0, 0x04, 0xB0, 0x04, 0x43, 0x50,
            0x55, 0x31, 0x5F, 0x44, 0x49, 0x4D, 0x4D, 0x5F, 0x31, 0x00, 0x4E, 0x4F, 0x44, 0x45,
            0x20, 0x31, 0x00, 0x48, 0x79, 0x6E, 0x69, 0x78, 0x00, 0x37, 0x32, 0x30, 0x39, 0x31,
            0x30, 0x30, 0x33, 0x00, 0x20, 0x00, 0x48, 0x4D, 0x41, 0x38, 0x31, 0x47, 0x52, 0x37,
            0x41, 0x46, 0x52, 0x38, 0x4E, 0x2D, 0x56, 0x4B, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type17);
        let test_struct = SMBiosMemoryDevice::new(&parts);
        assert_eq!(test_struct.physical_memory_array_handle(), Some(Handle(62)));
        assert_eq!(
            test_struct.memory_error_information_handle(),
            Some(Handle(65534))
        );
        assert_eq!(test_struct.total_width(), Some(72));
        assert_eq!(test_struct.data_width(), Some(64));
        match test_struct.size().unwrap() {
            MemorySize::Megabytes(size) => assert_eq!(size, 8192),
            _ => panic!("incorrect size"),
        }
        assert_eq!(*test_struct.form_factor().unwrap(), MemoryFormFactor::Dimm);
        assert_eq!(test_struct.device_set(), Some(0));
        assert_eq!(
            test_struct.device_locator().to_string(),
            "CPU1_DIMM_1".to_string()
        );
        assert_eq!(test_struct.bank_locator().to_string(), "NODE 1".to_string());
        assert_eq!(
            test_struct.memory_type(),
            Some(MemoryDeviceTypeData::from(26))
        );
        assert_eq!(
            test_struct.type_detail(),
            Some(MemoryTypeDetails::from(128))
        );
        match test_struct.speed().unwrap() {
            MemorySpeed::MTs(speed) => assert_eq!(speed, 2666),
            MemorySpeed::Unknown => panic!("expected speed"),
            MemorySpeed::SeeExtendedSpeed => panic!("expected speed"),
        }
        assert_eq!(test_struct.manufacturer().to_string(), "Hynix".to_string());
        assert_eq!(
            test_struct.serial_number().to_string(),
            "72091003".to_string()
        );
        assert_eq!(test_struct.asset_tag().to_string(), " ".to_string());
        assert_eq!(
            test_struct.part_number().to_string(),
            "HMA81GR7AFR8N-VK    ".to_string()
        );
        assert_eq!(test_struct.attributes(), Some(1));
        assert_eq!(
            test_struct.extended_size(),
            Some(MemorySizeExtended::SeeSize)
        );
        match test_struct.configured_memory_speed().unwrap() {
            MemorySpeed::MTs(speed) => assert_eq!(speed, 2666),
            MemorySpeed::Unknown => panic!("expected speed"),
            MemorySpeed::SeeExtendedSpeed => panic!("expected speed"),
        }
        assert_eq!(test_struct.minimum_voltage(), Some(1200));
        assert_eq!(test_struct.maximum_voltage(), Some(1200));
        assert_eq!(test_struct.configured_voltage(), Some(1200));

        // version 2.8 does not contain _memory_technology()_ field and fields beyond
        assert!(test_struct.memory_technology().is_none());

        // 3.3 structure
        let struct_type17 = vec![
            0x11, 0x5C, 0x40, 0x00, 0x3E, 0x00, 0xFE, 0xFF, 0x48, 0x00, 0x40, 0x00, 0x00, 0x20,
            0x09, 0x00, 0x01, 0x02, 0x1A, 0x80, 0x00, 0x6A, 0x0A, 0x03, 0x04, 0x05, 0x06, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x6A, 0x0A, 0xB0, 0x04, 0xB0, 0x04, 0xB0, 0x04,
            0x07, // MemoryDeviceTechnology::IntelOptaneDcPersistentMemory
            0x20, 0x00, // MemoryOperatingModeCapabilities block_accessible_persistent_memory
            0x07, // firmware version "8"
            0x00, 0x00, // module_manufacturer_id
            0x00, 0x00, // module_product_id
            0x00, 0x00, // memory_subsystem_controller_manufacturer_id
            0x00, 0x00, // memory_subsystem_controller_product_id
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // non_volatile_size
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // volatile_size
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // cache_size
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // logical_size
            0x04, 0x03, 0x02, 0x01, // extended_speed
            0x08, 0x07, 0x06, 0x05, // extended_configured_memory_speed
            0x43, 0x50, 0x55, 0x31, 0x5F, 0x44, 0x49, 0x4D, 0x4D, 0x5F, 0x31, 0x00, 0x4E, 0x4F,
            0x44, 0x45, 0x20, 0x31, 0x00, 0x48, 0x79, 0x6E, 0x69, 0x78, 0x00, 0x37, 0x32, 0x30,
            0x39, 0x31, 0x30, 0x30, 0x33, 0x00, 0x20, 0x00, 0x48, 0x4D, 0x41, 0x38, 0x31, 0x47,
            0x52, 0x37, 0x41, 0x46, 0x52, 0x38, 0x4E, 0x2D, 0x56, 0x4B, 0x20, 0x20, 0x20, 0x20,
            0x00, 0x38, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type17);
        let test_struct = SMBiosMemoryDevice::new(&parts);

        assert_eq!(
            *test_struct.memory_technology().unwrap(),
            MemoryDeviceTechnology::IntelOptaneDcPersistentMemory
        );
        assert!(test_struct
            .memory_operating_mode_capability()
            .unwrap()
            .block_accessible_persistent_memory());
        assert_eq!(test_struct.firmware_version().to_string(), "8".to_string());
        assert_eq!(test_struct.module_manufacturer_id(), Some(0));
        assert_eq!(test_struct.module_product_id(), Some(0));
        assert_eq!(
            test_struct.memory_subsystem_controller_manufacturer_id(),
            Some(0)
        );
        assert_eq!(
            test_struct.memory_subsystem_controller_product_id(),
            Some(0)
        );
        match test_struct.non_volatile_size().unwrap() {
            MemoryIndicatedSize::Unknown => (),
            MemoryIndicatedSize::Bytes(_) => panic!("expected unknown"),
        }
        match test_struct.volatile_size().unwrap() {
            MemoryIndicatedSize::Unknown => (),
            MemoryIndicatedSize::Bytes(_) => panic!("expected unknown"),
        }
        match test_struct.cache_size().unwrap() {
            MemoryIndicatedSize::Unknown => (),
            MemoryIndicatedSize::Bytes(_) => panic!("expected unknown"),
        }
        match test_struct.logical_size().unwrap() {
            MemoryIndicatedSize::Unknown => (),
            MemoryIndicatedSize::Bytes(_) => panic!("expected unknown"),
        }
        assert_eq!(
            test_struct.extended_speed(),
            Some(MemorySpeedExtended::MTs(0x01020304))
        );
        assert_eq!(
            test_struct.extended_configured_memory_speed(),
            Some(MemorySpeedExtended::MTs(0x05060708))
        );
    }
}
