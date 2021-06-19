use crate::core::{Handle, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::convert::TryInto;
use std::fmt;
use std::ops::Deref;

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
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosProcessorInformation<'a> {
    const STRUCT_TYPE: u8 = 4u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
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
            .map(|raw| ProcessorTypeData::from(raw))
    }

    /// Processor family
    pub fn processor_family(&self) -> Option<ProcessorFamilyData> {
        self.parts
            .get_field_byte(0x06)
            .map(|raw| ProcessorFamilyData::from(raw))
    }

    /// Processor manufacturer
    pub fn processor_manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Raw processor identification data
    pub fn processor_id(&self) -> Option<&[u8; 8]> {
        // Note: There are two more levels to the design of ProcessorId to consider.
        // 1. These 8 bytes can represent one of 4 classes of processor (see 7.5.3)
        //    perhaps return an enum with 4 variants.
        // 2. Per variant the data represents specific Id information which can be
        //    accomodated in specific representative structures.
        self.parts
            .get_field_data(0x08, 0x10)
            .map(|raw| raw.try_into().expect("incorrect length"))
    }

    /// Processor version
    pub fn processor_version(&self) -> Option<String> {
        self.parts.get_field_string(0x10)
    }

    /// Voltage
    pub fn voltage(&self) -> Option<ProcessorVoltage> {
        self.parts
            .get_field_byte(0x11)
            .map(|raw| ProcessorVoltage::from(raw))
    }

    /// External clock frequency, in MHz
    ///
    /// If the value is unknown, the field is set to 0.
    pub fn external_clock(&self) -> Option<ProcessorExternalClock> {
        self.parts
            .get_field_word(0x12)
            .map(|raw| ProcessorExternalClock::from(raw))
    }

    /// Maximum processor speed (in MHz) supported
    /// by the system for this processor socket
    ///
    /// 0E9h is for a 233 MHz processor.
    ///
    /// NOTE: This field identifies a capability for the system,
    /// not the processor itself.
    pub fn max_speed(&self) -> Option<ProcessorSpeed> {
        self.parts
            .get_field_word(0x14)
            .map(|raw| ProcessorSpeed::from(raw))
    }

    /// Current speed
    ///
    /// Same format as Max Speed
    ///
    /// NOTE: This field identifies the processor's speed at
    /// system boot; the processor may support
    /// more than one speed.
    pub fn current_speed(&self) -> Option<ProcessorSpeed> {
        self.parts
            .get_field_word(0x16)
            .map(|raw| ProcessorSpeed::from(raw))
    }

    /// Status bit field
    pub fn status(&self) -> Option<ProcessorStatus> {
        self.parts
            .get_field_byte(0x18)
            .map(|raw| ProcessorStatus::from(raw))
    }

    /// Processor upgrade
    pub fn processor_upgrade(&self) -> Option<ProcessorUpgradeData> {
        self.parts
            .get_field_byte(0x19)
            .map(|raw| ProcessorUpgradeData::from(raw))
    }

    /// Handle of a [super::SMBiosCacheInformation] structure that
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

    /// Handle of a [super::SMBiosCacheInformation] structure that
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

    /// Handle of a [super::SMBiosCacheInformation] structure that
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
    /// For core counts of 256 or greater, the
    /// 'core_count_2' field is set to the number of cores.
    pub fn core_count(&self) -> Option<CoreCount> {
        self.parts
            .get_field_byte(0x23)
            .map(|raw| CoreCount::from(raw))
    }

    /// Number of enabled cores per processor socket
    ///
    /// For core counts of 256 or greater, the
    /// 'cores_enabled_2' field is set to the number of enabled
    /// cores.
    pub fn cores_enabled(&self) -> Option<CoresEnabled> {
        self.parts
            .get_field_byte(0x24)
            .map(|raw| CoresEnabled::from(raw))
    }

    /// Number of threads per processor socket
    ///
    /// For thread counts of 256 or greater,
    /// 'thread_count_2' field is set to the number of
    /// threads.
    pub fn thread_count(&self) -> Option<ThreadCount> {
        self.parts
            .get_field_byte(0x25)
            .map(|raw| ThreadCount::from(raw))
    }

    /// Defines which functions the processor supports
    pub fn processor_characteristics(&self) -> Option<ProcessorCharacteristics> {
        self.parts
            .get_field_word(0x26)
            .map(|raw| ProcessorCharacteristics::from(raw))
    }

    /// Processor family 2
    pub fn processor_family_2(&self) -> Option<ProcessorFamilyData2> {
        self.parts
            .get_field_word(0x28)
            .map(|raw| ProcessorFamilyData2::from(raw))
    }

    /// Number of Cores per processor socket.
    ///
    /// Supports core counts >255. If this field is
    /// present, it holds the core count for the
    /// processor socket. 'core_count' will also hold the
    /// core count, except for core counts that are 256
    /// or greater. In that case, 'core_count' shall be set
    /// to 'CoreCount::SeeCoreCount2' and 'core_count_2' will hold the count.
    pub fn core_count_2(&self) -> Option<CoreCount2> {
        self.parts
            .get_field_word(0x2A)
            .map(|raw| CoreCount2::from(raw))
    }

    /// Number of enabled cores per processor socket.
    ///
    /// Supports core enabled counts >255. If this field
    /// is present, it holds the core enabled count for
    /// the processor socket. 'cores_enabled' will also
    /// hold the core enabled count, except for core
    /// counts that are 256 or greater. In that case,
    /// 'cores_enabled' shall be set to 'CoresEnabled::SeeCoresEnabled2'
    /// and 'cores_enabled_2' will hold the count.
    pub fn cores_enabled_2(&self) -> Option<CoresEnabled2> {
        self.parts
            .get_field_word(0x2C)
            .map(|raw| CoresEnabled2::from(raw))
    }

    /// Number of threads per processor socket.
    ///
    /// Supports thread counts >255. If this field is
    /// present, it holds the thread count for the
    /// processor socket. 'thread_count' will also hold
    /// the thread count, except for thread counts that
    /// are 256 or greater. In that case, 'thread_count'
    /// shall be set to 'ThreadCount::SeeThreadCount2'
    /// and 'thread_count_2' will hold the count.
    pub fn thread_count_2(&self) -> Option<ThreadCount2> {
        self.parts
            .get_field_word(0x2E)
            .map(|raw| ThreadCount2::from(raw))
    }
}

impl fmt::Debug for SMBiosProcessorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosProcessorInformation<'_>>())
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
            .field("cores_enabled", &self.cores_enabled())
            .field("thread_count", &self.thread_count())
            .field(
                "processor_characteristics",
                &self.processor_characteristics(),
            )
            .field("processor_family_2", &self.processor_family_2())
            .field("core_count_2", &self.core_count_2())
            .field("cores_enabled_2", &self.cores_enabled_2())
            .field("thread_count_2", &self.thread_count_2())
            .finish()
    }
}

impl Serialize for SMBiosProcessorInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosProcessorInformation", 27)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("socket_designation", &self.socket_designation())?;
        state.serialize_field("processor_type", &self.processor_type())?;
        state.serialize_field("processor_family", &self.processor_family())?;
        state.serialize_field("processor_manufacturer", &self.processor_manufacturer())?;
        state.serialize_field("processor_id", &self.processor_id())?;
        state.serialize_field("processor_version", &self.processor_version())?;
        state.serialize_field("voltage", &self.voltage())?;
        state.serialize_field("external_clock", &self.external_clock())?;
        state.serialize_field("max_speed", &self.max_speed())?;
        state.serialize_field("current_speed", &self.current_speed())?;
        state.serialize_field("status", &self.status())?;
        state.serialize_field("processor_upgrade", &self.processor_upgrade())?;
        state.serialize_field("l1cache_handle", &self.l1cache_handle())?;
        state.serialize_field("l2cache_handle", &self.l2cache_handle())?;
        state.serialize_field("l3cache_handle", &self.l3cache_handle())?;
        state.serialize_field("serial_number", &self.serial_number())?;
        state.serialize_field("asset_tag", &self.asset_tag())?;
        state.serialize_field("part_number", &self.part_number())?;
        state.serialize_field("core_count", &self.core_count())?;
        state.serialize_field("cores_enabled", &self.cores_enabled())?;
        state.serialize_field("thread_count", &self.thread_count())?;
        state.serialize_field(
            "processor_characteristics",
            &self.processor_characteristics(),
        )?;
        state.serialize_field("processor_family_2", &self.processor_family_2())?;
        state.serialize_field("core_count_2", &self.core_count_2())?;
        state.serialize_field("cores_enabled_2", &self.cores_enabled_2())?;
        state.serialize_field("thread_count_2", &self.thread_count_2())?;
        state.end()
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ProcessorTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ProcessorTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ProcessorType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ProcessorTypeData {
    type Target = ProcessorType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Processor Type
#[derive(Serialize, Debug, PartialEq, Eq)]
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

/// # Processor Family Data
pub struct ProcessorFamilyData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ProcessorFamily] value
    pub value: ProcessorFamily,
}

impl fmt::Debug for ProcessorFamilyData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorFamilyData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ProcessorFamilyData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorFamilyData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ProcessorFamilyData {
    /// Displays ProcessorFamily either by name or as a hex value if the name for the value is unknown.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ProcessorFamily::None => write!(f, "{:#X}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ProcessorFamilyData {
    type Target = ProcessorFamily;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for ProcessorFamilyData {
    fn from(raw: u8) -> Self {
        ProcessorFamilyData {
            value: ProcessorFamily::from(raw as u16),
            raw,
        }
    }
}

/// # Processor Family Data #2
pub struct ProcessorFamilyData2 {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u16,
    /// The contained [ProcessorFamily] value
    pub value: ProcessorFamily,
}

impl fmt::Debug for ProcessorFamilyData2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorFamilyData2>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ProcessorFamilyData2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorFamilyData2", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ProcessorFamilyData2 {
    /// Displays ProcessorFamily either by name or as a hex value if the name for the value is unknown.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ProcessorFamily::None => write!(f, "{:#X}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ProcessorFamilyData2 {
    type Target = ProcessorFamily;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u16> for ProcessorFamilyData2 {
    fn from(raw: u16) -> Self {
        ProcessorFamilyData2 {
            value: ProcessorFamily::from(raw),
            raw,
        }
    }
}
/// # Processor Family
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ProcessorFamily {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// 8086
    I8086,
    /// 80286
    I80286,
    /// Intel386™ processor
    Intel386Processor,
    /// Intel486™ processor
    Intel486Processor,
    /// 8087
    I8087,
    /// 80287
    I80287,
    /// 80387
    I80387,
    /// 80487
    I80487,
    /// Intel® Pentium® processor
    IntelPentiumProcessor,
    /// Pentium® Pro processor
    PentiumProProcessor,
    /// Pentium® II processor
    PentiumIIProcessor,
    /// Pentium® processor with MMX™ technology
    PentiumprocessorwithMMXtechnology,
    /// Intel® Celeron® processor
    IntelCeleronProcessor,
    /// Pentium® II Xeon™ processor
    PentiumIIXeonProcessor,
    /// Pentium® III processor
    PentiumIIIProcessor,
    /// M1 Family
    M1Family,
    /// M2 Family
    M2Family,
    /// Intel® Celeron® M processor
    IntelCeleronMProcessor,
    /// Intel® Pentium® 4 HT processor
    IntelPentium4HTProcessor,
    /// AMD Duron™ Processor Family
    AMDDuronProcessorFamily,
    /// K5 Family
    K5Family,
    /// K6 Family
    K6Family,
    /// K6-2
    K62,
    /// K6-3
    K63,
    /// AMD Athlon™ Processor Family
    AMDAthlonProcessorFamily,
    /// AMD29000 Family
    AMD29000Family,
    /// K6-2+
    K62Plus,
    /// Power PC Family
    PowerPCFamily,
    /// Power PC 601
    PowerPC601,
    /// Power PC 603
    PowerPC603,
    /// Power PC 603+
    PowerPC603Plus,
    /// Power PC 604
    PowerPC604,
    /// Power PC 620
    PowerPC620,
    /// Power PC x704
    PowerPCx704,
    /// Power PC 750
    PowerPC750,
    /// Intel® Core™ Duo processor
    IntelCoreDuoProcessor,
    /// Intel® Core™ Duo mobile processor
    IntelCoreDuomobileProcessor,
    /// Intel® Core™ Solo mobile processor
    IntelCoreSolomobileProcessor,
    /// Intel® Atom™ processor
    IntelAtomProcessor,
    /// Intel® Core™ M processor
    IntelCoreMProcessor,
    /// Intel(R) Core(TM) m3 processor
    IntelCorem3Processor,
    /// Intel(R) Core(TM) m5 processor
    IntelCorem5Processor,
    /// Intel(R) Core(TM) m7 processor
    IntelCorem7Processor,
    /// Alpha Family
    AlphaFamily,
    /// Alpha 21064
    Alpha21064,
    /// Alpha 21066
    Alpha21066,
    /// Alpha 21164
    Alpha21164,
    /// Alpha 21164PC
    Alpha21164PC,
    /// Alpha 21164a
    Alpha21164a,
    /// Alpha 21264
    Alpha21264,
    /// Alpha 21364
    Alpha21364,
    /// AMD Turion™ II Ultra Dual-Core Mobile M Processor Family
    AMDTurionIIUltraDualCoreMobileMProcessorFamily,
    /// AMD Turion™ II Dual-Core Mobile M Processor Family
    AMDTurionIIDualCoreMobileMProcessorFamily,
    /// AMD Athlon™ II Dual-Core M Processor Family
    AMDAthlonIIDualCoreMProcessorFamily,
    /// AMD Opteron™ 6100 Series Processor
    AMDOpteron6100SeriesProcessor,
    /// AMD Opteron™ 4100 Series Processor
    AMDOpteron4100SeriesProcessor,
    /// AMD Opteron™ 6200 Series Processor
    AMDOpteron6200SeriesProcessor,
    /// AMD Opteron™ 4200 Series Processor
    AMDOpteron4200SeriesProcessor,
    /// AMD FX™ Series Processor
    AMDFXSeriesProcessor,
    /// MIPS Family
    MIPSFamily,
    /// MIPS R4000
    MIPSR4000,
    /// MIPS R4200
    MIPSR4200,
    /// MIPS R4400
    MIPSR4400,
    /// MIPS R4600
    MIPSR4600,
    /// MIPS R10000
    MIPSR10000,
    /// AMD C-Series Processor
    AMDCSeriesProcessor,
    /// AMD E-Series Processor
    AMDESeriesProcessor,
    /// AMD A-Series Processor
    AMDASeriesProcessor,
    /// AMD G-Series Processor
    AMDGSeriesProcessor,
    /// AMD Z-Series Processor
    AMDZSeriesProcessor,
    /// AMD R-Series Processor
    AMDRSeriesProcessor,
    /// AMD Opteron™ 4300 Series Processor
    AMDOpteron4300SeriesProcessor,
    /// AMD Opteron™ 6300 Series Processor
    AMDOpteron6300SeriesProcessor,
    /// AMD Opteron™ 3300 Series Processor
    AMDOpteron3300SeriesProcessor,
    /// AMD FirePro™ Series Processor
    AMDFireProSeriesProcessor,
    /// SPARC Family
    SPARCFamily,
    /// SuperSPARC
    SuperSPARC,
    /// microSPARC II
    MicroSparcii,
    /// microSPARC IIep
    MicroSparciiep,
    /// UltraSPARC
    UltraSPARC,
    /// UltraSPARC II
    UltraSPARCII,
    /// UltraSPARC Iii
    UltraSPARCIii,
    /// UltraSPARC III
    UltraSPARCIII,
    /// UltraSPARC IIIi
    UltraSPARCIIIi,
    /// 68040 Family
    M68040Family,
    /// 68xxx
    M68xxx,
    /// 68000
    M68000,
    /// 68010
    M68010,
    /// 68020
    M68020,
    /// 68030
    M68030,
    /// AMD Athlon(TM) X4 Quad-Core Processor Family
    AMDAthlonX4QuadCoreProcessorFamily,
    /// AMD Opteron(TM) X1000 Series Processor
    AMDOpteronX1000SeriesProcessor,
    /// AMD Opteron(TM) X2000 Series APU
    AMDOpteronX2000SeriesAPU,
    /// AMD Opteron(TM) A-Series Processor
    AMDOpteronASeriesProcessor,
    /// AMD Opteron(TM) X3000 Series APU
    AMDOpteronX3000SeriesAPU,
    /// AMD Zen Processor Family
    AMDZenProcessorFamily,
    /// Hobbit Family
    HobbitFamily,
    /// Crusoe™ TM5000 Family
    CrusoeTM5000Family,
    /// Crusoe™ TM3000 Family
    CrusoeTM3000Family,
    /// Efficeon™ TM8000 Family
    EfficeonTM8000Family,
    /// Weitek
    Weitek,
    /// Itanium™ processor
    Itaniumprocessor,
    /// AMD Athlon™ 64 Processor Family
    AMDAthlon64ProcessorFamily,
    /// AMD Opteron™ Processor Family
    AMDOpteronProcessorFamily,
    /// AMD Sempron™ Processor Family
    AMDSempronProcessorFamily,
    /// AMD Turion™ 64 Mobile Technology
    AMDTurion64MobileTechnology,
    /// Dual-Core AMD Opteron™ Processor Family
    DualCoreAMDOpteronProcessorFamily,
    /// AMD Athlon™ 64 X2 Dual-Core Processor Family
    AMDAthlon64X2DualCoreProcessorFamily,
    /// AMD Turion™ 64 X2 Mobile Technology
    AMDTurion64X2MobileTechnology,
    /// Quad-Core AMD Opteron™ Processor Family
    QuadCoreAMDOpteronProcessorFamily,
    /// Third-Generation AMD Opteron™ Processor Family
    ThirdGenerationAMDOpteronProcessorFamily,
    /// AMD Phenom™ FX Quad-Core Processor Family
    AMDPhenomFXQuadCoreProcessorFamily,
    /// AMD Phenom™ X4 Quad-Core Processor Family
    AMDPhenomX4QuadCoreProcessorFamily,
    /// AMD Phenom™ X2 Dual-Core Processor Family
    AMDPhenomX2DualCoreProcessorFamily,
    /// AMD Athlon™ X2 Dual-Core Processor Family
    AMDAthlonX2DualCoreProcessorFamily,
    /// PA-RISC Family
    PARISCFamily,
    /// PA-RISC 8500
    PARISC8500,
    /// PA-RISC 8000
    PARISC8000,
    /// PA-RISC 7300LC
    PARISC7300LC,
    /// PA-RISC 7200
    PARISC7200,
    /// PA-RISC 7100LC
    PARISC7100LC,
    /// PA-RISC 7100
    PARISC7100,
    /// V30 Family
    V30Family,
    /// Quad-Core Intel® Xeon® processor 3200 Series
    QuadCoreIntelXeonProcessor3200Series,
    /// Dual-Core Intel® Xeon® processor 3000 Series
    DualCoreIntelXeonProcessor3000Series,
    /// Quad-Core Intel® Xeon® processor 5300 Series
    QuadCoreIntelXeonProcessor5300Series,
    /// Dual-Core Intel® Xeon® processor 5100 Series
    DualCoreIntelXeonProcessor5100Series,
    /// Dual-Core Intel® Xeon® processor 5000 Series
    DualCoreIntelXeonProcessor5000Series,
    /// Dual-Core Intel® Xeon® processor LV
    DualCoreIntelXeonProcessorLV,
    /// Dual-Core Intel® Xeon® processor ULV
    DualCoreIntelXeonProcessorULV,
    /// Dual-Core Intel® Xeon® processor 7100 Series
    DualCoreIntelXeonProcessor7100Series,
    /// Quad-Core Intel® Xeon® processor 5400 Series
    QuadCoreIntelXeonProcessor5400Series,
    /// Quad-Core Intel® Xeon® processor
    QuadCoreIntelXeonProcessor,
    /// Dual-Core Intel® Xeon® processor 5200 Series
    DualCoreIntelXeonProcessor5200Series,
    /// Dual-Core Intel® Xeon® processor 7200 Series
    DualCoreIntelXeonProcessor7200Series,
    /// Quad-Core Intel® Xeon® processor 7300 Series
    QuadCoreIntelXeonProcessor7300Series,
    /// Quad-Core Intel® Xeon® processor 7400 Series
    QuadCoreIntelXeonProcessor7400Series,
    /// Multi-Core Intel® Xeon® processor 7400 Series
    MultiCoreIntelXeonProcessor7400Series,
    /// Pentium® III Xeon™ processor
    PentiumIIIXeonProcessor,
    /// Pentium® III Processor with Intel® SpeedStep™ Technology
    PentiumIIIProcessorwithIntelSpeedStepTechnology,
    /// Pentium® 4 Processor
    Pentium4Processor,
    /// Intel® Xeon® processor
    IntelXeonProcessor,
    /// AS400 Family
    AS400Family,
    /// Intel® Xeon™ processor MP
    IntelXeonProcessorMP,
    /// AMD Athlon™ XP Processor Family
    AMDAthlonXPProcessorFamily,
    /// AMD Athlon™ MP Processor Family
    AMDAthlonMPProcessorFamily,
    /// Intel® Itanium® 2 processor
    IntelItanium2Processor,
    /// Intel® Pentium® M processor
    IntelPentiumMProcessor,
    /// Intel® Celeron® D processor
    IntelCeleronDProcessor,
    /// Intel® Pentium® D processor
    IntelPentiumDProcessor,
    /// Intel® Pentium® Processor Extreme Edition
    IntelPentiumProcessorExtremeEdition,
    /// Intel® Core™ Solo Processor
    IntelCoreSoloProcessor,
    /// Intel® Core™ 2 Duo Processor
    IntelCore2DuoProcessor,
    /// Intel® Core™ 2 Solo processor
    IntelCore2SoloProcessor,
    /// Intel® Core™ 2 Extreme processor
    IntelCore2ExtremeProcessor,
    /// Intel® Core™ 2 Quad processor
    IntelCore2QuadProcessor,
    /// Intel® Core™ 2 Extreme mobile processor
    IntelCore2ExtremeMobileProcessor,
    /// Intel® Core™ 2 Duo mobile processor
    IntelCore2DuoMobileProcessor,
    /// Intel® Core™ 2 Solo mobile processor
    IntelCore2SoloMobileProcessor,
    /// Intel® Core™ i7 processor
    IntelCorei7Processor,
    /// Dual-Core Intel® Celeron® processor
    DualCoreIntelCeleronProcessor,
    /// IBM390 Family
    IBM390Family,
    /// G4
    G4,
    /// G5
    G5,
    /// ESA/390 G6
    ESA390G6,
    /// z/Architecture base
    ZArchitecturebase,
    /// Intel® Core™ i5 processor
    IntelCorei5processor,
    /// Intel® Core™ i3 processor
    IntelCorei3processor,
    /// Intel® Core™ i9 processor
    IntelCorei9processor,
    /// VIA C7™-M Processor Family
    VIAC7MProcessorFamily,
    /// VIA C7™-D Processor Family
    VIAC7DProcessorFamily,
    /// VIA C7™ Processor Family
    VIAC7ProcessorFamily,
    /// VIA Eden™ Processor Family
    VIAEdenProcessorFamily,
    /// Multi-Core Intel® Xeon® processor
    MultiCoreIntelXeonProcessor,
    /// Dual-Core Intel® Xeon® processor 3xxx Series
    DualCoreIntelXeonProcessor3xxxSeries,
    /// Quad-Core Intel® Xeon® processor 3xxx Series
    QuadCoreIntelXeonProcessor3xxxSeries,
    /// VIA Nano™ Processor Family
    VIANanoProcessorFamily,
    /// Dual-Core Intel® Xeon® processor 5xxx Series
    DualCoreIntelXeonProcessor5xxxSeries,
    /// Quad-Core Intel® Xeon® processor 5xxx Series
    QuadCoreIntelXeonProcessor5xxxSeries,
    /// Dual-Core Intel® Xeon® processor 7xxx Series
    DualCoreIntelXeonProcessor7xxxSeries,
    /// Quad-Core Intel® Xeon® processor 7xxx Series
    QuadCoreIntelXeonProcessor7xxxSeries,
    /// Multi-Core Intel® Xeon® processor 7xxx Series
    MultiCoreIntelXeonProcessor7xxxSeries,
    /// Multi-Core Intel® Xeon® processor 3400 Series
    MultiCoreIntelXeonProcessor3400Series,
    /// AMD Opteron™ 3000 Series Processor
    AMDOpteron3000SeriesProcessor,
    /// AMD Sempron™ II Processor
    AMDSempronIIProcessor,
    /// Embedded AMD Opteron™ Quad-Core Processor Family
    EmbeddedAMDOpteronQuadCoreProcessorFamily,
    /// AMD Phenom™ Triple-Core Processor Family
    AMDPhenomTripleCoreProcessorFamily,
    /// AMD Turion™ Ultra Dual-Core Mobile Processor Family
    AMDTurionUltraDualCoreMobileProcessorFamily,
    /// AMD Turion™ Dual-Core Mobile Processor Family
    AMDTurionDualCoreMobileProcessorFamily,
    /// AMD Athlon™ Dual-Core Processor Family
    AMDAthlonDualCoreProcessorFamily,
    /// AMD Sempron™ SI Processor Family
    AMDSempronSIProcessorFamily,
    /// AMD Phenom™ II Processor Family
    AMDPhenomIIProcessorFamily,
    /// AMD Athlon™ II Processor Family
    AMDAthlonIIProcessorFamily,
    /// Six-Core AMD Opteron™ Processor Family
    SixCoreAMDOpteronProcessorFamily,
    /// AMD Sempron™ M Processor Family
    AMDSempronMProcessorFamily,
    /// i860
    I860,
    /// i960
    I960,
    /// Indicator to obtain the processor family from the 'processor_family_2' field
    SeeProcessorFamily2,
    /// ARMv7
    ARMv7,
    /// ARMv8
    ARMv8,
    /// SH-3
    SH3,
    /// SH-4
    SH4,
    /// ARM
    ARM,
    /// StrongARM
    StrongARM,
    /// 6x86
    Cyrix6x86,
    /// MediaGX
    MediaGX,
    /// MII
    MII,
    /// WinChip
    WinChip,
    /// DSP
    DSP,
    /// Video Processor
    VideoProcessor,
    /// RISC-V RV32
    RISCVRV32,
    /// RISC-V RV64
    RISCVRV64,
    /// RISC-V RV128
    RISCVRV128,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u16> for ProcessorFamily {
    fn from(raw: u16) -> Self {
        match raw {
            0x01 => ProcessorFamily::Other,
            0x02 => ProcessorFamily::Unknown,
            0x03 => ProcessorFamily::I8086,
            0x04 => ProcessorFamily::I80286,
            0x05 => ProcessorFamily::Intel386Processor,
            0x06 => ProcessorFamily::Intel486Processor,
            0x07 => ProcessorFamily::I8087,
            0x08 => ProcessorFamily::I80287,
            0x09 => ProcessorFamily::I80387,
            0x0A => ProcessorFamily::I80487,
            0x0B => ProcessorFamily::IntelPentiumProcessor,
            0x0C => ProcessorFamily::PentiumProProcessor,
            0x0D => ProcessorFamily::PentiumIIProcessor,
            0x0E => ProcessorFamily::PentiumprocessorwithMMXtechnology,
            0x0F => ProcessorFamily::IntelCeleronProcessor,
            0x10 => ProcessorFamily::PentiumIIXeonProcessor,
            0x11 => ProcessorFamily::PentiumIIIProcessor,
            0x12 => ProcessorFamily::M1Family,
            0x13 => ProcessorFamily::M2Family,
            0x14 => ProcessorFamily::IntelCeleronMProcessor,
            0x15 => ProcessorFamily::IntelPentium4HTProcessor,
            0x18 => ProcessorFamily::AMDDuronProcessorFamily,
            0x19 => ProcessorFamily::K5Family,
            0x1A => ProcessorFamily::K6Family,
            0x1B => ProcessorFamily::K62,
            0x1C => ProcessorFamily::K63,
            0x1D => ProcessorFamily::AMDAthlonProcessorFamily,
            0x1E => ProcessorFamily::AMD29000Family,
            0x1F => ProcessorFamily::K62Plus,
            0x20 => ProcessorFamily::PowerPCFamily,
            0x21 => ProcessorFamily::PowerPC601,
            0x22 => ProcessorFamily::PowerPC603,
            0x23 => ProcessorFamily::PowerPC603Plus,
            0x24 => ProcessorFamily::PowerPC604,
            0x25 => ProcessorFamily::PowerPC620,
            0x26 => ProcessorFamily::PowerPCx704,
            0x27 => ProcessorFamily::PowerPC750,
            0x28 => ProcessorFamily::IntelCoreDuoProcessor,
            0x29 => ProcessorFamily::IntelCoreDuomobileProcessor,
            0x2A => ProcessorFamily::IntelCoreSolomobileProcessor,
            0x2B => ProcessorFamily::IntelAtomProcessor,
            0x2C => ProcessorFamily::IntelCoreMProcessor,
            0x2D => ProcessorFamily::IntelCorem3Processor,
            0x2E => ProcessorFamily::IntelCorem5Processor,
            0x2F => ProcessorFamily::IntelCorem7Processor,
            0x30 => ProcessorFamily::AlphaFamily,
            0x31 => ProcessorFamily::Alpha21064,
            0x32 => ProcessorFamily::Alpha21066,
            0x33 => ProcessorFamily::Alpha21164,
            0x34 => ProcessorFamily::Alpha21164PC,
            0x35 => ProcessorFamily::Alpha21164a,
            0x36 => ProcessorFamily::Alpha21264,
            0x37 => ProcessorFamily::Alpha21364,
            0x38 => ProcessorFamily::AMDTurionIIUltraDualCoreMobileMProcessorFamily,
            0x39 => ProcessorFamily::AMDTurionIIDualCoreMobileMProcessorFamily,
            0x3A => ProcessorFamily::AMDAthlonIIDualCoreMProcessorFamily,
            0x3B => ProcessorFamily::AMDOpteron6100SeriesProcessor,
            0x3C => ProcessorFamily::AMDOpteron4100SeriesProcessor,
            0x3D => ProcessorFamily::AMDOpteron6200SeriesProcessor,
            0x3E => ProcessorFamily::AMDOpteron4200SeriesProcessor,
            0x3F => ProcessorFamily::AMDFXSeriesProcessor,
            0x40 => ProcessorFamily::MIPSFamily,
            0x41 => ProcessorFamily::MIPSR4000,
            0x42 => ProcessorFamily::MIPSR4200,
            0x43 => ProcessorFamily::MIPSR4400,
            0x44 => ProcessorFamily::MIPSR4600,
            0x45 => ProcessorFamily::MIPSR10000,
            0x46 => ProcessorFamily::AMDCSeriesProcessor,
            0x47 => ProcessorFamily::AMDESeriesProcessor,
            0x48 => ProcessorFamily::AMDASeriesProcessor,
            0x49 => ProcessorFamily::AMDGSeriesProcessor,
            0x4A => ProcessorFamily::AMDZSeriesProcessor,
            0x4B => ProcessorFamily::AMDRSeriesProcessor,
            0x4C => ProcessorFamily::AMDOpteron4300SeriesProcessor,
            0x4D => ProcessorFamily::AMDOpteron6300SeriesProcessor,
            0x4E => ProcessorFamily::AMDOpteron3300SeriesProcessor,
            0x4F => ProcessorFamily::AMDFireProSeriesProcessor,
            0x50 => ProcessorFamily::SPARCFamily,
            0x51 => ProcessorFamily::SuperSPARC,
            0x52 => ProcessorFamily::MicroSparcii,
            0x53 => ProcessorFamily::MicroSparciiep,
            0x54 => ProcessorFamily::UltraSPARC,
            0x55 => ProcessorFamily::UltraSPARCII,
            0x56 => ProcessorFamily::UltraSPARCIii,
            0x57 => ProcessorFamily::UltraSPARCIII,
            0x58 => ProcessorFamily::UltraSPARCIIIi,
            0x60 => ProcessorFamily::M68040Family,
            0x61 => ProcessorFamily::M68xxx,
            0x62 => ProcessorFamily::M68000,
            0x63 => ProcessorFamily::M68010,
            0x64 => ProcessorFamily::M68020,
            0x65 => ProcessorFamily::M68030,
            0x66 => ProcessorFamily::AMDAthlonX4QuadCoreProcessorFamily,
            0x67 => ProcessorFamily::AMDOpteronX1000SeriesProcessor,
            0x68 => ProcessorFamily::AMDOpteronX2000SeriesAPU,
            0x69 => ProcessorFamily::AMDOpteronASeriesProcessor,
            0x6A => ProcessorFamily::AMDOpteronX3000SeriesAPU,
            0x6B => ProcessorFamily::AMDZenProcessorFamily,
            0x70 => ProcessorFamily::HobbitFamily,
            0x78 => ProcessorFamily::CrusoeTM5000Family,
            0x79 => ProcessorFamily::CrusoeTM3000Family,
            0x7A => ProcessorFamily::EfficeonTM8000Family,
            0x80 => ProcessorFamily::Weitek,
            0x82 => ProcessorFamily::Itaniumprocessor,
            0x83 => ProcessorFamily::AMDAthlon64ProcessorFamily,
            0x84 => ProcessorFamily::AMDOpteronProcessorFamily,
            0x85 => ProcessorFamily::AMDSempronProcessorFamily,
            0x86 => ProcessorFamily::AMDTurion64MobileTechnology,
            0x87 => ProcessorFamily::DualCoreAMDOpteronProcessorFamily,
            0x88 => ProcessorFamily::AMDAthlon64X2DualCoreProcessorFamily,
            0x89 => ProcessorFamily::AMDTurion64X2MobileTechnology,
            0x8A => ProcessorFamily::QuadCoreAMDOpteronProcessorFamily,
            0x8B => ProcessorFamily::ThirdGenerationAMDOpteronProcessorFamily,
            0x8C => ProcessorFamily::AMDPhenomFXQuadCoreProcessorFamily,
            0x8D => ProcessorFamily::AMDPhenomX4QuadCoreProcessorFamily,
            0x8E => ProcessorFamily::AMDPhenomX2DualCoreProcessorFamily,
            0x8F => ProcessorFamily::AMDAthlonX2DualCoreProcessorFamily,
            0x90 => ProcessorFamily::PARISCFamily,
            0x91 => ProcessorFamily::PARISC8500,
            0x92 => ProcessorFamily::PARISC8000,
            0x93 => ProcessorFamily::PARISC7300LC,
            0x94 => ProcessorFamily::PARISC7200,
            0x95 => ProcessorFamily::PARISC7100LC,
            0x96 => ProcessorFamily::PARISC7100,
            0xA0 => ProcessorFamily::V30Family,
            0xA1 => ProcessorFamily::QuadCoreIntelXeonProcessor3200Series,
            0xA2 => ProcessorFamily::DualCoreIntelXeonProcessor3000Series,
            0xA3 => ProcessorFamily::QuadCoreIntelXeonProcessor5300Series,
            0xA4 => ProcessorFamily::DualCoreIntelXeonProcessor5100Series,
            0xA5 => ProcessorFamily::DualCoreIntelXeonProcessor5000Series,
            0xA6 => ProcessorFamily::DualCoreIntelXeonProcessorLV,
            0xA7 => ProcessorFamily::DualCoreIntelXeonProcessorULV,
            0xA8 => ProcessorFamily::DualCoreIntelXeonProcessor7100Series,
            0xA9 => ProcessorFamily::QuadCoreIntelXeonProcessor5400Series,
            0xAA => ProcessorFamily::QuadCoreIntelXeonProcessor,
            0xAB => ProcessorFamily::DualCoreIntelXeonProcessor5200Series,
            0xAC => ProcessorFamily::DualCoreIntelXeonProcessor7200Series,
            0xAD => ProcessorFamily::QuadCoreIntelXeonProcessor7300Series,
            0xAE => ProcessorFamily::QuadCoreIntelXeonProcessor7400Series,
            0xAF => ProcessorFamily::MultiCoreIntelXeonProcessor7400Series,
            0xB0 => ProcessorFamily::PentiumIIIXeonProcessor,
            0xB1 => ProcessorFamily::PentiumIIIProcessorwithIntelSpeedStepTechnology,
            0xB2 => ProcessorFamily::Pentium4Processor,
            0xB3 => ProcessorFamily::IntelXeonProcessor,
            0xB4 => ProcessorFamily::AS400Family,
            0xB5 => ProcessorFamily::IntelXeonProcessorMP,
            0xB6 => ProcessorFamily::AMDAthlonXPProcessorFamily,
            0xB7 => ProcessorFamily::AMDAthlonMPProcessorFamily,
            0xB8 => ProcessorFamily::IntelItanium2Processor,
            0xB9 => ProcessorFamily::IntelPentiumMProcessor,
            0xBA => ProcessorFamily::IntelCeleronDProcessor,
            0xBB => ProcessorFamily::IntelPentiumDProcessor,
            0xBC => ProcessorFamily::IntelPentiumProcessorExtremeEdition,
            0xBD => ProcessorFamily::IntelCoreSoloProcessor,
            0xBF => ProcessorFamily::IntelCore2DuoProcessor,
            0xC0 => ProcessorFamily::IntelCore2SoloProcessor,
            0xC1 => ProcessorFamily::IntelCore2ExtremeProcessor,
            0xC2 => ProcessorFamily::IntelCore2QuadProcessor,
            0xC3 => ProcessorFamily::IntelCore2ExtremeMobileProcessor,
            0xC4 => ProcessorFamily::IntelCore2DuoMobileProcessor,
            0xC5 => ProcessorFamily::IntelCore2SoloMobileProcessor,
            0xC6 => ProcessorFamily::IntelCorei7Processor,
            0xC7 => ProcessorFamily::DualCoreIntelCeleronProcessor,
            0xC8 => ProcessorFamily::IBM390Family,
            0xC9 => ProcessorFamily::G4,
            0xCA => ProcessorFamily::G5,
            0xCB => ProcessorFamily::ESA390G6,
            0xCC => ProcessorFamily::ZArchitecturebase,
            0xCD => ProcessorFamily::IntelCorei5processor,
            0xCE => ProcessorFamily::IntelCorei3processor,
            0xCF => ProcessorFamily::IntelCorei9processor,
            0xD2 => ProcessorFamily::VIAC7MProcessorFamily,
            0xD3 => ProcessorFamily::VIAC7DProcessorFamily,
            0xD4 => ProcessorFamily::VIAC7ProcessorFamily,
            0xD5 => ProcessorFamily::VIAEdenProcessorFamily,
            0xD6 => ProcessorFamily::MultiCoreIntelXeonProcessor,
            0xD7 => ProcessorFamily::DualCoreIntelXeonProcessor3xxxSeries,
            0xD8 => ProcessorFamily::QuadCoreIntelXeonProcessor3xxxSeries,
            0xD9 => ProcessorFamily::VIANanoProcessorFamily,
            0xDA => ProcessorFamily::DualCoreIntelXeonProcessor5xxxSeries,
            0xDB => ProcessorFamily::QuadCoreIntelXeonProcessor5xxxSeries,
            0xDD => ProcessorFamily::DualCoreIntelXeonProcessor7xxxSeries,
            0xDE => ProcessorFamily::QuadCoreIntelXeonProcessor7xxxSeries,
            0xDF => ProcessorFamily::MultiCoreIntelXeonProcessor7xxxSeries,
            0xE0 => ProcessorFamily::MultiCoreIntelXeonProcessor3400Series,
            0xE4 => ProcessorFamily::AMDOpteron3000SeriesProcessor,
            0xE5 => ProcessorFamily::AMDSempronIIProcessor,
            0xE6 => ProcessorFamily::EmbeddedAMDOpteronQuadCoreProcessorFamily,
            0xE7 => ProcessorFamily::AMDPhenomTripleCoreProcessorFamily,
            0xE8 => ProcessorFamily::AMDTurionUltraDualCoreMobileProcessorFamily,
            0xE9 => ProcessorFamily::AMDTurionDualCoreMobileProcessorFamily,
            0xEA => ProcessorFamily::AMDAthlonDualCoreProcessorFamily,
            0xEB => ProcessorFamily::AMDSempronSIProcessorFamily,
            0xEC => ProcessorFamily::AMDPhenomIIProcessorFamily,
            0xED => ProcessorFamily::AMDAthlonIIProcessorFamily,
            0xEE => ProcessorFamily::SixCoreAMDOpteronProcessorFamily,
            0xEF => ProcessorFamily::AMDSempronMProcessorFamily,
            0xFA => ProcessorFamily::I860,
            0xFB => ProcessorFamily::I960,
            0xFE => ProcessorFamily::SeeProcessorFamily2,
            0x100 => ProcessorFamily::ARMv7,
            0x101 => ProcessorFamily::ARMv8,
            0x104 => ProcessorFamily::SH3,
            0x105 => ProcessorFamily::SH4,
            0x118 => ProcessorFamily::ARM,
            0x119 => ProcessorFamily::StrongARM,
            0x12C => ProcessorFamily::Cyrix6x86,
            0x12D => ProcessorFamily::MediaGX,
            0x12E => ProcessorFamily::MII,
            0x140 => ProcessorFamily::WinChip,
            0x15E => ProcessorFamily::DSP,
            0x1F4 => ProcessorFamily::VideoProcessor,
            0x200 => ProcessorFamily::RISCVRV32,
            0x201 => ProcessorFamily::RISCVRV64,
            0x202 => ProcessorFamily::RISCVRV128,
            _ => ProcessorFamily::None,
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorUpgradeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ProcessorUpgradeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorUpgradeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for ProcessorUpgradeData {
    type Target = ProcessorUpgrade;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// #
#[derive(Serialize, Debug, PartialEq, Eq)]
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl Serialize for ProcessorCharacteristics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorCharacteristics", 10)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("bit_64capable", &self.bit_64capable())?;
        state.serialize_field("multi_core", &self.multi_core())?;
        state.serialize_field("hardware_thread", &self.hardware_thread())?;
        state.serialize_field("execute_protection", &self.execute_protection())?;
        state.serialize_field("enhanced_virtualization", &self.enhanced_virtualization())?;
        state.serialize_field(
            "power_performance_control",
            &self.power_performance_control(),
        )?;
        state.serialize_field("bit_128capable", &self.bit_128capable())?;
        state.serialize_field("arm_64soc_id", &self.arm_64soc_id())?;
        state.end()
    }
}

/// # Processor Voltage
#[derive(Serialize, Debug)]
pub enum ProcessorVoltage {
    /// Current Processor Voltage
    CurrentVolts(f32),
    /// Processor Supported Voltages
    SupportedVolts(ProcessorSupportedVoltages),
}

impl From<u8> for ProcessorVoltage {
    fn from(raw: u8) -> Self {
        if raw & 0b1000_0000 == 0b1000_0000 {
            ProcessorVoltage::CurrentVolts((raw & 0b0111_1111) as f32 / 10.0)
        } else {
            ProcessorVoltage::SupportedVolts(ProcessorSupportedVoltages::from(raw))
        }
    }
}

/// # Processor Supported Voltages
#[derive(PartialEq, Eq)]
pub struct ProcessorSupportedVoltages {
    /// Raw value
    pub raw: u8,
}

impl Deref for ProcessorSupportedVoltages {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for ProcessorSupportedVoltages {
    fn from(raw: u8) -> Self {
        debug_assert_eq!(raw, raw & 0b0111_1111);
        ProcessorSupportedVoltages { raw }
    }
}

impl ProcessorSupportedVoltages {
    /// Bit 0 – 5V
    pub fn volts_5_0(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// Bit 1 – 3.3V
    pub fn volts_3_3(&self) -> bool {
        self.raw & 0x02 == 0x02
    }

    /// Bit 2 – 2.9V
    pub fn volts_2_9(&self) -> bool {
        self.raw & 0x04 == 0x04
    }

    /// Available Voltages
    pub fn voltages(&self) -> Vec<f32> {
        let mut result = Vec::new();

        if self.volts_2_9() {
            result.push(2.9);
        }

        if self.volts_3_3() {
            result.push(3.3);
        }

        if self.volts_5_0() {
            result.push(5.0);
        }

        result
    }
}

impl fmt::Debug for ProcessorSupportedVoltages {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorSupportedVoltages>())
            .field("raw", &self.raw)
            .field("voltages", &self.voltages().as_slice())
            .finish()
    }
}

impl Serialize for ProcessorSupportedVoltages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorSupportedVoltages", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("voltages", &self.voltages().as_slice())?;
        state.end()
    }
}

/// External Clock Frequency in MHz
#[derive(Serialize)]
pub enum ProcessorExternalClock {
    /// The value is unknown
    Unknown,
    /// External Clock Frequency in MHz
    MHz(u16),
}

impl From<u16> for ProcessorExternalClock {
    fn from(raw: u16) -> Self {
        match raw {
            0 => ProcessorExternalClock::Unknown,
            _ => ProcessorExternalClock::MHz(raw),
        }
    }
}

impl fmt::Debug for ProcessorExternalClock {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ProcessorExternalClock::*;
        match self {
            Unknown => write!{fmt, "Unknown"},
            MHz(n) => write!{fmt, "{} MHz", n}
        }
    }
}

/// Processor Speed in MHz
#[derive(Serialize)]
pub enum ProcessorSpeed {
    /// The value is unknown
    Unknown,
    /// The Processor Speed in MHz
    MHz(u16),
}

impl From<u16> for ProcessorSpeed {
    fn from(raw: u16) -> Self {
        match raw {
            0 => ProcessorSpeed::Unknown,
            _ => ProcessorSpeed::MHz(raw),
        }
    }
}

impl fmt::Debug for ProcessorSpeed {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ProcessorSpeed::*;
        match self {
            Unknown => write!{fmt, "Unknown"},
            MHz(n) => write!{fmt, "{} MHz", n}
        }
    }
}

/// # Processor Socket and CPU Status
#[derive(PartialEq, Eq)]
pub struct ProcessorStatus {
    /// Raw value
    pub raw: u8,
}

impl Deref for ProcessorStatus {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for ProcessorStatus {
    fn from(raw: u8) -> Self {
        ProcessorStatus { raw }
    }
}

impl ProcessorStatus {
    /// CPU Socket Populated
    pub fn socket_populated(&self) -> bool {
        self.raw & 0b0100_0000 == 0b0100_0000
    }

    /// CPU Status
    pub fn cpu_status(&self) -> CpuStatus {
        CpuStatus::from(self.raw)
    }
}

impl fmt::Debug for ProcessorStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProcessorStatus>())
            .field("raw", &self.raw)
            .field("socket_populated", &self.socket_populated())
            .field("cpu_status", &self.cpu_status())
            .finish()
    }
}

impl Serialize for ProcessorStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProcessorStatus", 3)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("socket_populated", &self.socket_populated())?;
        state.serialize_field("cpu_status", &self.cpu_status())?;
        state.end()
    }
}

/// CPU Status
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum CpuStatus {
    /// 0h – Unknown
    Unknown,
    /// 1h – CPU Enabled
    Enabled,
    /// 2h – CPU Disabled by User through BIOS Setup
    UserDisabled,
    /// 3h – CPU Disabled By BIOS (POST Error)
    BiosDisabled,
    /// 4h – CPU is Idle, waiting to be enabled.
    Idle,
    /// 7h – Other
    Other,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for CpuStatus {
    fn from(raw: u8) -> Self {
        match raw & 0b0000_0111 {
            0 => CpuStatus::Unknown,
            1 => CpuStatus::Enabled,
            2 => CpuStatus::UserDisabled,
            3 => CpuStatus::BiosDisabled,
            4 => CpuStatus::Idle,
            7 => CpuStatus::Other,
            _ => CpuStatus::None,
        }
    }
}

/// Processor Core Count
#[derive(Serialize, Debug)]
pub enum CoreCount {
    /// The value is unknown
    Unknown,
    /// Number of cores per processor socket
    Count(u8),
    /// For core counts of 256 or greater the 'core_count_2' field
    /// is set to the number of cores.
    SeeCoreCount2,
}

impl From<u8> for CoreCount {
    fn from(raw: u8) -> Self {
        match raw {
            0 => CoreCount::Unknown,
            0xFF => CoreCount::SeeCoreCount2,
            _ => CoreCount::Count(raw),
        }
    }
}

/// Processor Core Count #2
#[derive(Serialize, Debug)]
pub enum CoreCount2 {
    /// The value is unknown
    Unknown,
    /// Number of cores per processor socket
    Count(u16),
    /// Reserved (0xFFFF)
    Reserved,
}

impl From<u16> for CoreCount2 {
    fn from(raw: u16) -> Self {
        match raw {
            0 => CoreCount2::Unknown,
            0xFFFF => CoreCount2::Reserved,
            _ => CoreCount2::Count(raw),
        }
    }
}

/// Processor Cores Enabled
#[derive(Serialize, Debug)]
pub enum CoresEnabled {
    /// The value is unknown
    Unknown,
    /// Number of cores enabled
    Count(u8),
    /// For core counts of 256 or greater the 'cores_enabled_2' field
    /// is set to the number of enabled cores.
    SeeCoresEnabled2,
}

impl From<u8> for CoresEnabled {
    fn from(raw: u8) -> Self {
        match raw {
            0 => CoresEnabled::Unknown,
            0xFF => CoresEnabled::SeeCoresEnabled2,
            _ => CoresEnabled::Count(raw),
        }
    }
}

/// Processor Cores Enabled #2
#[derive(Serialize, Debug)]
pub enum CoresEnabled2 {
    /// The value is unknown
    Unknown,
    /// Number of cores enabled
    Count(u16),
    /// Reserved (0xFFFF)
    Reserved,
}

impl From<u16> for CoresEnabled2 {
    fn from(raw: u16) -> Self {
        match raw {
            0 => CoresEnabled2::Unknown,
            0xFFFF => CoresEnabled2::Reserved,
            _ => CoresEnabled2::Count(raw),
        }
    }
}

/// Processor Thread Count
#[derive(Serialize, Debug)]
pub enum ThreadCount {
    /// The value is unknown
    Unknown,
    /// Number of threads per processor socket
    Count(u8),
    /// For thread counts of 256 or greater the 'thread_count_2' field
    /// is set to the number of cores.
    SeeThreadCount2,
}

impl From<u8> for ThreadCount {
    fn from(raw: u8) -> Self {
        match raw {
            0 => ThreadCount::Unknown,
            0xFF => ThreadCount::SeeThreadCount2,
            _ => ThreadCount::Count(raw),
        }
    }
}

/// Processor Thread Count #2
#[derive(Serialize, Debug)]
pub enum ThreadCount2 {
    /// The value is unknown
    Unknown,
    /// Number of threads per processor socket
    Count(u16),
    /// Reserved (0xFFFF)
    Reserved,
}

impl From<u16> for ThreadCount2 {
    fn from(raw: u16) -> Self {
        match raw {
            0 => ThreadCount2::Unknown,
            0xFFFF => ThreadCount2::Reserved,
            _ => ThreadCount2::Count(raw),
        }
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

        let parts = UndefinedStruct::new(&struct_type4);
        let test_struct = SMBiosProcessorInformation::new(&parts);

        assert_eq!(test_struct.socket_designation(), Some("CPU0".to_string()));
        assert_eq!(
            *test_struct.processor_type().unwrap(),
            ProcessorType::CentralProcessor
        );
        assert_eq!(
            *test_struct.processor_family().unwrap(),
            ProcessorFamily::IntelXeonProcessor
        );
        assert_eq!(
            test_struct.processor_manufacturer(),
            Some("Intel(R) Corporation".to_string())
        );
        assert_eq!(
            test_struct.processor_id(),
            Some(&[0x54u8, 0x06, 0x05, 0x00, 0xFF, 0xFB, 0xEB, 0xBF])
        );
        assert_eq!(
            test_struct.processor_version(),
            Some("Intel(R) Xeon(R) W-2133 CPU @ 3.60GHz".to_string())
        );
        match test_struct.voltage().unwrap() {
            ProcessorVoltage::CurrentVolts(volts) => assert_eq!(volts, 1.6),
            ProcessorVoltage::SupportedVolts(_) => panic!("expected current volts"),
        }
        match test_struct.external_clock().unwrap() {
            ProcessorExternalClock::MHz(mhz) => assert_eq!(mhz, 100),
            ProcessorExternalClock::Unknown => panic!("expected MHz"),
        }
        match test_struct.max_speed().unwrap() {
            ProcessorSpeed::MHz(mhz) => assert_eq!(mhz, 3900),
            ProcessorSpeed::Unknown => panic!("expected MHz"),
        }
        match test_struct.current_speed().unwrap() {
            ProcessorSpeed::MHz(mhz) => assert_eq!(mhz, 3600),
            ProcessorSpeed::Unknown => panic!("expected MHz"),
        }
        let processor_status = test_struct.status().unwrap();
        assert!(processor_status.socket_populated());
        assert_eq!(processor_status.cpu_status(), CpuStatus::Enabled);
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
        match test_struct.core_count().unwrap() {
            CoreCount::Count(number) => assert_eq!(number, 6),
            CoreCount::Unknown => panic!("expected number"),
            CoreCount::SeeCoreCount2 => panic!("expected number"),
        }
        match test_struct.cores_enabled().unwrap() {
            CoresEnabled::Count(number) => assert_eq!(number, 6),
            CoresEnabled::Unknown => panic!("expected number"),
            CoresEnabled::SeeCoresEnabled2 => panic!("expected number"),
        }
        match test_struct.thread_count().unwrap() {
            ThreadCount::Count(number) => assert_eq!(number, 12),
            ThreadCount::Unknown => panic!("expected number"),
            ThreadCount::SeeThreadCount2 => panic!("expected number"),
        }
        assert_eq!(
            test_struct.processor_characteristics(),
            Some(ProcessorCharacteristics::from(252))
        );
        assert_eq!(
            *test_struct.processor_family_2().unwrap(),
            ProcessorFamily::IntelXeonProcessor
        );

        match test_struct.core_count_2().unwrap() {
            CoreCount2::Count(number) => assert_eq!(number, 6),
            CoreCount2::Unknown => panic!("expected number"),
            CoreCount2::Reserved => panic!("expected number"),
        }
        match test_struct.cores_enabled_2().unwrap() {
            CoresEnabled2::Count(number) => assert_eq!(number, 6),
            CoresEnabled2::Unknown => panic!("expected number"),
            CoresEnabled2::Reserved => panic!("expected number"),
        }
        match test_struct.thread_count_2().unwrap() {
            ThreadCount2::Count(number) => assert_eq!(number, 12),
            ThreadCount2::Unknown => panic!("expected number"),
            ThreadCount2::Reserved => panic!("expected number"),
        }
    }
}
