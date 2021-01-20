use super::*;
use std::ops::Deref;

/// # System Slots (Type 9)
///
/// The information in this structure defines the attributes of a system slot. One
/// structure is provided for each slot in the system.
pub struct SMBiosSystemSlot<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemSlot<'a> {
    const STRUCT_TYPE: u8 = 9u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemSlot<'a> {
    /// Slot Designation
    pub fn slot_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Slot Type
    pub fn system_slot_type(&self) -> Option<SystemSlotTypeData> {
        self.parts
            .get_field_byte(0x05)
            .and_then(|raw| Some(SystemSlotTypeData::from(raw)))
    }

    /// Slot Data Bus Width
    pub fn slot_data_bus_width(&self) -> Option<SlotWidthData> {
        self.parts
            .get_field_byte(0x06)
            .and_then(|raw| Some(SlotWidthData::from(raw)))
    }

    /// Current Usage
    pub fn current_usage(&self) -> Option<SlotCurrentUsageData> {
        self.parts
            .get_field_byte(0x07)
            .and_then(|raw| Some(SlotCurrentUsageData::from(raw)))
    }

    /// Slot Length
    pub fn slot_length(&self) -> Option<SlotLengthData> {
        self.parts
            .get_field_byte(0x08)
            .and_then(|raw| Some(SlotLengthData::from(raw)))
    }

    /// Slot Id
    pub fn slot_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x09)
    }

    /// Slot Characteristics 1
    pub fn slot_characteristics_1(&self) -> Option<SystemSlotCharacteristics1> {
        self.parts
            .get_field_byte(0x0B)
            .and_then(|raw| Some(SystemSlotCharacteristics1::from(raw)))
    }

    /// Slot Characteristics 2
    pub fn slot_characteristics_2(&self) -> Option<SystemSlotCharacteristics2> {
        self.parts
            .get_field_byte(0x0C)
            .and_then(|raw| Some(SystemSlotCharacteristics2::from(raw)))
    }

    /// Segment Group Number (Base)
    pub fn segment_group_number(&self) -> Option<u16> {
        self.parts.get_field_word(0x0D)
    }

    /// Bus Number (Base)
    pub fn bus_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    /// Device/Function Number (Base)
    pub fn device_function_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0x10)
    }

    /// Data Bus Width (Base)
    pub fn data_bus_width(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    /// Number of peer Segment/Bus/Device/Function/Width groups that follow
    pub fn peer_group_count(&self) -> Option<usize> {
        self.parts
            .get_field_byte(0x12)
            .and_then(|count| Some(count as usize))
    }

    /// 5*n
    fn peer_group_size(&self) -> Option<usize> {
        self.peer_group_count()
            .and_then(|count| Some(count as usize * SlotPeerGroup::SIZE))
    }

    /// Iterates over the [SlotPeerGroup] entries
    pub fn peer_group_iterator(&'a self) -> SlotPeerGroupIterator<'a> {
        SlotPeerGroupIterator::new(self)
    }

    /// Slot Information
    pub fn slot_information(&self) -> Option<u8> {
        self.peer_group_size()
            .and_then(|size| self.parts.get_field_byte(size + 0x13)) // TODO: watch for an errata, 0x13 is 0x14 in the 3.4.0 spec.
    }

    /// Slot Physical Width
    ///
    /// This field indicates the physical width of the slot whereas _slot_data_bus_width()_ indicates the
    /// electrical width of the slot.
    ///
    /// The possible values of both fields are listed in Table 46 – System Slots: Slot Width field.
    pub fn slot_physical_width(&self) -> Option<SlotWidthData> {
        self.peer_group_size().and_then(|size| {
            self.parts
                .get_field_byte(size + 0x14)
                .and_then(|raw| Some(SlotWidthData::from(raw)))
        })
    }

    /// Slot Pitch
    ///
    /// The Slot Pitch field contains a numeric value that indicates the pitch of the slot in units of 1/100 millimeter.
    ///
    /// The pitch is defined by each slot/card specification, but typically describes add-in card to add-in card
    /// pitch.
    ///
    /// For EDSFF slots, the pitch is defined in SFF-TA-1006 table 7.1, SFF-TA-1007 table 7.1 (add-in card to
    /// add-in card pitch), and SFF-TA-1008 table 6-1 (SSD to SSD pitch).
    ///
    /// For example, if the pitch for the slot is 12.5 mm, the value 1250 would be used.
    ///
    /// A value of 0 implies that the slot pitch is not given or is unknown.
    pub fn slot_pitch(&self) -> Option<u16> {
        self.peer_group_size()
            .and_then(|size| self.parts.get_field_word(size + 0x15))
    }
}

impl fmt::Debug for SMBiosSystemSlot<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemSlot>())
            .field("header", &self.parts.header)
            .field("slot_designation", &self.slot_designation())
            .field("system_slot_type", &self.system_slot_type())
            .field("slot_data_bus_width", &self.slot_data_bus_width())
            .field("current_usage", &self.current_usage())
            .field("slot_length", &self.slot_length())
            .field("slot_id", &self.slot_id())
            .field("slot_characteristics_1", &self.slot_characteristics_1())
            .field("slot_characteristics_2", &self.slot_characteristics_2())
            .field("segment_group_number", &self.segment_group_number())
            .field("bus_number", &self.bus_number())
            .field("device_function_number", &self.device_function_number())
            .field("data_bus_width", &self.data_bus_width())
            .field("peer_group_count", &self.peer_group_count())
            .field("peer_group_iterator", &self.peer_group_iterator())
            .field("slot_information", &self.slot_information())
            .field("slot_physical_width", &self.slot_physical_width())
            .field("slot_pitch", &self.slot_pitch())
            .finish()
    }
}

/// # System Slot Type Data
pub struct SystemSlotTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    raw: u8,
    /// The contained [SystemSlotType] value
    value: SystemSlotType,
}

impl Deref for SystemSlotTypeData {
    type Target = SystemSlotType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for SystemSlotTypeData {
    /// System Slot Type
    fn from(raw: u8) -> Self {
        SystemSlotTypeData {
            value: match raw {
                0x01 => SystemSlotType::Other,
                0x02 => SystemSlotType::Unknown,
                0x03 => SystemSlotType::Isa,
                0x04 => SystemSlotType::Mca,
                0x05 => SystemSlotType::Eisa,
                0x06 => SystemSlotType::Pci,
                0x07 => SystemSlotType::Pcmcia,
                0x08 => SystemSlotType::VlVesa,
                0x09 => SystemSlotType::Proprietary,
                0x0A => SystemSlotType::ProcessorCardSlot,
                0x0B => SystemSlotType::ProprietaryMemoryCardSlot,
                0x0C => SystemSlotType::IORiserCardSlot,
                0x0D => SystemSlotType::NuBus,
                0x0E => SystemSlotType::Pci66MhzCapable,
                0x0F => SystemSlotType::Agp,
                0x10 => SystemSlotType::Agp2x,
                0x11 => SystemSlotType::Agp4x,
                0x12 => SystemSlotType::PciX,
                0x13 => SystemSlotType::Agp8X,
                0x14 => SystemSlotType::M2Socket1DP,
                0x15 => SystemSlotType::M2Socket1SD,
                0x16 => SystemSlotType::M2Socket2,
                0x17 => SystemSlotType::M2Socket3,
                0x18 => SystemSlotType::MxmTypeI,
                0x19 => SystemSlotType::MxmTypeII,
                0x1A => SystemSlotType::MxmTypeIIIStandard,
                0x1B => SystemSlotType::MxmTypeIIIHE,
                0x1C => SystemSlotType::MxmTypeIV,
                0x1D => SystemSlotType::Mxm3TypeA,
                0x1E => SystemSlotType::Mxm3TypeB,
                0x1F => SystemSlotType::PciExpressGen2Sff8639,
                0x20 => SystemSlotType::PciExpressGen3Sff8639,
                0x21 => SystemSlotType::PciExpressMini52WithKeepouts,
                0x22 => SystemSlotType::PciExpressMini52WithoutKeepouts,
                0x23 => SystemSlotType::PciExpressMini76,
                0x24 => SystemSlotType::PciExpressGen4Sff8639,
                0x25 => SystemSlotType::PciExpressGen5Sff8639,
                0x30 => SystemSlotType::CxlFlexbus1,
                0xA0 => SystemSlotType::PC98C20,
                0xA1 => SystemSlotType::PC98C24,
                0xA2 => SystemSlotType::PC98E,
                0xA3 => SystemSlotType::PC98LocalBus,
                0xA4 => SystemSlotType::PC98Card,
                0xA5 => SystemSlotType::PciExpress,
                0xA6 => SystemSlotType::PciExpressx1,
                0xA7 => SystemSlotType::PciExpressx2,
                0xA8 => SystemSlotType::PciExpressx4,
                0xA9 => SystemSlotType::PciExpressx8,
                0xAA => SystemSlotType::PciExpressx16,
                0xAB => SystemSlotType::PciExpressGen2,
                0xAC => SystemSlotType::PciExpressGen2x1,
                0xAD => SystemSlotType::PciExpressGen2x2,
                0xAE => SystemSlotType::PciExpressGen2x4,
                0xAF => SystemSlotType::PciExpressGen2x8,
                0xB0 => SystemSlotType::PciExpressGen2x16,
                0xB1 => SystemSlotType::PciExpressGen3,
                0xB2 => SystemSlotType::PciExpressGen3x1,
                0xB3 => SystemSlotType::PciExpressGen3x2,
                0xB4 => SystemSlotType::PciExpressGen3x4,
                0xB5 => SystemSlotType::PciExpressGen3x8,
                0xB6 => SystemSlotType::PciExpressGen3x16,
                0xB8 => SystemSlotType::PciExpressGen4,
                0xB9 => SystemSlotType::PciExpressGen4x1,
                0xBA => SystemSlotType::PciExpressGen4x2,
                0xBB => SystemSlotType::PciExpressGen4x4,
                0xBC => SystemSlotType::PciExpressGen4x8,
                0xBD => SystemSlotType::PciExpressGen4x16,
                0xBE => SystemSlotType::PciExpressGen5,
                0xBF => SystemSlotType::PciExpressGen5x1,
                0xC0 => SystemSlotType::PciExpressGen5x2,
                0xC1 => SystemSlotType::PciExpressGen5x4,
                0xC2 => SystemSlotType::PciExpressGen5x8,
                0xC3 => SystemSlotType::PciExpressGen5x16,
                0xC4 => SystemSlotType::PciExpressGen6,
                0xC5 => SystemSlotType::EnterpriseAndDataCenter1UE1,
                0xC6 => SystemSlotType::EnterpriseAndDataCenter3InE3,
                _ => SystemSlotType::None,
            },
            raw,
        }
    }
}

impl fmt::Debug for SystemSlotTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SystemSlotTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

/// # System Slot Type
#[derive(Debug, PartialEq, Eq)]
pub enum SystemSlotType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// ISA
    Isa,
    /// MCA
    Mca,
    /// EISA
    Eisa,
    /// PCI
    Pci,
    /// PC Card (PCMCIA)
    Pcmcia,
    /// VL-VESA
    VlVesa,
    /// Proprietary
    Proprietary,
    /// Processor Card Slot
    ProcessorCardSlot,
    /// Proprietary Memory Card Slot
    ProprietaryMemoryCardSlot,
    /// I/O Riser Card Slot
    IORiserCardSlot,
    /// NuBus
    NuBus,
    /// PCI – 66MHz Capable
    Pci66MhzCapable,
    /// AGP
    Agp,
    /// AGP 2X
    Agp2x,
    /// AGP 4X
    Agp4x,
    /// PCI-X
    PciX,
    /// AGP 8X
    Agp8X,
    /// M.2 Socket 1-DP (Mechanical Key A)
    M2Socket1DP,
    /// M.2 Socket 1-SD (Mechanical Key E)
    M2Socket1SD,
    /// M.2 Socket 2 (Mechanical Key B)
    M2Socket2,
    /// M.2 Socket 3 (Mechanical Key M)
    M2Socket3,
    /// MXM Type I
    MxmTypeI,
    /// MXM Type II
    MxmTypeII,
    /// MXM Type III (standard connector)
    MxmTypeIIIStandard,
    /// MXM Type III (HE connector)
    MxmTypeIIIHE,
    /// MXM Type IV
    MxmTypeIV,
    /// MXM 3.0 Type A
    Mxm3TypeA,
    /// MXM 3.0 Type B
    Mxm3TypeB,
    /// PCI Express Gen 2 SFF-8639 (U.2)
    PciExpressGen2Sff8639,
    /// PCI Express Gen 3 SFF-8639 (U.2)
    PciExpressGen3Sff8639,
    /// PCI Express Mini 52-pin (CEM spec. 2.0) with bottom-side keep-outs. Use Slot Length field value 03h (short length) for "half-Mini card" -only support, 04h (long length) for "full-Mini card" or dual support.
    PciExpressMini52WithKeepouts,
    /// PCI Express Mini 52-pin (CEM spec. 2.0) without bottom-side keep-outs. Use Slot Length field value 03h (short length) for "half-Mini card" -only support, 04h (long length) for "full-Mini card" or dual support.
    PciExpressMini52WithoutKeepouts,
    /// PCI Express Mini 76-pin (CEM spec. 2.0) Corresponds to Display-Mini card.
    PciExpressMini76,
    /// PCI Express Gen 4 SFF-8639 (U.2)
    PciExpressGen4Sff8639,
    /// PCI Express Gen 5 SFF-8639 (U.2)
    PciExpressGen5Sff8639,
    /// CXL Flexbus 1.0 (deprecated, see note below)
    CxlFlexbus1,
    /// PC-98/C20
    PC98C20,
    /// PC-98/C24
    PC98C24,
    /// PC-98/E
    PC98E,
    /// PC-98/Local Bus
    PC98LocalBus,
    /// PC-98/Card
    PC98Card,
    /// PCI Express (see note below)
    PciExpress,
    /// PCI Express x1
    PciExpressx1,
    /// PCI Express x2
    PciExpressx2,
    /// PCI Express x4
    PciExpressx4,
    /// PCI Express x8
    PciExpressx8,
    /// PCI Express x16
    PciExpressx16,
    /// PCI Express Gen 2 (see note below)
    PciExpressGen2,
    /// PCI Express Gen 2 x1
    PciExpressGen2x1,
    /// PCI Express Gen 2 x2
    PciExpressGen2x2,
    /// PCI Express Gen 2 x4
    PciExpressGen2x4,
    /// PCI Express Gen 2 x8
    PciExpressGen2x8,
    /// PCI Express Gen 2 x16
    PciExpressGen2x16,
    /// PCI Express Gen 3 (see note below)
    PciExpressGen3,
    /// PCI Express Gen 3 x1
    PciExpressGen3x1,
    /// PCI Express Gen 3 x2
    PciExpressGen3x2,
    /// PCI Express Gen 3 x4
    PciExpressGen3x4,
    /// PCI Express Gen 3 x8
    PciExpressGen3x8,
    /// PCI Express Gen 3 x16
    PciExpressGen3x16,
    /// PCI Express Gen 4 (see note below)
    PciExpressGen4,
    /// PCI Express Gen 4 x1
    PciExpressGen4x1,
    /// PCI Express Gen 4 x2
    PciExpressGen4x2,
    /// PCI Express Gen 4 x4
    PciExpressGen4x4,
    /// PCI Express Gen 4 x8
    PciExpressGen4x8,
    /// PCI Express Gen 4 x16
    PciExpressGen4x16,
    /// PCI Express Gen 5 (see note below)
    PciExpressGen5,
    /// PCI Express Gen 5 x1
    PciExpressGen5x1,
    /// PCI Express Gen 5 x2
    PciExpressGen5x2,
    /// PCI Express Gen 5 x4
    PciExpressGen5x4,
    /// PCI Express Gen 5 x8
    PciExpressGen5x8,
    /// PCI Express Gen 5 x16
    PciExpressGen5x16,
    /// PCI Express Gen 6 and Beyond (see Slot Information and Slot Physical Width fields for more details)
    PciExpressGen6,
    /// Enterprise and Datacenter 1U E1 Form Factor Slot (EDSFF E1.S, E1.L) E1 slot length is reported in Slot Length field (see section 7.10.4). E1 slot pitch is reported in Slot Pitch field (see section 7.10.12). See specifications SFF-TA-1006 and SFF-TA-1007 for more details on values for slot length and pitch.
    EnterpriseAndDataCenter1UE1,
    /// Enterprise and Datacenter 3" E3 Form Factor Slot (EDSFF E3.S, E3.L) E3 slot length is reported in Slot Length field (see section 7.10.4). E3 slot pitch is reported in Slot Pitch field (see section 7.10.12). See specification SFF-TA-1008 for details on values for slot length and pitch.
    EnterpriseAndDataCenter3InE3,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Data Bus Width Data
pub struct SlotWidthData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [SlotWidth] value
    pub value: SlotWidth,
}

impl Deref for SlotWidthData {
    type Target = SlotWidth;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for SlotWidthData {
    fn from(raw: u8) -> Self {
        SlotWidthData {
            value: match raw {
                0x01 => SlotWidth::Other,
                0x02 => SlotWidth::Unknown,
                0x03 => SlotWidth::Bit8,
                0x04 => SlotWidth::Bit16,
                0x05 => SlotWidth::Bit32,
                0x06 => SlotWidth::Bit64,
                0x07 => SlotWidth::Bit128,
                0x08 => SlotWidth::X1,
                0x09 => SlotWidth::X2,
                0x0A => SlotWidth::X4,
                0x0B => SlotWidth::X8,
                0x0C => SlotWidth::X12,
                0x0D => SlotWidth::X16,
                0x0E => SlotWidth::X32,
                _ => SlotWidth::None,
            },
            raw,
        }
    }
}

impl fmt::Debug for SlotWidthData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotWidthData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

/// # Slot Width
#[derive(Debug, PartialEq, Eq)]
pub enum SlotWidth {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// 8 bit
    Bit8,
    /// 16 bit
    Bit16,
    /// 32 bit
    Bit32,
    /// 64 bit
    Bit64,
    /// 128 bit
    Bit128,
    /// 1x or x1
    X1,
    /// 2x or x2
    X2,
    /// 4x or x4
    X4,
    /// 8x or x8
    X8,
    /// 12x or x12
    X12,
    /// 16x or x16
    X16,
    /// 32x or x32
    X32,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # System Slot Current Usage Data
pub struct SlotCurrentUsageData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [SlotCurrentUsage] value
    pub value: SlotCurrentUsage,
}

impl Deref for SlotCurrentUsageData {
    type Target = SlotCurrentUsage;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for SlotCurrentUsageData {
    fn from(raw: u8) -> Self {
        SlotCurrentUsageData {
            value: match raw {
                0x01 => SlotCurrentUsage::Other,
                0x02 => SlotCurrentUsage::Unknown,
                0x03 => SlotCurrentUsage::Available,
                0x04 => SlotCurrentUsage::InUse,
                0x05 => SlotCurrentUsage::Unavailable,
                _ => SlotCurrentUsage::None,
            },
            raw,
        }
    }
}

impl fmt::Debug for SlotCurrentUsageData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotCurrentUsageData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

/// # System Slot Current Usage
#[derive(Debug, PartialEq, Eq)]
pub enum SlotCurrentUsage {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Available
    Available,
    /// In use
    InUse,
    /// Unavailable
    Unavailable,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # System Slot Current Usage Data
pub struct SlotLengthData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [SlotLength] value
    pub value: SlotLength,
}

impl Deref for SlotLengthData {
    type Target = SlotLength;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for SlotLengthData {
    fn from(raw: u8) -> Self {
        SlotLengthData {
            value: match raw {
                0x01 => SlotLength::Other,
                0x02 => SlotLength::Unknown,
                0x03 => SlotLength::ShortLength,
                0x04 => SlotLength::LongLength,
                0x05 => SlotLength::DriveFormFactor25,
                0x06 => SlotLength::DriveFormFactor35,
                _ => SlotLength::None,
            },
            raw,
        }
    }
}

impl fmt::Debug for SlotLengthData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotLengthData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

/// # System Slot Length
#[derive(Debug, PartialEq, Eq)]
pub enum SlotLength {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Short Length
    ShortLength,
    /// Long Length
    LongLength,
    /// 2.5" drive form factor
    DriveFormFactor25,
    /// 3.5" drive form factor
    DriveFormFactor35,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # System Slot Characteristics 1
pub struct SystemSlotCharacteristics1 {
    raw: u8,
}

impl Deref for SystemSlotCharacteristics1 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for SystemSlotCharacteristics1 {
    fn from(raw: u8) -> Self {
        SystemSlotCharacteristics1 { raw }
    }
}

impl SystemSlotCharacteristics1 {
    /// Characteristics unknown.
    pub fn unknown(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// Provides 5.0 volts.
    pub fn provides5_volts(&self) -> bool {
        self.raw & 0x02 == 0x02
    }

    /// Provides 3.3 volts.
    pub fn provides33_volts(&self) -> bool {
        self.raw & 0x04 == 0x04
    }

    /// Slot’s opening is shared with another slot (for example, PCI/EISA shared slot).
    pub fn shared(&self) -> bool {
        self.raw & 0x08 == 0x08
    }

    /// PC Card slot supports PC Card-16.
    pub fn supports_pc_card16(&self) -> bool {
        self.raw & 0x10 == 0x10
    }

    /// PC Card slot supports CardBus.
    pub fn supports_card_bus(&self) -> bool {
        self.raw & 0x20 == 0x20
    }

    /// PC Card slot supports Zoom Video.
    pub fn supports_zoom_video(&self) -> bool {
        self.raw & 0x40 == 0x40
    }

    /// PC Card slot supports Modem Ring Resume.
    pub fn supports_modem_ring_resume(&self) -> bool {
        self.raw & 0x80 == 0x80
    }
}

impl fmt::Debug for SystemSlotCharacteristics1 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SystemSlotCharacteristics1>())
            .field("raw", &self.raw)
            .field("unknown", &self.unknown())
            .field("provides5_volts", &self.provides5_volts())
            .field("provides33_volts", &self.provides33_volts())
            .field("shared", &self.shared())
            .field("supports_pc_card16", &self.supports_pc_card16())
            .field("supports_card_bus", &self.supports_card_bus())
            .field("supports_zoom_video", &self.supports_zoom_video())
            .field(
                "supports_modem_ring_resume",
                &self.supports_modem_ring_resume(),
            )
            .finish()
    }
}

/// # System Slot Characteristics 2
pub struct SystemSlotCharacteristics2 {
    raw: u8,
}

impl Deref for SystemSlotCharacteristics2 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for SystemSlotCharacteristics2 {
    fn from(raw: u8) -> Self {
        SystemSlotCharacteristics2 { raw }
    }
}

impl SystemSlotCharacteristics2 {
    /// PCI slot supports Power Management Event (PME#) signal.
    pub fn supports_power_management_event(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// Slot supports hot-plug devices.
    pub fn supports_hot_plug_devices(&self) -> bool {
        self.raw & 0x02 == 0x02
    }

    /// PCI slot supports SMBus signal.
    pub fn supports_smbus_signal(&self) -> bool {
        self.raw & 0x04 == 0x04
    }

    /// PCIe slot supports bifurcation.
    ///
    /// This slot can partition its lanes into two or more PCIe devices plugged into the slot.
    /// Note: This field does not indicate complete details on what levels of bifurcation
    /// are supported by the slot, but only that the slot supports some level of bifurcation.
    pub fn supports_bifurcation(&self) -> bool {
        self.raw & 0x08 == 0x08
    }

    /// Slot supports async/surprise removal.
    ///
    /// i.e., removal without prior notification to the operating system, device driver, or applications.
    pub fn supports_suprise_removal(&self) -> bool {
        self.raw & 0x10 == 0x10
    }

    /// Flexbus slot, CXL 1.0 capable.
    pub fn flexbus_slot_cxl10_capable(&self) -> bool {
        self.raw & 0x20 == 0x20
    }

    /// Flexbus slot, CXL 2.0 capable.
    pub fn flexbus_slot_cxl20_capable(&self) -> bool {
        self.raw & 0x40 == 0x40
    }
}

impl fmt::Debug for SystemSlotCharacteristics2 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SystemSlotCharacteristics2>())
            .field("raw", &self.raw)
            .field(
                "supports_power_management_event",
                &self.supports_power_management_event(),
            )
            .field(
                "supports_hot_plug_devices",
                &self.supports_hot_plug_devices(),
            )
            .field("supports_smbus_signal", &self.supports_smbus_signal())
            .field("supports_bifurcation", &self.supports_bifurcation())
            .field("supports_suprise_removal", &self.supports_suprise_removal())
            .field(
                "flexbus_slot_cxl10_capable",
                &self.flexbus_slot_cxl10_capable(),
            )
            .field(
                "flexbus_slot_cxl20_capable",
                &self.flexbus_slot_cxl20_capable(),
            )
            .finish()
    }
}

/// # Slot Peer Group entry within [SMBiosSystemSlot]
pub struct SlotPeerGroup<'a> {
    system_slot: &'a SMBiosSystemSlot<'a>,
    entry_offset: usize,
}

impl<'a> SlotPeerGroup<'a> {
    /// Size in bytes for this structure
    const SIZE: usize = 5;
    const SEGMENT_GROUP_NUMBER_OFFSET: usize = 0;
    const BUS_NUMBER_OFFSET: usize = 2;
    const DEVICE_FUNCTION_NUMBER_OFFSET: usize = 3;
    const DATA_BUS_WIDTH_OFFSET: usize = 4;

    fn new(system_slot: &'a SMBiosSystemSlot<'a>, entry_offset: usize) -> Self {
        Self {
            system_slot,
            entry_offset,
        }
    }

    /// Segment Group Number (Peer)
    pub fn segment_group_number(&self) -> Option<u16> {
        self.system_slot
            .parts()
            .get_field_word(self.entry_offset + Self::SEGMENT_GROUP_NUMBER_OFFSET)
    }

    /// Bus Number (Peer)
    pub fn bus_number(&self) -> Option<u8> {
        self.system_slot
            .parts()
            .get_field_byte(self.entry_offset + Self::BUS_NUMBER_OFFSET)
    }

    /// Device/Function Number (Peer)
    pub fn device_function_number(&self) -> Option<u8> {
        self.system_slot
            .parts()
            .get_field_byte(self.entry_offset + Self::DEVICE_FUNCTION_NUMBER_OFFSET)
    }

    /// Data bus width (Peer)
    ///
    /// Indicates electrical bus width of peer Segment/Bus/Device/Function.
    pub fn data_bus_width(&self) -> Option<u8> {
        self.system_slot
            .parts()
            .get_field_byte(self.entry_offset + Self::DATA_BUS_WIDTH_OFFSET)
    }
}

impl fmt::Debug for SlotPeerGroup<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotPeerGroup>())
            .field("segment_group_number", &self.segment_group_number())
            .field("bus_number", &self.bus_number())
            .field("device_function_number", &self.device_function_number())
            .field("data_bus_width", &self.data_bus_width())
            .finish()
    }
}

/// # On-board Device Itereator for [SlotPeerGroup]s contained within [SMBiosSystemSlot]
pub struct SlotPeerGroupIterator<'a> {
    data: &'a SMBiosSystemSlot<'a>,
    current_index: usize,
    current_entry: usize,
    number_of_entries: usize,
}

impl<'a> SlotPeerGroupIterator<'a> {
    const PEER_GROUPS_OFFSET: usize = 0x13;

    fn new(data: &'a SMBiosSystemSlot<'a>) -> Self {
        SlotPeerGroupIterator {
            data: data,
            current_index: Self::PEER_GROUPS_OFFSET,
            current_entry: 0,
            number_of_entries: data.peer_group_count().unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::PEER_GROUPS_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a SlotPeerGroupIterator<'a> {
    type Item = SlotPeerGroup<'a>;
    type IntoIter = SlotPeerGroupIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SlotPeerGroupIterator {
            data: self.data,
            current_index: SlotPeerGroupIterator::PEER_GROUPS_OFFSET,
            current_entry: 0,
            number_of_entries: self.data.peer_group_count().unwrap_or(0),
        }
    }
}

impl<'a> Iterator for SlotPeerGroupIterator<'a> {
    type Item = SlotPeerGroup<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_entries {
            self.reset();
            return None;
        }

        let next_index = self.current_index + SlotPeerGroup::SIZE;
        match self
            .data
            .parts()
            .get_field_data(self.current_index, next_index)
        {
            Some(_) => {
                let result = SlotPeerGroup::new(self.data, self.current_index);
                self.current_index = next_index;
                self.current_entry += 1;
                Some(result)
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for SlotPeerGroupIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        // System Slot structure lengths and their versions:
        // 0Ch for version 2.0 implementations
        // 0Dh for versions 2.1 to 2.5
        // 11h for versions 2.6 to 3.1.1
        // Minimum of 11h for version 3.2 and later.

        // 2.6 to 3.1.1 System Slot structure (0x11 length, it does not include _data_bus_width()_ and beyond)
        let struct_type9 = vec![
            0x09, 0x11, 0x1C, 0x00, 0x01, 0xA5, 0x0D, 0x04, 0x04, 0x00, 0x00, 0x0C, 0x01, 0x00,
            0x00, 0x00, 0x08, 0x4A, 0x36, 0x42, 0x32, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type9.as_slice());
        let test_struct = SMBiosSystemSlot::new(&parts);

        assert_eq!(test_struct.slot_designation(), Some("J6B2".to_string()));

        assert_eq!(
            *test_struct.system_slot_type().unwrap(),
            SystemSlotType::PciExpress
        );

        assert_eq!(*test_struct.slot_data_bus_width().unwrap(), SlotWidth::X16);

        // 2.6 to 3.1.1 has no data_bus_width() field or beyond fields
        assert!(test_struct.data_bus_width().is_none());

        // 3.4 System Slot structure
        let struct_type9 = vec![
            0x09, 0x1C, 0x1C, 0x00, 0x01, 0xA5, 0x0D, 0x04, 0x04, 0x00, 0x00, 0x0C, 0x01, 0x00,
            0x00, 0x00, 0x08, 0x99, 0x01, 0x23, 0x01, 0x04, 0x05, 0x06, 0x07, 0x08, 0xAB, 0x09,
            0x4A, 0x36, 0x42, 0x32, 0x00, 0x00,
        ];
        let parts = SMBiosStructParts::new(struct_type9.as_slice());
        let test_struct = SMBiosSystemSlot::new(&parts);

        // 3.2 fields
        assert_eq!(test_struct.data_bus_width(), Some(0x99));
        assert_eq!(test_struct.peer_group_count(), Some(0x01));

        let mut iterator = test_struct.peer_group_iterator().into_iter();
        let first = iterator.next().unwrap();
        assert_eq!(first.segment_group_number(), Some(0x0123));
        assert_eq!(first.bus_number(), Some(0x04));
        assert_eq!(first.device_function_number(), Some(0x05));
        assert_eq!(first.data_bus_width(), Some(0x06));

        // 3.4 fields
        // TODO:
        // Note: There will be an erratum published for these fields.  For this test case
        // the field offsets have been shifted back by 1 from 0x14, 0x15, 0x16 (+ 5 * n), to 0x13...
        assert_eq!(test_struct.slot_information(), Some(0x07));
        assert_eq!(*test_struct.slot_physical_width().unwrap(), SlotWidth::X1);
        assert_eq!(test_struct.slot_pitch(), Some(0x09AB));

        println!("{:?}", test_struct);
    }
}
