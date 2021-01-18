use super::*;

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
    pub fn slot_designation(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Slot Type
    pub fn system_slot_type_data(&self) -> Option<SystemSlotTypeData> {
        self.parts
            .get_field_byte(0x05)
            .and_then(|raw| Some(SystemSlotTypeData { raw }))
    }

    /// Slot Data Bus Width
    pub fn slot_data_bus_width(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Current Usage
    pub fn current_usage(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    /// Slot Length
    pub fn slot_length(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }

    /// Slot Id
    pub fn slot_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x09)
    }

    /// Slot Characteristics 1
    pub fn slot_characteristics_1(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0B)
    }

    /// Slot Characteristics 2
    pub fn slot_characteristics_2(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0C)
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
}

impl fmt::Debug for SMBiosSystemSlot<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemSlot>())
            .field("header", &self.parts.header)
            .field("slot_designation", &self.slot_designation())
            .field("system_slot_type_data", &self.system_slot_type_data())
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
            .finish()
    }
}

/// # System Slot Type Data
pub struct SystemSlotTypeData {
    raw: u8,
}

impl SystemSlotTypeData {
    /// System Slot Type
    pub fn system_slot_type(&self) -> SystemSlotType {
        match self.raw {
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
        }
    }
}

impl fmt::Debug for SystemSlotTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SystemSlotTypeData>())
            .field("raw", &self.raw)
            .field("slot_designation", &self.system_slot_type())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type9 = vec![
            0x09, 0x11, 0x1C, 0x00, 0x01, 0xA5, 0x0D, 0x04, 0x04, 0x00, 0x00, 0x0C, 0x01, 0x00,
            0x00, 0x00, 0x08, 0x4A, 0x36, 0x42, 0x32, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type9.as_slice());
        let test_struct = SMBiosSystemSlot::new(&parts);

        println!("{:?}", test_struct);
    }
}
