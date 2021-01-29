use crate::*;

/// # Processor Information (Type 4)
///
/// The information in this structure defines the attributes of a single processor; a separate
/// structure instance is provided for each system processor socket/slot. For example, a system with an
/// IntelDX2™ processor would have a single structure instance while a system with an IntelSX2™ processor
/// would have a structure to describe the main CPU and a second structure to describe the 80487 co1021 processor.
///
/// NOTE One structure is provided for each processor instance in a system. For example, a system that supports up
/// to two processors includes two Processor Information structures — even if only one processor is currently
/// installed. Software that interprets the SMBIOS information can count the Processor Information structures to
/// determine the maximum possible configuration of the system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosProcessorInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosProcessorInformation<'a> {
    const STRUCT_TYPE: u8 = 4u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosProcessorInformation<'a> {
    /// Socket reference designation
    ///
    /// EXAMPLE: "J202"
    pub fn socket_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Processor type
    pub fn processor_type(&self) -> Option<ProcessorTypeData> {
        self.parts
            .get_field_byte(0x05)
            .and_then(|raw| Some(ProcessorTypeData::from(raw)))
    }

    /// Processor family
    pub fn processor_family(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Processor manufacturer
    pub fn processor_manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Raw processor identification data
    pub fn processor_id(&self) -> Option<u64> {
        self.parts.get_field_qword(0x08)
    }

    /// Processor version
    pub fn processor_version(&self) -> Option<String> {
        self.parts.get_field_string(0x10)
    }

    /// Voltage
    pub fn voltage(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    /// External clock frequency, in MHz
    ///
    /// If the value is unknown, the field is set to 0.
    pub fn external_clock(&self) -> Option<u16> {
        self.parts.get_field_word(0x12)
    }

    /// Maximum processor speed (in MHz) supported
    /// by the system for this processor socket
    ///
    /// 0E9h is for a 233 MHz processor. If the value is
    /// unknown, the field is set to 0.
    ///
    /// NOTE: This field identifies a capability for the system,
    /// not the processor itself.
    pub fn max_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }

    /// Current speed
    ///
    /// Same format as Max Speed
    ///
    /// NOTE: This field identifies the processor's speed at
    /// system boot; the processor may support
    /// more than one speed.
    pub fn current_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x16)
    }

    /// Status bit field
    pub fn status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x18)
    }

    /// Processor upgrade
    pub fn processor_upgrade(&self) -> Option<ProcessorUpgradeData> {
        self.parts
            .get_field_byte(0x19)
            .and_then(|raw| Some(ProcessorUpgradeData::from(raw)))
    }

    /// Handle of a [SMBiosCacheInformation] structure that
    /// defines the attributes of the primary (Level 1)
    /// cache for this processor
    ///
    /// For version 2.1 and version 2.2
    /// implementations, the value is 0FFFFh if the
    /// processor has no L1 cache. For version 2.3 and
    /// later implementations, the value is 0FFFFh if
    /// the Cache Information structure is not provided.
    pub fn l1cache_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x1A)
    }

    /// Handle of a [SMBiosCacheInformation] structure that
    /// defines the attributes of the primary (Level 2)
    /// cache for this processor
    ///
    /// For version 2.1 and version 2.2
    /// implementations, the value is 0FFFFh if the
    /// processor has no L2 cache. For version 2.3 and
    /// later implementations, the value is 0FFFFh if
    /// the Cache Information structure is not provided.
    pub fn l2cache_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x1C)
    }

    /// Handle of a [SMBiosCacheInformation] structure that
    /// defines the attributes of the primary (Level 3)
    /// cache for this processor
    ///
    /// For version 2.1 and version 2.2
    /// implementations, the value is 0FFFFh if the
    /// processor has no L3 cache. For version 2.3 and
    /// later implementations, the value is 0FFFFh if
    /// the Cache Information structure is not provided.
    pub fn l3cache_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x1E)
    }

    /// The serial number of this processor
    ///
    /// This value is set by the manufacturer and
    /// normally not changeable.
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x20)
    }

    /// The asset tag of this processor
    pub fn asset_tag(&self) -> Option<String> {
        self.parts.get_field_string(0x21)
    }

    /// The part number of this processor
    ///
    /// This value is set by the manufacturer and
    /// normally not changeable.
    pub fn part_number(&self) -> Option<String> {
        self.parts.get_field_string(0x22)
    }

    /// Number of cores per processor socket
    ///
    /// If the value is unknown, the field is
    /// set to 0. For core counts of 256 or greater, the
    /// Core Count field is set to FFh and the Core
    /// Count 2 field is set to the number of cores.
    pub fn core_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x23)
    }

    /// Number of enabled cores per processor socket
    ///
    /// If the value is unknown, the field is
    /// set 0. For core counts of 256 or greater, the
    /// Core Enabled field is set to FFh and the Core
    /// Enabled 2 field is set to the number of enabled
    /// cores.
    pub fn core_enabled(&self) -> Option<u8> {
        self.parts.get_field_byte(0x24)
    }

    /// Number of threads per processor socket
    ///
    /// If the value is unknown, the field is
    /// set to 0. For thread counts of 256 or greater,
    /// the Thread Count field is set to FFh and the
    /// Thread Count 2 field is set to the number of
    /// threads.
    pub fn thread_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x25)
    }

    /// Defines which functions the processor supports
    pub fn processor_characteristics(&self) -> Option<ProcessorCharacteristics> {
        self.parts
            .get_field_word(0x26)
            .and_then(|raw| Some(ProcessorCharacteristics::from(raw)))
    }

    /// Processor family 2
    pub fn processor_family_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x28)
    }

    /// Number of Cores per processor socket.
    ///
    /// Supports core counts >255. If this field is
    /// present, it holds the core count for the
    /// processor socket. Core Count will also hold the
    /// core count, except for core counts that are 256
    /// or greater. In that case, Core Count shall be set
    /// to FFh and Core Count 2 will hold the count.
    ///
    /// Legal values:
    /// 0000h = unknown
    /// 0001h-00FFh = core counts 1 to 255.
    /// Matches Core Count value.
    /// 0100h-FFFEh = Core counts 256 to
    /// 65534, respectively.
    /// FFFFh = reserved.
    pub fn core_count_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2A)
    }

    /// Number of enabled cores per processor socket.
    ///
    /// Supports core enabled counts >255. If this field
    /// is present, it holds the core enabled count for
    /// the processor socket. Core Enabled will also
    /// hold the core enabled count, except for core
    /// counts that are 256 or greater. In that case,
    /// Core Enabled shall be set to FFh and Core
    /// Enabled 2 will hold the count.
    ///
    /// Legal values:
    /// 0000h = unknown
    /// 0001h-00FFh = core enabled counts 1 to
    /// 255. Matches Core Enabled value.
    /// 0100h-FFFEh = core enabled counts 256
    /// to 65534, respectively.
    /// FFFFh = reserved.
    pub fn core_enabled_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2C)
    }

    /// Number of threads per processor socket.
    ///
    /// Supports thread counts >255. If this field is
    /// present, it holds the thread count for the
    /// processor socket. Thread Count will also hold
    /// the thread count, except for thread counts that
    /// are 256 or greater. In that case, Thread Count
    /// shall be set to FFh and Thread Count 2 will
    /// hold the count.
    ///
    /// Legal values:0000h = unknown
    /// 0001h-00FFh = thread counts 1 to 255.
    /// Matches Thread Count value.
    /// 0100h-FFFEh = thread counts 256 to
    /// 65534, respectively.
    /// FFFFh = reserved.
    pub fn thread_count_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2E)
    }
}

impl fmt::Debug for SMBiosProcessorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosProcessorInformation>())
            .field("header", &self.parts.header)
            .field("socket_designation", &self.socket_designation())
            .field("processor_type", &self.processor_type())
            .field("processor_family", &self.processor_family())
            .field("processor_manufacturer", &self.processor_manufacturer())
            .field("processor_id", &self.processor_id())
            .field("processor_version", &self.processor_version())
            .field("voltage", &self.voltage())
            .field("external_clock", &self.external_clock())
            .field("max_speed", &self.max_speed())
            .field("current_speed", &self.current_speed())
            .field("status", &self.status())
            .field("processor_upgrade", &self.processor_upgrade())
            .field("l1cache_handle", &self.l1cache_handle())
            .field("l2cache_handle", &self.l2cache_handle())
            .field("l3cache_handle", &self.l3cache_handle())
            .field("serial_number", &self.serial_number())
            .field("asset_tag", &self.asset_tag())
            .field("part_number", &self.part_number())
            .field("core_count", &self.core_count())
            .field("core_enabled", &self.core_enabled())
            .field("thread_count", &self.thread_count())
            .field(
                "processor_characteristics",
                &self.processor_characteristics(),
            )
            .field("processor_family_2", &self.processor_family_2())
            .field("core_count_2", &self.core_count_2())
            .field("core_enabled_2", &self.core_enabled_2())
            .field("thread_count_2", &self.thread_count_2())
            .finish()
    }
}

/// # Processor Type Data
pub struct ProcessorTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ProcessorType] value
    pub value: ProcessorType,
}

impl fmt::Debug for ProcessorTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for ProcessorTypeData {
    type Target = ProcessorType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Processor Type
#[derive(Debug, PartialEq, Eq)]
pub enum ProcessorType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Central Processor
    CentralProcessor,
    /// Math Processor
    MathProcessor,
    /// DSP Processor
    DspProcessor,
    /// Video Processor
    VideoProcessor,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ProcessorTypeData {
    fn from(raw: u8) -> Self {
        ProcessorTypeData {
            value: match raw {
                0x01 => ProcessorType::Other,
                0x02 => ProcessorType::Unknown,
                0x03 => ProcessorType::CentralProcessor,
                0x04 => ProcessorType::MathProcessor,
                0x05 => ProcessorType::DspProcessor,
                0x06 => ProcessorType::VideoProcessor,
                _ => ProcessorType::None,
            },
            raw,
        }
    }
}

/// #
pub struct ProcessorUpgradeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ProcessorUpgrade] value
    pub value: ProcessorUpgrade,
}

impl fmt::Debug for ProcessorUpgradeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorUpgradeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for ProcessorUpgradeData {
    type Target = ProcessorUpgrade;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// #
#[derive(Debug, PartialEq, Eq)]
pub enum ProcessorUpgrade {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Daughter Board
    DaughterBoard,
    /// ZIF Socket
    ZIFSocket,
    /// Replaceable Piggy Back
    ReplaceablePiggyBack,
    /// No Upgrade
    NoUpgrade,
    /// LIF Socket
    LIFSocket,
    /// Slot 1
    Slot1,
    /// Slot 2
    Slot2,
    /// 370-pin socket
    PinSocket370,
    /// Slot A
    SlotA,
    /// Slot M
    SlotM,
    /// Socket 423
    Socket423,
    /// Socket A (Socket 462)
    SocketASocket462,
    /// Socket 478
    Socket478,
    /// Socket 754
    Socket754,
    /// Socket 940
    Socket940,
    /// Socket 939
    Socket939,
    /// Socket mPGA604
    SocketmPGA604,
    /// Socket LGA771
    SocketLGA771,
    /// Socket LGA775
    SocketLGA775,
    /// Socket S1
    SocketS1,
    /// Socket AM2
    SocketAM2,
    /// Socket F (1207)
    SocketF1207,
    /// Socket LGA1366
    SocketLGA1366,
    /// Socket G34
    SocketG34,
    /// Socket AM3
    SocketAM3,
    /// Socket C32
    SocketC32,
    /// Socket LGA1156
    SocketLGA1156,
    /// Socket LGA1567
    SocketLGA1567,
    /// Socket PGA988A
    SocketPGA988A,
    /// Socket BGA1288
    SocketBGA1288,
    /// Socket rPGA988B
    SocketrPGA988B,
    /// Socket BGA1023
    SocketBGA1023,
    /// Socket BGA1224
    SocketBGA1224,
    /// Socket LGA1155
    SocketLGA1155,
    /// Socket LGA1356
    SocketLGA1356,
    /// Socket LGA2011
    SocketLGA2011,
    /// Socket FS1
    SocketFS1,
    /// Socket FS2
    SocketFS2,
    /// Socket FM1
    SocketFM1,
    /// Socket FM2
    SocketFM2,
    /// Socket LGA2011-3
    SocketLGA2011_3,
    /// Socket LGA1356-3
    SocketLGA1356_3,
    /// Socket LGA1150
    SocketLGA1150,
    /// Socket BGA1168
    SocketBGA1168,
    /// Socket BGA1234
    SocketBGA1234,
    /// Socket BGA1364
    SocketBGA1364,
    /// Socket AM4
    SocketAM4,
    /// Socket LGA1151
    SocketLGA1151,
    /// Socket BGA1356
    SocketBGA1356,
    /// Socket BGA1440
    SocketBGA1440,
    /// Socket BGA1515
    SocketBGA1515,
    /// Socket LGA3647-1
    SocketLGA3647_1,
    /// Socket SP3
    SocketSP3,
    /// Socket SP3r2
    SocketSP3r23,
    /// Socket LGA2066
    SocketLGA2066,
    /// Socket BGA1392
    SocketBGA1392,
    /// Socket BGA1510
    SocketBGA1510,
    /// Socket BGA1528
    SocketBGA1528,
    /// Socket LGA4189
    SocketLGA4189,
    /// Socket LGA1200
    SocketLGA1200,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ProcessorUpgradeData {
    fn from(raw: u8) -> Self {
        ProcessorUpgradeData {
            value: match raw {
                0x01 => ProcessorUpgrade::Other,
                0x02 => ProcessorUpgrade::Unknown,
                0x03 => ProcessorUpgrade::DaughterBoard,
                0x04 => ProcessorUpgrade::ZIFSocket,
                0x05 => ProcessorUpgrade::ReplaceablePiggyBack,
                0x06 => ProcessorUpgrade::NoUpgrade,
                0x07 => ProcessorUpgrade::LIFSocket,
                0x08 => ProcessorUpgrade::Slot1,
                0x09 => ProcessorUpgrade::Slot2,
                0x0A => ProcessorUpgrade::PinSocket370,
                0x0B => ProcessorUpgrade::SlotA,
                0x0C => ProcessorUpgrade::SlotM,
                0x0D => ProcessorUpgrade::Socket423,
                0x0E => ProcessorUpgrade::SocketASocket462,
                0x0F => ProcessorUpgrade::Socket478,
                0x10 => ProcessorUpgrade::Socket754,
                0x11 => ProcessorUpgrade::Socket940,
                0x12 => ProcessorUpgrade::Socket939,
                0x13 => ProcessorUpgrade::SocketmPGA604,
                0x14 => ProcessorUpgrade::SocketLGA771,
                0x15 => ProcessorUpgrade::SocketLGA775,
                0x16 => ProcessorUpgrade::SocketS1,
                0x17 => ProcessorUpgrade::SocketAM2,
                0x18 => ProcessorUpgrade::SocketF1207,
                0x19 => ProcessorUpgrade::SocketLGA1366,
                0x1A => ProcessorUpgrade::SocketG34,
                0x1B => ProcessorUpgrade::SocketAM3,
                0x1C => ProcessorUpgrade::SocketC32,
                0x1D => ProcessorUpgrade::SocketLGA1156,
                0x1E => ProcessorUpgrade::SocketLGA1567,
                0x1F => ProcessorUpgrade::SocketPGA988A,
                0x20 => ProcessorUpgrade::SocketBGA1288,
                0x21 => ProcessorUpgrade::SocketrPGA988B,
                0x22 => ProcessorUpgrade::SocketBGA1023,
                0x23 => ProcessorUpgrade::SocketBGA1224,
                0x24 => ProcessorUpgrade::SocketLGA1155,
                0x25 => ProcessorUpgrade::SocketLGA1356,
                0x26 => ProcessorUpgrade::SocketLGA2011,
                0x27 => ProcessorUpgrade::SocketFS1,
                0x28 => ProcessorUpgrade::SocketFS2,
                0x29 => ProcessorUpgrade::SocketFM1,
                0x2A => ProcessorUpgrade::SocketFM2,
                0x2B => ProcessorUpgrade::SocketLGA2011_3,
                0x2C => ProcessorUpgrade::SocketLGA1356_3,
                0x2D => ProcessorUpgrade::SocketLGA1150,
                0x2E => ProcessorUpgrade::SocketBGA1168,
                0x2F => ProcessorUpgrade::SocketBGA1234,
                0x30 => ProcessorUpgrade::SocketBGA1364,
                0x31 => ProcessorUpgrade::SocketAM4,
                0x32 => ProcessorUpgrade::SocketLGA1151,
                0x33 => ProcessorUpgrade::SocketBGA1356,
                0x34 => ProcessorUpgrade::SocketBGA1440,
                0x35 => ProcessorUpgrade::SocketBGA1515,
                0x36 => ProcessorUpgrade::SocketLGA3647_1,
                0x37 => ProcessorUpgrade::SocketSP3,
                0x38 => ProcessorUpgrade::SocketSP3r23,
                0x39 => ProcessorUpgrade::SocketLGA2066,
                0x3A => ProcessorUpgrade::SocketBGA1392,
                0x3B => ProcessorUpgrade::SocketBGA1510,
                0x3C => ProcessorUpgrade::SocketBGA1528,
                0x3D => ProcessorUpgrade::SocketLGA4189,
                0x3E => ProcessorUpgrade::SocketLGA1200,
                _ => ProcessorUpgrade::None,
            },
            raw,
        }
    }
}

/// # Processor Characteristics
#[derive(PartialEq, Eq)]
pub struct ProcessorCharacteristics {
    /// Raw value
    pub raw: u16,
}

impl Deref for ProcessorCharacteristics {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for ProcessorCharacteristics {
    fn from(raw: u16) -> Self {
        ProcessorCharacteristics { raw }
    }
}

impl ProcessorCharacteristics {
    /// Bit 1 Unknown
    pub fn unknown(&self) -> bool {
        self.raw & 0x0002 == 0x0002
    }

    /// Bit 2 64-bit Capable
    pub fn bit_64capable(&self) -> bool {
        self.raw & 0x0004 == 0x0004
    }

    /// Bit 3 Multi-Core
    pub fn multi_core(&self) -> bool {
        self.raw & 0x0008 == 0x0008
    }

    /// Bit 4 Hardware Thread
    pub fn hardware_thread(&self) -> bool {
        self.raw & 0x0010 == 0x0010
    }

    /// Bit 5 Execute Protection
    pub fn execute_protection(&self) -> bool {
        self.raw & 0x0020 == 0x0020
    }

    /// Bit 6 Enhanced Virtualization
    pub fn enhanced_virtualization(&self) -> bool {
        self.raw & 0x0040 == 0x0040
    }

    /// Bit 7 Power/Performance Control
    pub fn power_performance_control(&self) -> bool {
        self.raw & 0x0080 == 0x0080
    }

    /// Bit 8 128-bit Capable
    pub fn bit_128capable(&self) -> bool {
        self.raw & 0x0100 == 0x0100
    }

    /// Bit 9 Arm64 SoC ID
    pub fn arm_64soc_id(&self) -> bool {
        self.raw & 0x200 == 0x200
    }
}

impl fmt::Debug for ProcessorCharacteristics {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorCharacteristics>())
            .field("raw", &self.raw)
            .field("unknown", &self.unknown())
            .field("bit_64capable", &self.bit_64capable())
            .field("multi_core", &self.multi_core())
            .field("hardware_thread", &self.hardware_thread())
            .field("execute_protection", &self.execute_protection())
            .field("enhanced_virtualization", &self.enhanced_virtualization())
            .field(
                "power_performance_control",
                &self.power_performance_control(),
            )
            .field("bit_128capable", &self.bit_128capable())
            .field("arm_64soc_id", &self.arm_64soc_id())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type4 = vec![
            0x04, 0x30, 0x56, 0x00, 0x01, 0x03, 0xB3, 0x02, 0x54, 0x06, 0x05, 0x00, 0xFF, 0xFB,
            0xEB, 0xBF, 0x03, 0x90, 0x64, 0x00, 0x3C, 0x0F, 0x10, 0x0E, 0x41, 0x01, 0x53, 0x00,
            0x54, 0x00, 0x55, 0x00, 0x00, 0x04, 0x00, 0x06, 0x06, 0x0C, 0xFC, 0x00, 0xB3, 0x00,
            0x06, 0x00, 0x06, 0x00, 0x0C, 0x00, 0x43, 0x50, 0x55, 0x30, 0x00, 0x49, 0x6E, 0x74,
            0x65, 0x6C, 0x28, 0x52, 0x29, 0x20, 0x43, 0x6F, 0x72, 0x70, 0x6F, 0x72, 0x61, 0x74,
            0x69, 0x6F, 0x6E, 0x00, 0x49, 0x6E, 0x74, 0x65, 0x6C, 0x28, 0x52, 0x29, 0x20, 0x58,
            0x65, 0x6F, 0x6E, 0x28, 0x52, 0x29, 0x20, 0x57, 0x2D, 0x32, 0x31, 0x33, 0x33, 0x20,
            0x43, 0x50, 0x55, 0x20, 0x40, 0x20, 0x33, 0x2E, 0x36, 0x30, 0x47, 0x48, 0x7A, 0x00,
            0x55, 0x4E, 0x4B, 0x4E, 0x4F, 0x57, 0x4E, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type4.as_slice());
        let test_struct = SMBiosProcessorInformation::new(&parts);

        assert_eq!(test_struct.socket_designation(), Some("CPU0".to_string()));
        assert_eq!(
            *test_struct.processor_type().unwrap(),
            ProcessorType::CentralProcessor
        );
        assert_eq!(test_struct.processor_family(), Some(179));
        assert_eq!(
            test_struct.processor_manufacturer(),
            Some("Intel(R) Corporation".to_string())
        );
        assert_eq!(test_struct.processor_id(), Some(13829424153406801492));
        assert_eq!(
            test_struct.processor_version(),
            Some("Intel(R) Xeon(R) W-2133 CPU @ 3.60GHz".to_string())
        );
        assert_eq!(test_struct.voltage(), Some(144));
        assert_eq!(test_struct.external_clock(), Some(100));
        assert_eq!(test_struct.max_speed(), Some(3900));
        assert_eq!(test_struct.current_speed(), Some(3600));
        assert_eq!(test_struct.status(), Some(65));
        assert_eq!(
            *test_struct.processor_upgrade().unwrap(),
            ProcessorUpgrade::Other
        );
        assert_eq!(*test_struct.l1cache_handle().unwrap(), 83);
        assert_eq!(*test_struct.l2cache_handle().unwrap(), 84);
        assert_eq!(*test_struct.l3cache_handle().unwrap(), 85);
        assert_eq!(test_struct.serial_number(), None);
        assert_eq!(test_struct.asset_tag(), Some("UNKNOWN".to_string()));
        assert_eq!(test_struct.part_number(), None);
        assert_eq!(test_struct.core_count(), Some(6));
        assert_eq!(test_struct.core_enabled(), Some(6));
        assert_eq!(test_struct.thread_count(), Some(12));
        assert_eq!(
            test_struct.processor_characteristics(),
            Some(ProcessorCharacteristics::from(252))
        );
        assert_eq!(test_struct.processor_family_2(), Some(179));
        assert_eq!(test_struct.core_count_2(), Some(6));
        assert_eq!(test_struct.core_enabled_2(), Some(6));
        assert_eq!(test_struct.thread_count_2(), Some(12));

        println!("{:?}", test_struct);
    }
}
