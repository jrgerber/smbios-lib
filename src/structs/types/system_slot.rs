use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeSeq, ser::SerializeStruct, Serialize, Serializer};
use std::{convert::TryInto, fmt, ops::Deref};

/// # System Slots (Type 9)
///
/// The information in this structure defines the attributes of a system slot. One
/// structure is provided for each slot in the system.
pub struct SMBiosSystemSlot<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemSlot<'a> {
    const STRUCT_TYPE: u8 = 9u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
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
            .map(|raw| SystemSlotTypeData::from(raw))
    }

    /// Slot Data Bus Width
    pub fn slot_data_bus_width(&self) -> Option<SlotWidthData> {
        self.parts
            .get_field_byte(0x06)
            .map(|raw| SlotWidthData::from(raw))
    }

    /// Current Usage
    pub fn current_usage(&self) -> Option<SlotCurrentUsageData> {
        self.parts
            .get_field_byte(0x07)
            .map(|raw| SlotCurrentUsageData::from(raw))
    }

    /// Slot Length
    pub fn slot_length(&self) -> Option<SlotLengthData> {
        self.parts
            .get_field_byte(0x08)
            .map(|raw| SlotLengthData::from(raw))
    }

    /// Slot Id
    pub fn slot_id(&self) -> Option<SystemSlotId> {
        self.parts
            .get_field_data(0x09, 0x0B)
            .map(|id| SystemSlotId(id.try_into().unwrap()))
    }

    /// Slot Characteristics 1
    pub fn slot_characteristics_1(&self) -> Option<SystemSlotCharacteristics1> {
        self.parts
            .get_field_byte(0x0B)
            .map(|raw| SystemSlotCharacteristics1::from(raw))
    }

    /// Slot Characteristics 2
    pub fn slot_characteristics_2(&self) -> Option<SystemSlotCharacteristics2> {
        self.parts
            .get_field_byte(0x0C)
            .map(|raw| SystemSlotCharacteristics2::from(raw))
    }

    /// Segment Group Number (Base)
    pub fn segment_group_number(&self) -> Option<SegmentGroupNumber> {
        self.parts
            .get_field_word(0x0D)
            .map(|raw| SegmentGroupNumber::from(raw))
    }

    /// Bus Number (Base)
    pub fn bus_number(&self) -> Option<BusNumber> {
        self.parts
            .get_field_byte(0x0F)
            .map(|raw| BusNumber::from(raw))
    }

    /// Device/Function Number (Base)
    pub fn device_function_number(&self) -> Option<DeviceFunctionNumber> {
        self.parts
            .get_field_byte(0x10)
            .map(|raw| DeviceFunctionNumber::from(raw))
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
            .and_then(|size| self.parts.get_field_byte(size + 0x13))
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
                .map(|raw| SlotWidthData::from(raw))
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

    /// Slot Height
    ///
    /// This field indicates the maximum supported card height for the slot.
    ///
    /// Available in version 3.5.0 and later.
    pub fn slot_height(&self) -> Option<SlotHeightData> {
        self.peer_group_size().and_then(|size| {
            self.parts
                .get_field_byte(size + 0x17)
                .map(|raw| SlotHeightData::from(raw))
        })
    }
}

impl fmt::Debug for SMBiosSystemSlot<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemSlot<'_>>())
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

impl Serialize for SMBiosSystemSlot<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosSystemSlot", 18)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("slot_designation", &self.slot_designation())?;
        state.serialize_field("system_slot_type", &self.system_slot_type())?;
        state.serialize_field("slot_data_bus_width", &self.slot_data_bus_width())?;
        state.serialize_field("current_usage", &self.current_usage())?;
        state.serialize_field("slot_length", &self.slot_length())?;
        state.serialize_field("slot_id", &self.slot_id())?;
        state.serialize_field("slot_characteristics_1", &self.slot_characteristics_1())?;
        state.serialize_field("slot_characteristics_2", &self.slot_characteristics_2())?;
        state.serialize_field("segment_group_number", &self.segment_group_number())?;
        state.serialize_field("bus_number", &self.bus_number())?;
        state.serialize_field("device_function_number", &self.device_function_number())?;
        state.serialize_field("data_bus_width", &self.data_bus_width())?;
        state.serialize_field("peer_group_count", &self.peer_group_count())?;
        state.serialize_field("peer_group_iterator", &self.peer_group_iterator())?;
        state.serialize_field("slot_information", &self.slot_information())?;
        state.serialize_field("slot_physical_width", &self.slot_physical_width())?;
        state.serialize_field("slot_pitch", &self.slot_pitch())?;
        state.end()
    }
}

/// # System Slots - Slot Id
///
/// The Slot ID field of the System Slot structure provides a mechanism to correlate the physical attributes of
/// the slot to its logical access method (which varies based on the Slot Type field). The Slot ID field has
/// meaning only for the slot types described in the table:
///
/// | Slot Type | Slot ID Field Meaning |
/// | --------- | --------------------- |
/// | MCA | Identifies the logical Micro Channel slot number, in the range 1 to 15, in byte 0. Byte 1 is set to 0. |
/// | PCI, AGP, PCIX, PCI Express | On a system that supports ACPI, identifies the value returned in the _SUN object for this slot. On a system that supports the PCI IRQ Routing Table Specification, identifies the value present in the Slot Number field of the PCI Interrupt Routing table entry that is associated with this slot, in byte 0 - byte 1 is set to 0. The table is returned by the "Get PCI Interrupt Routing Options" PCI BIOS function call and provided directly in the PCI IRQ Routing Table Specification ($PIRQ). Software can determine the PCI bus number and device associated with the slot by matching the "Slot ID" to an entry in the routing-table and ultimately determine what device is present in that slot. NOTE: This definition also applies to the 66 MHz-capable PCI slots. |
/// | PCMCIA | Identifies the Adapter Number (byte 0) and Socket Number (byte 1) to be passed toPCMCIA Socket Services to identify this slot |
#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SystemSlotId(pub [u8; 2]);

impl Deref for SystemSlotId {
    type Target = [u8; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SystemSlotId {
    /// The first system slot Id byte (found at offset 09h).
    pub fn byte_0(&self) -> u8 {
        self.0[0]
    }

    /// The second system slot Id byte (found at offset 0Ah).
    pub fn byte_1(&self) -> u8 {
        self.0[1]
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
    pub raw: u8,
    /// The contained [SystemSlotType] value
    pub value: SystemSlotType,
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
        use M2SlotType::*;
        use MXMSlotType::*;
        use PciExpressGeneration::*;
        use PciExpressSlotWidth::*;
        use SystemSlotType::*;
        SystemSlotTypeData {
            value: match raw {
                0x01 => Other,
                0x02 => Unknown,
                0x03 => Isa,
                0x04 => Mca,
                0x05 => Eisa,
                0x06 => Pci,
                0x07 => Pcmcia,
                0x08 => VlVesa,
                0x09 => Proprietary,
                0x0A => ProcessorCardSlot,
                0x0B => ProprietaryMemoryCardSlot,
                0x0C => IORiserCardSlot,
                0x0D => NuBus,
                0x0E => Pci66MhzCapable,
                0x0F => Agp(AgpSlotWidth::X1),
                0x10 => Agp(AgpSlotWidth::X2),
                0x11 => Agp(AgpSlotWidth::X4),
                0x12 => PciX,
                0x13 => Agp(AgpSlotWidth::X8),
                0x14 => M2(M2Socket1DP),
                0x15 => M2(M2Socket1SD),
                0x16 => M2(M2Socket2),
                0x17 => M2(M2Socket3),
                0x18 => Mxm(MxmTypeI),
                0x19 => Mxm(MxmTypeII),
                0x1A => Mxm(MxmTypeIIIStandard),
                0x1B => Mxm(MxmTypeIIIHE),
                0x1C => Mxm(MxmTypeIV),
                0x1D => Mxm(Mxm3TypeA),
                0x1E => Mxm(Mxm3TypeB),
                0x1F => PciExpress(PCIExpressGen2, Sff8639),
                0x20 => PciExpress(PCIExpressGen3, Sff8639),
                0x21 => PciExpress(Undefined, PciExpressMini52WithKeepouts),
                0x22 => PciExpress(Undefined, PciExpressMini52WithoutKeepouts),
                0x23 => PciExpress(Undefined, PciExpressMini76),
                0x24 => PciExpress(PCIExpressGen4, Sff8639),
                0x25 => PciExpress(PCIExpressGen5, Sff8639),
                0x26 => OcpNic30SmallFormFactor,
                0x27 => OcpNic30LargeFormFactor,
                0x28 => OcpNicPriorTo30,
                0x30 => CxlFlexbus1,
                0xA0 => PC98C20,
                0xA1 => PC98C24,
                0xA2 => PC98E,
                0xA3 => PC98LocalBus,
                0xA4 => PC98Card,
                0xA5 => PciExpress(PCIExpressGen1, UndefinedSlotWidth),
                0xA6 => PciExpress(PCIExpressGen1, X1),
                0xA7 => PciExpress(PCIExpressGen1, X2),
                0xA8 => PciExpress(PCIExpressGen1, X4),
                0xA9 => PciExpress(PCIExpressGen1, X8),
                0xAA => PciExpress(PCIExpressGen1, X16),
                0xAB => PciExpress(PCIExpressGen2, UndefinedSlotWidth),
                0xAC => PciExpress(PCIExpressGen2, X1),
                0xAD => PciExpress(PCIExpressGen2, X2),
                0xAE => PciExpress(PCIExpressGen2, X4),
                0xAF => PciExpress(PCIExpressGen2, X8),
                0xB0 => PciExpress(PCIExpressGen2, X16),
                0xB1 => PciExpress(PCIExpressGen3, UndefinedSlotWidth),
                0xB2 => PciExpress(PCIExpressGen3, X1),
                0xB3 => PciExpress(PCIExpressGen3, X2),
                0xB4 => PciExpress(PCIExpressGen3, X4),
                0xB5 => PciExpress(PCIExpressGen3, X8),
                0xB6 => PciExpress(PCIExpressGen3, X16),
                0xB8 => PciExpress(PCIExpressGen4, UndefinedSlotWidth),
                0xB9 => PciExpress(PCIExpressGen4, X1),
                0xBA => PciExpress(PCIExpressGen4, X2),
                0xBB => PciExpress(PCIExpressGen4, X4),
                0xBC => PciExpress(PCIExpressGen4, X8),
                0xBD => PciExpress(PCIExpressGen4, X16),
                0xBE => PciExpress(PCIExpressGen5, UndefinedSlotWidth),
                0xBF => PciExpress(PCIExpressGen5, X1),
                0xC0 => PciExpress(PCIExpressGen5, X2),
                0xC1 => PciExpress(PCIExpressGen5, X4),
                0xC2 => PciExpress(PCIExpressGen5, X8),
                0xC3 => PciExpress(PCIExpressGen5, X16),
                0xC4 => PciExpress(PCIExpressGen6, UndefinedSlotWidth),
                0xC5 => EnterpriseAndDataCenter1UE1,
                0xC6 => EnterpriseAndDataCenter3InE3,
                _ => None,
            },
            raw,
        }
    }
}

impl fmt::Debug for SystemSlotTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SystemSlotTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for SystemSlotTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SystemSlotTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for SystemSlotTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            SystemSlotType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

/// # System Slot Type
#[derive(Serialize, Debug, PartialEq, Eq)]
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
    Agp(AgpSlotWidth),
    /// MXM
    Mxm(MXMSlotType),
    /// PCI-X
    PciX,
    /// M.2
    M2(M2SlotType),
    /// OCP NIC 3.0 Small Form Factor (SFF)
    OcpNic30SmallFormFactor,
    /// OCP NIC 3.0 Large Form Factor (LFF)
    OcpNic30LargeFormFactor,
    /// OCP NIC Prior to 3.0
    OcpNicPriorTo30,
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
    /// PCI Express
    PciExpress(PciExpressGeneration, PciExpressSlotWidth),
    /// Enterprise and Datacenter 1U E1 Form Factor Slot (EDSFF E1.S, E1.L) E1 slot length is reported in Slot Length field (see section 7.10.4). E1 slot pitch is reported in Slot Pitch field (see section 7.10.12). See specifications SFF-TA-1006 and SFF-TA-1007 for more details on values for slot length and pitch.
    EnterpriseAndDataCenter1UE1,
    /// Enterprise and Datacenter 3" E3 Form Factor Slot (EDSFF E3.S, E3.L) E3 slot length is reported in Slot Length field (see section 7.10.4). E3 slot pitch is reported in Slot Pitch field (see section 7.10.12). See specification SFF-TA-1008 for details on values for slot length and pitch.
    EnterpriseAndDataCenter3InE3,
    /// A value unknown to this standard, check the raw value
    None,
}

/// The generation of PciExpress used by the slot.
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PciExpressGeneration {
    /// PCI Express Gen 1
    PCIExpressGen1,
    /// PCI Express Gen 2
    PCIExpressGen2,
    /// PCI Express Gen 3
    PCIExpressGen3,
    /// PCI Express Gen 4
    PCIExpressGen4,
    /// PCI Express Gen 5
    PCIExpressGen5,
    /// PCI Express Gen 6 and Beyond
    PCIExpressGen6,
    /// Undefined
    Undefined,
}

/// The slot width of a PCI Express slot specified in the SystemSlotType
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PciExpressSlotWidth {
    /// An undefined slot width
    UndefinedSlotWidth,
    /// X1
    X1,
    /// X2
    X2,
    /// X4
    X4,
    /// X8
    X8,
    /// X16
    X16,
    /// Small form factor 639
    Sff8639,
    /// PCI Express Mini 52-pin (CEM spec. 2.0) with bottom-side keep-outs. Use Slot Length field value 03h (short length) for "half-Mini card" -only support, 04h (long length) for "full-Mini card" or dual support.
    PciExpressMini52WithKeepouts,
    /// PCI Express Mini 52-pin (CEM spec. 2.0) without bottom-side keep-outs. Use Slot Length field value 03h (short length) for "half-Mini card" -only support, 04h (long length) for "full-Mini card" or dual support.
    PciExpressMini52WithoutKeepouts,
    /// PCI Express Mini 76-pin (CEM spec. 2.0) Corresponds to Display-Mini card.
    PciExpressMini76,
}

/// The slot width of an AGP slot specified in the SystemSlotType
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum AgpSlotWidth {
    /// X1
    X1,
    /// X2
    X2,
    /// X4
    X4,
    /// X8
    X8,
}

/// An MXM SlotType
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum MXMSlotType {
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
}

/// An M.2 SlotType
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum M2SlotType {
    /// M.2 Socket 1-DP (Mechanical Key A)
    M2Socket1DP,
    /// M.2 Socket 1-SD (Mechanical Key E)
    M2Socket1SD,
    /// M.2 Socket 2 (Mechanical Key B)
    M2Socket2,
    /// M.2 Socket 3 (Mechanical Key M)
    M2Socket3,
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotWidthData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for SlotWidthData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SlotWidthData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

/// # Slot Width
#[derive(Serialize, Debug, PartialEq, Eq)]
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

/// # Slot Height Data
pub struct SlotHeightData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [SlotHeight] value
    pub value: SlotHeight,
}

impl Deref for SlotHeightData {
    type Target = SlotHeight;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for SlotHeightData {
    fn from(raw: u8) -> Self {
        SlotHeightData {
            value: match raw {
                0x00 => SlotHeight::NotApplicable,
                0x01 => SlotHeight::Other,
                0x02 => SlotHeight::Unknown,
                0x03 => SlotHeight::FullHeight,
                0x04 => SlotHeight::LowProfile,
                _ => SlotHeight::None,
            },
            raw,
        }
    }
}

impl fmt::Debug for SlotHeightData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotHeightData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for SlotHeightData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SlotHeightData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

/// # Slot Height
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum SlotHeight {
    /// Not Applicable
    NotApplicable,
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Full Height
    FullHeight,
    /// Low-profile
    LowProfile,
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotCurrentUsageData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for SlotCurrentUsageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SlotCurrentUsageData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

/// # System Slot Current Usage
#[derive(Serialize, Debug, PartialEq, Eq)]
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
        use SlotLength::*;
        SlotLengthData {
            value: match raw {
                0x01 => Other,
                0x02 => Unknown,
                0x03 => ShortLength,
                0x04 => LongLength,
                0x05 => DriveFormFactor25,
                0x06 => DriveFormFactor35,
                _ => None,
            },
            raw,
        }
    }
}

impl fmt::Debug for SlotLengthData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotLengthData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for SlotLengthData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SlotLengthData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

/// # System Slot Length
#[derive(Serialize, Debug, PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
pub struct SystemSlotCharacteristics1 {
    /// Raw value
    ///
    /// _raw_ is useful for masked comparisons.
    pub raw: u8,
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl Serialize for SystemSlotCharacteristics1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SystemSlotCharacteristics1", 9)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("unknown", &self.unknown())?;
        state.serialize_field("provides5_volts", &self.provides5_volts())?;
        state.serialize_field("provides33_volts", &self.provides33_volts())?;
        state.serialize_field("shared", &self.shared())?;
        state.serialize_field("supports_pc_card16", &self.supports_pc_card16())?;
        state.serialize_field("supports_card_bus", &self.supports_card_bus())?;
        state.serialize_field("supports_zoom_video", &self.supports_zoom_video())?;
        state.serialize_field(
            "supports_modem_ring_resume",
            &self.supports_modem_ring_resume(),
        )?;
        state.end()
    }
}

/// # System Slot Characteristics 2
#[derive(PartialEq, Eq)]
pub struct SystemSlotCharacteristics2 {
    /// Raw value
    ///
    /// _raw_ is useful when there are values not yet defiend.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl Serialize for SystemSlotCharacteristics2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SystemSlotCharacteristics2", 8)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field(
            "supports_power_management_event",
            &self.supports_power_management_event(),
        )?;
        state.serialize_field(
            "supports_hot_plug_devices",
            &self.supports_hot_plug_devices(),
        )?;
        state.serialize_field("supports_smbus_signal", &self.supports_smbus_signal())?;
        state.serialize_field("supports_bifurcation", &self.supports_bifurcation())?;
        state.serialize_field("supports_suprise_removal", &self.supports_suprise_removal())?;
        state.serialize_field(
            "flexbus_slot_cxl10_capable",
            &self.flexbus_slot_cxl10_capable(),
        )?;
        state.serialize_field(
            "flexbus_slot_cxl20_capable",
            &self.flexbus_slot_cxl20_capable(),
        )?;
        state.end()
    }
}

/// # Segment Group Number
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum SegmentGroupNumber {
    /// Single-Segment Topology (no group number)
    SingleSegment,
    /// Segment Group Number
    Number(u16),
    /// For devices that are not of types PCI, AGP, PCI-X, or PCI-Express
    /// and that do not have bus/device/function information.
    NotApplicable,
}

impl From<u16> for SegmentGroupNumber {
    fn from(raw: u16) -> Self {
        match raw {
            0x00 => SegmentGroupNumber::SingleSegment,
            0xFF => SegmentGroupNumber::NotApplicable,
            _ => SegmentGroupNumber::Number(raw),
        }
    }
}

/// # Bus Number
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BusNumber {
    /// Bus Number
    Number(u8),
    /// For devices that are not of types PCI, AGP, PCI-X, or PCI-Express
    /// and that do not have bus/device/function information.
    NotApplicable,
}

impl From<u8> for BusNumber {
    fn from(raw: u8) -> Self {
        match raw {
            0xFF => BusNumber::NotApplicable,
            _ => BusNumber::Number(raw),
        }
    }
}

/// # Device/Function Number
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum DeviceFunctionNumber {
    /// Device/Function Number
    Number {
        ///Bits 7:3 – Device number
        device: u8,
        /// Bits 2:0 – Function number
        function: u8,
    },
    /// For devices that are not of types PCI, AGP, PCI-X, or PCI-Express
    /// and that do not have bus/device/function information.
    NotApplicable,
}

impl From<u8> for DeviceFunctionNumber {
    fn from(raw: u8) -> Self {
        match raw {
            0xFF => DeviceFunctionNumber::NotApplicable,
            _ => DeviceFunctionNumber::Number {
                device: (raw & 0b11111000) >> 3,
                function: raw & 0b00000111,
            },
        }
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SlotPeerGroup<'_>>())
            .field("segment_group_number", &self.segment_group_number())
            .field("bus_number", &self.bus_number())
            .field("device_function_number", &self.device_function_number())
            .field("data_bus_width", &self.data_bus_width())
            .finish()
    }
}

impl Serialize for SlotPeerGroup<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SlotPeerGroup", 4)?;
        state.serialize_field("segment_group_number", &self.segment_group_number())?;
        state.serialize_field("bus_number", &self.bus_number())?;
        state.serialize_field("device_function_number", &self.device_function_number())?;
        state.serialize_field("data_bus_width", &self.data_bus_width())?;
        state.end()
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

impl<'a> Serialize for SlotPeerGroupIterator<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let groups: Vec<SlotPeerGroup<'_>> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(groups.len()))?;
        for e in groups {
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
        // System Slot structure lengths and their versions:
        // 0Ch for version 2.0 implementations
        // 0Dh for versions 2.1 to 2.5
        // 11h for versions 2.6 to 3.1.1
        // Minimum of 11h for version 3.2 and later.

        // 2.6 to 3.1.1 System Slot structure (0x11 length, it does not include _data_bus_width()_ and beyond)
        let struct_type9 = vec![
            0x09, 0x11, 0x1C, 0x00, 0x01, 0xA5, 0x0D, 0x04, 0x04, 0x05, 0x07, 0x0C, 0x01, 0x00,
            0x00, 0x00, 0x08, 0x4A, 0x36, 0x42, 0x32, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type9);
        let test_struct = SMBiosSystemSlot::new(&parts);

        assert_eq!(test_struct.slot_designation(), Some("J6B2".to_string()));

        assert_eq!(
            *test_struct.system_slot_type().unwrap(),
            SystemSlotType::PciExpress(
                PciExpressGeneration::PCIExpressGen1,
                PciExpressSlotWidth::UndefinedSlotWidth,
            )
        );

        assert_eq!(*test_struct.slot_data_bus_width().unwrap(), SlotWidth::X16);

        let slot_id = test_struct.slot_id().unwrap();
        assert_eq!(slot_id.byte_0(), 5);
        assert_eq!(slot_id.byte_1(), 7);

        // 2.6 to 3.1.1 has no data_bus_width() field or beyond fields
        assert!(test_struct.data_bus_width().is_none());

        // 3.4 System Slot structure
        let struct_type9 = vec![
            0x09, 0x1C, 0x1C, 0x00, 0x01, 0xA5, 0x0D, 0x04, 0x04, 0x00, 0x00, 0x0C, 0x01, 0x00,
            0x00, 0x00, 0x08, 0x99, 0x01, 0x23, 0x01, 0x04, 0x05, 0x06, 0x07, 0x08, 0xAB, 0x09,
            0x4A, 0x36, 0x42, 0x32, 0x00, 0x00,
        ];
        let parts = UndefinedStruct::new(&struct_type9);
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
