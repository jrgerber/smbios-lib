use crate::core::{Handle, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeSeq, ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use core::ops::Deref;
use alloc::vec::Vec;

/// # Memory Controller Information (Type 5, Obsolete)
///
/// The information in this structure defines the attributes of the system’s memory controller(s) and the
/// supported attributes of any memory-modules present in the sockets controlled by this controller.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryControllerInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryControllerInformation<'a> {
    const STRUCT_TYPE: u8 = 5u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosMemoryControllerInformation<'a> {
    /// Error detecting method
    pub fn error_detecting_method(&self) -> Option<ErrorDetectingMethodData> {
        self.parts
            .get_field_byte(0x04)
            .map(|raw| ErrorDetectingMethodData::from(raw))
    }

    /// Error correcting capability
    pub fn error_correcting_capability(&self) -> Option<ErrorCorrectingCapabilities> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| ErrorCorrectingCapabilities::from(raw))
    }

    /// Supported interleave
    pub fn supported_interleave(&self) -> Option<InterleaveSupportData> {
        self.parts
            .get_field_byte(0x06)
            .map(|raw| InterleaveSupportData::from(raw))
    }

    /// Current interleave
    pub fn current_interleave(&self) -> Option<InterleaveSupportData> {
        self.parts
            .get_field_byte(0x07)
            .map(|raw| InterleaveSupportData::from(raw))
    }

    /// Maximum Memory Module Size
    ///
    /// Size of the largest memory module supported (per slot),
    /// specified as n, where 2**n is the maximum size in MB
    ///
    /// The maximum amount of memory supported by this controller
    /// is that value times the number of slots, as specified in
    /// offset 0Eh of this structure.
    pub fn maximum_memory_module_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }

    /// Supported Speeds
    pub fn supported_speeds(&self) -> Option<MemorySpeeds> {
        self.parts
            .get_field_word(0x09)
            .map(|raw| MemorySpeeds::from(raw))
    }

    /// Supported Memory Types
    pub fn supported_memory_types(&self) -> Option<MemoryTypes> {
        self.parts
            .get_field_word(0x0B)
            .map(|raw| MemoryTypes::from(raw))
    }

    /// Memory Module Voltage
    pub fn memory_module_voltage(&self) -> Option<ModuleVoltage> {
        self.parts
            .get_field_byte(0x0D)
            .map(|raw| ModuleVoltage::from(raw))
    }

    /// Number of Associated Memory Slots
    pub fn number_of_associated_memory_slots(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0E)
    }

    /// Memory Module Configuration Handles
    pub fn memory_module_handle_iterator(&self) -> ModuleHandleIterator<'_> {
        ModuleHandleIterator::new(self)
    }

    /// Memory Moduel Error Correcting Capabilities
    pub fn error_correcting_capabilities_iterator(&self) -> ErrorCapabilitiesIterator<'_> {
        ErrorCapabilitiesIterator::new(self)
    }
}

impl fmt::Debug for SMBiosMemoryControllerInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosMemoryControllerInformation<'_>>())
            .field("header", &self.parts.header)
            .field("error_detecting_method", &self.error_detecting_method())
            .field(
                "error_correcting_capability",
                &self.error_correcting_capability(),
            )
            .field("supported_interleave", &self.supported_interleave())
            .field("current_interleave", &self.current_interleave())
            .field(
                "maximum_memory_module_size",
                &self.maximum_memory_module_size(),
            )
            .field("supported_speeds", &self.supported_speeds())
            .field("supported_memory_types", &self.supported_memory_types())
            .field("memory_module_voltage", &self.memory_module_voltage())
            .field(
                "number_of_associated_memory_slots",
                &self.number_of_associated_memory_slots(),
            )
            .field(
                "memory_module_handle_iterator",
                &self.memory_module_handle_iterator(),
            )
            .field(
                "error_correcting_capabilities_iterator",
                &self.error_correcting_capabilities_iterator(),
            )
            .finish()
    }
}

impl Serialize for SMBiosMemoryControllerInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosMemoryControllerInformation", 12)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("error_detecting_method", &self.error_detecting_method())?;
        state.serialize_field(
            "error_correcting_capability",
            &self.error_correcting_capability(),
        )?;
        state.serialize_field("supported_interleave", &self.supported_interleave())?;
        state.serialize_field("current_interleave", &self.current_interleave())?;
        state.serialize_field(
            "maximum_memory_module_size",
            &self.maximum_memory_module_size(),
        )?;
        state.serialize_field("supported_speeds", &self.supported_speeds())?;
        state.serialize_field("supported_memory_types", &self.supported_memory_types())?;
        state.serialize_field("memory_module_voltage", &self.memory_module_voltage())?;
        state.serialize_field(
            "number_of_associated_memory_slots",
            &self.number_of_associated_memory_slots(),
        )?;
        state.serialize_field(
            "memory_module_handle_iterator",
            &self.memory_module_handle_iterator(),
        )?;
        state.serialize_field(
            "error_correcting_capabilities_iterator",
            &self.error_correcting_capabilities_iterator(),
        )?;
        state.end()
    }
}

/// # Memory Controller Error Detecting Method Data
pub struct ErrorDetectingMethodData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ErrorDetectingMethod] value
    pub value: ErrorDetectingMethod,
}

impl fmt::Debug for ErrorDetectingMethodData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<ErrorDetectingMethodData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ErrorDetectingMethodData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ErrorDetectingMethodData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for ErrorDetectingMethodData {
    type Target = ErrorDetectingMethod;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Controller Error Detecting Method
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ErrorDetectingMethod {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// No Error Detection
    NoErrorDetection,
    /// 8-bit Parity
    Parity8Bit,
    /// 32-bit ECC
    Ecc32Bit,
    /// 64-bit ECC
    Ecc64Bit,
    /// 128-bit ECC
    Ecc128Bit,
    /// CRC
    Crc,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ErrorDetectingMethodData {
    fn from(raw: u8) -> Self {
        ErrorDetectingMethodData {
            value: match raw {
                0x01 => ErrorDetectingMethod::Other,
                0x02 => ErrorDetectingMethod::Unknown,
                0x03 => ErrorDetectingMethod::NoErrorDetection,
                0x04 => ErrorDetectingMethod::Parity8Bit,
                0x05 => ErrorDetectingMethod::Ecc32Bit,
                0x06 => ErrorDetectingMethod::Ecc64Bit,
                0x07 => ErrorDetectingMethod::Ecc128Bit,
                0x08 => ErrorDetectingMethod::Crc,
                _ => ErrorDetectingMethod::None,
            },
            raw,
        }
    }
}

/// # Memory Controller Error Correcting Capability
#[derive(PartialEq, Eq)]
pub struct ErrorCorrectingCapabilities {
    /// Raw value
    pub raw: u8,
}

impl Deref for ErrorCorrectingCapabilities {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for ErrorCorrectingCapabilities {
    fn from(raw: u8) -> Self {
        ErrorCorrectingCapabilities { raw }
    }
}

impl ErrorCorrectingCapabilities {
    /// ErrorCorrectingCapabilities Size (1 byte)
    pub const SIZE: usize = 1usize;

    /// Other
    pub fn other(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// Unknown
    pub fn unknown(&self) -> bool {
        self.raw & 0x02 == 0x02
    }

    /// None
    pub fn no_capabilities(&self) -> bool {
        self.raw & 0x04 == 0x04
    }

    /// Single-Bit Error Correcting
    pub fn single_bit_error_correcting(&self) -> bool {
        self.raw & 0x08 == 0x08
    }

    /// Double-Bit Error Correcting
    pub fn double_bit_error_correcting(&self) -> bool {
        self.raw & 0x10 == 0x10
    }

    /// Error Scrubbing
    pub fn error_scrubbing(&self) -> bool {
        self.raw & 0x20 == 0x20
    }
}

impl fmt::Debug for ErrorCorrectingCapabilities {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<ErrorCorrectingCapabilities>())
            .field("raw", &self.raw)
            .field("other", &self.other())
            .field("unknown", &self.unknown())
            .field("no_capabilities", &self.no_capabilities())
            .field(
                "single_bit_error_correcting",
                &self.single_bit_error_correcting(),
            )
            .field(
                "double_bit_error_correcting",
                &self.double_bit_error_correcting(),
            )
            .field("error_scrubbing", &self.error_scrubbing())
            .finish()
    }
}

impl Serialize for ErrorCorrectingCapabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ErrorCorrectingCapabilities", 7)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("other", &self.other())?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("no_capabilities", &self.no_capabilities())?;
        state.serialize_field(
            "single_bit_error_correcting",
            &self.single_bit_error_correcting(),
        )?;
        state.serialize_field(
            "double_bit_error_correcting",
            &self.double_bit_error_correcting(),
        )?;
        state.serialize_field("error_scrubbing", &self.error_scrubbing())?;
        state.end()
    }
}

/// # Memory Controller Information — Interleave Support Data
pub struct InterleaveSupportData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [InterleaveSupport] value
    pub value: InterleaveSupport,
}

impl fmt::Debug for InterleaveSupportData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<InterleaveSupportData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for InterleaveSupportData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("InterleaveSupportData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for InterleaveSupportData {
    type Target = InterleaveSupport;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Memory Controller Information — Interleave Support
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum InterleaveSupport {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// One-Way Interleave
    OneWay,
    /// Two-Way Interleave
    TwoWay,
    /// Four-Way Interleave
    FourWay,
    /// Eight-Way Interleave
    EightWay,
    /// Sixteen-Way Interleave
    SixteenWay,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for InterleaveSupportData {
    fn from(raw: u8) -> Self {
        InterleaveSupportData {
            value: match raw {
                0x01 => InterleaveSupport::Other,
                0x02 => InterleaveSupport::Unknown,
                0x03 => InterleaveSupport::OneWay,
                0x04 => InterleaveSupport::TwoWay,
                0x05 => InterleaveSupport::FourWay,
                0x06 => InterleaveSupport::EightWay,
                0x07 => InterleaveSupport::SixteenWay,
                _ => InterleaveSupport::None,
            },
            raw,
        }
    }
}

/// # Memory Controller Information — Memory Speeds
#[derive(PartialEq, Eq)]
pub struct MemorySpeeds {
    /// Raw value
    pub raw: u16,
}

impl Deref for MemorySpeeds {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for MemorySpeeds {
    fn from(raw: u16) -> Self {
        MemorySpeeds { raw }
    }
}

impl MemorySpeeds {
    /// Other
    pub fn other(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// Unknown
    pub fn unknown(&self) -> bool {
        self.raw & 0x02 == 0x02
    }

    /// 70ns
    pub fn ns70(&self) -> bool {
        self.raw & 0x04 == 0x04
    }

    /// 60ns
    pub fn ns60(&self) -> bool {
        self.raw & 0x08 == 0x08
    }

    /// 50ns
    pub fn ns50(&self) -> bool {
        self.raw & 0x10 == 0x10
    }
}

impl fmt::Debug for MemorySpeeds {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<MemorySpeeds>())
            .field("raw", &self.raw)
            .field("other", &self.other())
            .field("unknown", &self.unknown())
            .field("ns70", &self.ns70())
            .field("ns60", &self.ns60())
            .field("ns50", &self.ns50())
            .finish()
    }
}

impl Serialize for MemorySpeeds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemorySpeeds", 6)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("other", &self.other())?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("ns70", &self.ns70())?;
        state.serialize_field("ns60", &self.ns60())?;
        state.serialize_field("ns50", &self.ns50())?;
        state.end()
    }
}

/// # Memory Module Information: Memory Types
#[derive(PartialEq, Eq)]
pub struct MemoryTypes {
    /// Raw value
    pub raw: u16,
}

impl Deref for MemoryTypes {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for MemoryTypes {
    fn from(raw: u16) -> Self {
        MemoryTypes { raw }
    }
}

impl MemoryTypes {
    /// Bit 0 Other
    pub fn other(&self) -> bool {
        self.raw & 0x0001 == 0x0001
    }

    /// Bit 1 Unknown
    pub fn unknown(&self) -> bool {
        self.raw & 0x0002 == 0x0002
    }

    /// Bit 2 Standard
    pub fn standard(&self) -> bool {
        self.raw & 0x0004 == 0x0004
    }

    /// Bit 3 Fast Page Mode
    pub fn fast_page_mode(&self) -> bool {
        self.raw & 0x0008 == 0x0008
    }

    /// Bit 4 EDO
    pub fn edo(&self) -> bool {
        self.raw & 0x0010 == 0x0010
    }

    /// Bit 5 Parity
    pub fn parity(&self) -> bool {
        self.raw & 0x0020 == 0x0020
    }

    /// Bit 6 ECC
    pub fn ecc(&self) -> bool {
        self.raw & 0x0040 == 0x0040
    }

    /// Bit 7 SIMM
    pub fn simm(&self) -> bool {
        self.raw & 0x0080 == 0x0080
    }

    /// Bit 8 DIMM
    pub fn dimm(&self) -> bool {
        self.raw & 0x0100 == 0x0100
    }

    /// Bit 9 Burst EDO
    pub fn burst_edo(&self) -> bool {
        self.raw & 0x0200 == 0x0200
    }

    /// Bit 10 SDRAM
    pub fn sdram(&self) -> bool {
        self.raw & 0x0400 == 0x0400
    }
}

impl fmt::Debug for MemoryTypes {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<MemoryTypes>())
            .field("raw", &self.raw)
            .field("other", &self.other())
            .field("unknown", &self.unknown())
            .field("standard", &self.standard())
            .field("fast_page_mode", &self.fast_page_mode())
            .field("edo", &self.edo())
            .field("parity", &self.parity())
            .field("ecc", &self.ecc())
            .field("simm", &self.simm())
            .field("dimm", &self.dimm())
            .field("burst_edo", &self.burst_edo())
            .field("sdram", &self.sdram())
            .finish()
    }
}

impl Serialize for MemoryTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MemoryTypes", 12)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("other", &self.other())?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("standard", &self.standard())?;
        state.serialize_field("fast_page_mode", &self.fast_page_mode())?;
        state.serialize_field("edo", &self.edo())?;
        state.serialize_field("parity", &self.parity())?;
        state.serialize_field("ecc", &self.ecc())?;
        state.serialize_field("simm", &self.simm())?;
        state.serialize_field("dimm", &self.dimm())?;
        state.serialize_field("burst_edo", &self.burst_edo())?;
        state.serialize_field("sdram", &self.sdram())?;
        state.end()
    }
}

/// # Memory Module Voltage
#[derive(PartialEq, Eq)]
pub struct ModuleVoltage {
    /// Raw value
    pub raw: u8,
}

impl Deref for ModuleVoltage {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for ModuleVoltage {
    fn from(raw: u8) -> Self {
        ModuleVoltage { raw }
    }
}

impl ModuleVoltage {
    /// Bit 0 5V
    pub fn volts_5(&self) -> bool {
        self.raw & 0x0001 == 0x0001
    }

    /// Bit 1 3.3V
    pub fn volts_3_3(&self) -> bool {
        self.raw & 0x0002 == 0x0002
    }

    /// Bit 2 2.9V
    pub fn volts_2_9(&self) -> bool {
        self.raw & 0x0004 == 0x0004
    }
}

impl fmt::Debug for ModuleVoltage {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<ModuleVoltage>())
            .field("raw", &self.raw)
            .field("volts_5", &self.volts_5())
            .field("volts_3_3", &self.volts_3_3())
            .field("volts_2_9", &self.volts_2_9())
            .finish()
    }
}

impl Serialize for ModuleVoltage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ModuleVoltage", 4)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("volts_5", &self.volts_5())?;
        state.serialize_field("volts_3_3", &self.volts_3_3())?;
        state.serialize_field("volts_2_9", &self.volts_2_9())?;
        state.end()
    }
}

/// # Memory Module Handle Iterator
///
/// Iterates over the memory module handles contained within the [SMBiosMemoryControllerInformation] structure
pub struct ModuleHandleIterator<'a> {
    data: &'a SMBiosMemoryControllerInformation<'a>,
    current_index: usize,
    current_entry: u8,
    number_of_handles: u8,
}

impl<'a> ModuleHandleIterator<'a> {
    const MODULE_HANDLES_OFFSET: usize = 0x0Fusize;

    /// Creates an instance of the memory module handle iterator.
    pub fn new(data: &'a SMBiosMemoryControllerInformation<'a>) -> Self {
        ModuleHandleIterator {
            data: data,
            current_index: Self::MODULE_HANDLES_OFFSET,
            current_entry: 0,
            number_of_handles: data.number_of_associated_memory_slots().unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::MODULE_HANDLES_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a ModuleHandleIterator<'a> {
    type Item = Handle;
    type IntoIter = ModuleHandleIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ModuleHandleIterator {
            data: self.data,
            current_index: ModuleHandleIterator::MODULE_HANDLES_OFFSET,
            current_entry: 0,
            number_of_handles: self.data.number_of_associated_memory_slots().unwrap_or(0),
        }
    }
}

impl<'a> Iterator for ModuleHandleIterator<'a> {
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_handles {
            self.reset();
            return None;
        }

        match self.data.parts().get_field_handle(self.current_index) {
            Some(current_handle) => {
                self.current_index = self.current_index + Handle::SIZE;
                self.current_entry += 1;
                Some(current_handle)
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for ModuleHandleIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

impl<'a> Serialize for ModuleHandleIterator<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let handles: Vec<Handle> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(handles.len()))?;
        for e in handles {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

/// # Memory Module Error Correcting Capabilities Iterator
///
/// Iterates over the memory module error correcting capabilities
/// contained within the [SMBiosMemoryControllerInformation] structure
pub struct ErrorCapabilitiesIterator<'a> {
    data: &'a SMBiosMemoryControllerInformation<'a>,
    start_index: usize,
    current_index: usize,
    current_entry: u8,
    number_of_items: u8,
}

impl<'a> ErrorCapabilitiesIterator<'a> {
    /// Creates an instance of the memory module error correcting capabilities iterator.
    pub fn new(data: &'a SMBiosMemoryControllerInformation<'a>) -> Self {
        let number_of_items = data.number_of_associated_memory_slots().unwrap_or(0);
        let start_index =
            number_of_items as usize * Handle::SIZE + ModuleHandleIterator::MODULE_HANDLES_OFFSET;
        ErrorCapabilitiesIterator {
            data,
            start_index,
            current_index: start_index,
            current_entry: 0,
            number_of_items,
        }
    }

    fn reset(&mut self) {
        self.current_index = self.start_index;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a ErrorCapabilitiesIterator<'a> {
    type Item = ErrorCorrectingCapabilities;
    type IntoIter = ErrorCapabilitiesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ErrorCapabilitiesIterator {
            data: self.data,
            start_index: self.start_index,
            current_index: self.start_index,
            current_entry: 0,
            number_of_items: self.number_of_items,
        }
    }
}

impl<'a> Iterator for ErrorCapabilitiesIterator<'a> {
    type Item = ErrorCorrectingCapabilities;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_items {
            self.reset();
            return None;
        }

        match self.data.parts().get_field_byte(self.current_index) {
            Some(current_item) => {
                self.current_index = self.current_index + ErrorCorrectingCapabilities::SIZE;
                self.current_entry += 1;
                Some(ErrorCorrectingCapabilities::from(current_item))
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for ErrorCapabilitiesIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

impl<'a> Serialize for ErrorCapabilitiesIterator<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let capabilities: Vec<ErrorCorrectingCapabilities> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(capabilities.len()))?;
        for e in capabilities {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type5 = vec![
            5u8, 0x15, 0x3F, 0x00,   // header
            0x04,   // error detecting Parity8Bit
            0b1000, // error correcting SingleBitErrorCorrecting
            0x03,   // interleave OneWay
            0x04,   // interleave TwoWay
            0x02,   // size
            0b10000, 0x00, // speeds NS50 (offsets 0x9-0xA)
            0b1000, 0x00, // types FastPageMode (offsets 0xB-0xC)
            0b10, // voltage Volts3dot3
            0x02, // slots
            0x0A, 0x00, // handle 0x000A (offsets 0xF-0x10)
            0x0B, 0x00,    // handle 0x000B (offsets 0x11-0x12)
            0b1000,  // error correcting 1, SingleBitErrorCorrecting
            0b10000, // error correcting 2, DoubleBitErrorCorrecting
            0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type5);
        let test_struct = SMBiosMemoryControllerInformation::new(&parts);

        assert_eq!(
            *test_struct.error_detecting_method().unwrap(),
            ErrorDetectingMethod::Parity8Bit
        );

        let error_correcting = test_struct.error_correcting_capability().unwrap();
        assert!(error_correcting.single_bit_error_correcting());

        assert_eq!(
            *test_struct.supported_interleave().unwrap(),
            InterleaveSupport::OneWay
        );

        assert_eq!(
            *test_struct.current_interleave().unwrap(),
            InterleaveSupport::TwoWay
        );

        assert_eq!(test_struct.maximum_memory_module_size(), Some(2));

        assert!(test_struct.supported_speeds().unwrap().ns50());

        assert!(test_struct
            .supported_memory_types()
            .unwrap()
            .fast_page_mode());

        assert!(test_struct.memory_module_voltage().unwrap().volts_3_3());

        assert_eq!(test_struct.number_of_associated_memory_slots(), Some(2));

        let mut iterator = test_struct
            .error_correcting_capabilities_iterator()
            .into_iter();
        let first = iterator.next().unwrap();
        assert!(first.single_bit_error_correcting());
        let second = iterator.next().unwrap();
        assert!(second.double_bit_error_correcting());
    }
}
