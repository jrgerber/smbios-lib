use crate::core::UndefinedStruct;
use crate::{BoardTypeData, SMBiosStruct, SMBiosType};
use serde::{ser::SerializeSeq, ser::SerializeStruct, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

/// # System Enclosure or Chassis (Type 3)
///
/// The information in this structure (see Table 16) defines attributes of the system’s mechanical
/// enclosure(s). For example, if a system included a separate enclosure for its peripheral devices, two
/// structures would be returned: one for the main system enclosure and the second for the peripheral device
/// enclosure. The additions to this structure in version 2.1 of this specification support the population of the
/// CIM_Chassis class.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemChassisInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemChassisInformation<'a> {
    const STRUCT_TYPE: u8 = 3u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosSystemChassisInformation<'a> {
    const CONTAINED_ELEMENTS_OFFSET: usize = 0x15usize;

    /// Manufacturer
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Chassis type
    ///
    /// Bit 7 Chassis lock is present if 1.
    /// Otherwise, either a lock is not present or it is
    /// unknown if the enclosure has a lock.
    /// Bits 6:0 Enumeration value.
    pub fn chassis_type(&self) -> Option<ChassisTypeData> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| ChassisTypeData::from(raw))
    }

    /// Version
    pub fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// Serial number
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Asset tag number
    pub fn asset_tag_number(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    /// Boot-up State
    ///
    /// State of the enclosure when it was last booted.
    pub fn bootup_state(&self) -> Option<ChassisStateData> {
        self.parts
            .get_field_byte(0x09)
            .map(|raw| ChassisStateData::from(raw))
    }

    /// Power supply state
    ///
    /// State of the enclosure’s power supply (or
    /// supplies) when last booted
    pub fn power_supply_state(&self) -> Option<ChassisStateData> {
        self.parts
            .get_field_byte(0x0A)
            .map(|raw| ChassisStateData::from(raw))
    }

    /// Thermal state
    ///
    /// Thermal state of the enclosure when last
    /// booted.
    pub fn thermal_state(&self) -> Option<ChassisStateData> {
        self.parts
            .get_field_byte(0x0B)
            .map(|raw| ChassisStateData::from(raw))
    }

    /// Security status
    ///
    /// Physical security status of the enclosure when
    /// last booted.
    pub fn security_status(&self) -> Option<ChassisSecurityStatusData> {
        self.parts
            .get_field_byte(0x0C)
            .map(|raw| ChassisSecurityStatusData::from(raw))
    }

    /// OEM-defined
    ///
    /// OEM- or BIOS vendor-specific information
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0D)
    }

    /// Height
    ///
    /// Height of the enclosure, in 'U's
    ///
    /// A U is a standard unit of measure for the
    /// height of a rack or rack-mountable component
    /// and is equal to 1.75 inches or 4.445 cm.
    pub fn height(&self) -> Option<ChassisHeight> {
        self.parts
            .get_field_byte(0x11)
            .map(|raw| ChassisHeight::from(raw))
    }

    /// Number of power cords
    ///
    /// Number of power cords associated with the
    /// enclosure or chassis
    pub fn number_of_power_cords(&self) -> Option<PowerCords> {
        self.parts
            .get_field_byte(0x12)
            .map(|raw| PowerCords::from(raw))
    }

    /// Contained element count (n)
    ///
    /// Number of Contained Element records that
    /// follow, in the range 0 to 255
    /// Each Contained Element group comprises m
    /// bytes, as specified by the Contained Element
    /// Record Length field that follows. If no
    /// Contained Elements are included, this field is
    /// set to 0.
    pub fn contained_element_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x13)
    }

    /// Contained element record length (m)
    ///
    /// Byte length of each Contained Element record
    /// that follows, in the range 0 to 255
    /// If no Contained Elements are included, this
    /// field is set to 0. For version 2.3.2 and later of
    /// this specification, this field is set to at least 03h
    /// when Contained Elements are specified.
    pub fn contained_element_record_length(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    fn contained_elements_size(&self) -> Option<usize> {
        self.contained_element_record_length().and_then(|m| {
            self.contained_element_count()
                .and_then(|n| Some(m as usize * n as usize))
        })
    }

    /// Contained Elements
    pub fn contained_elements(&self) -> Option<ContainedElements<'_>> {
        ContainedElements::new(self)
    }

    /// SKU number
    ///
    /// Chassis or enclosure SKU number
    pub fn sku_number(&self) -> Option<String> {
        self.contained_elements_size().and_then(|size| {
            self.parts
                .get_field_string(Self::CONTAINED_ELEMENTS_OFFSET + size)
        })
    }
}

impl fmt::Debug for SMBiosSystemChassisInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemChassisInformation<'_>>())
            .field("header", &self.parts.header)
            .field("manufacturer", &self.manufacturer())
            .field("chassis_type", &self.chassis_type())
            .field("version", &self.version())
            .field("serial_number", &self.serial_number())
            .field("asset_tag_number", &self.asset_tag_number())
            .field("bootup_state", &self.bootup_state())
            .field("power_supply_state", &self.power_supply_state())
            .field("thermal_state", &self.thermal_state())
            .field("security_status", &self.security_status())
            .field("oem_defined", &self.oem_defined())
            .field("height", &self.height())
            .field("number_of_power_cords", &self.number_of_power_cords())
            .field("contained_element_count", &self.contained_element_count())
            .field(
                "contained_element_record_length",
                &self.contained_element_record_length(),
            )
            .field("contained_elements", &self.contained_elements())
            .field("sku_number", &self.sku_number())
            .finish()
    }
}

impl Serialize for SMBiosSystemChassisInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosSystemChassisInformation", 17)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("manufacturer", &self.manufacturer())?;
        state.serialize_field("chassis_type", &self.chassis_type())?;
        state.serialize_field("version", &self.version())?;
        state.serialize_field("serial_number", &self.serial_number())?;
        state.serialize_field("asset_tag_number", &self.asset_tag_number())?;
        state.serialize_field("bootup_state", &self.bootup_state())?;
        state.serialize_field("power_supply_state", &self.power_supply_state())?;
        state.serialize_field("thermal_state", &self.thermal_state())?;
        state.serialize_field("security_status", &self.security_status())?;
        state.serialize_field("oem_defined", &self.oem_defined())?;
        state.serialize_field("height", &self.height())?;
        state.serialize_field("number_of_power_cords", &self.number_of_power_cords())?;
        state.serialize_field("contained_element_count", &self.contained_element_count())?;
        state.serialize_field(
            "contained_element_record_length",
            &self.contained_element_record_length(),
        )?;
        state.serialize_field("contained_elements", &self.contained_elements())?;
        state.serialize_field("sku_number", &self.sku_number())?;
        state.end()
    }
}

/// # Chassis Height
#[derive(Serialize, Debug)]
pub enum ChassisHeight {
    /// A chassis enclosure height is not specified.
    Unspecified,
    /// Height of the enclosure, in 'U's
    ///
    /// A U is a standard unit of measure for the height of a rack
    /// or rack-mountable component and is equal to 1.75 inches or
    /// 4.445 cm.
    U(u8),
}

impl From<u8> for ChassisHeight {
    fn from(raw: u8) -> Self {
        match raw {
            0 => ChassisHeight::Unspecified,
            _ => ChassisHeight::U(raw),
        }
    }
}

/// # Number of Power Cords
#[derive(Serialize, Debug)]
pub enum PowerCords {
    /// The number of power cords is not specified.
    Unspecified,
    /// The number of power cords
    Count(u8),
}

impl From<u8> for PowerCords {
    fn from(raw: u8) -> Self {
        match raw {
            0 => PowerCords::Unspecified,
            _ => PowerCords::Count(raw),
        }
    }
}

/// # Chassis Type Data
pub struct ChassisTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ChassisType] value
    pub value: ChassisType,
}

impl fmt::Debug for ChassisTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ChassisTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ChassisTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ChassisTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ChassisTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ChassisType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ChassisTypeData {
    type Target = ChassisType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Chassis Type
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ChassisType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Desktop
    Desktop,
    /// Low Profile Desktop
    LowProfileDesktop,
    /// Pizza Box
    PizzaBox,
    /// Mini Tower
    MiniTower,
    /// Tower
    Tower,
    /// Portable
    Portable,
    /// Laptop
    Laptop,
    /// Notebook
    Notebook,
    /// Hand Held
    HandHeld,
    /// Docking Station
    DockingStation,
    /// All in One
    AllInOne,
    /// Sub Notebook
    SubNotebook,
    /// Space-saving
    SpaceSaving,
    /// Lunch Box
    LunchBox,
    /// Main Server Chassis
    MainServerChassis,
    /// Expansion Chassis
    ExpansionChassis,
    /// SubChassis
    SubChassis,
    /// Bus Expansion Chassis
    BusExpansionChassis,
    /// Peripheral Chassis
    PeripheralChassis,
    /// RAID Chassis
    RaidChassis,
    /// Rack Mount Chassis
    RackMountChassis,
    /// Sealed-case PC
    SealedCasePC,
    /// Multi-system chassis
    MultiSystemChassis,
    /// Compact PCI
    CompactPci,
    /// Advanced TCA
    AdvancedTca,
    /// Blade
    Blade,
    /// Blade Encloser
    BladeEnclosure,
    /// Tablet
    Tablet,
    /// Convertible
    Convertible,
    /// Detachable
    Detachable,
    /// IoT Gateway
    IoTGateway,
    /// Embedded PC
    EmbeddedPC,
    /// Mini PC
    MiniPC,
    /// Stick PC
    StickPC,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ChassisTypeData {
    fn from(raw: u8) -> Self {
        ChassisTypeData {
            value: match raw {
                0x01 => ChassisType::Other,
                0x02 => ChassisType::Unknown,
                0x03 => ChassisType::Desktop,
                0x04 => ChassisType::LowProfileDesktop,
                0x05 => ChassisType::PizzaBox,
                0x06 => ChassisType::MiniTower,
                0x07 => ChassisType::Tower,
                0x08 => ChassisType::Portable,
                0x09 => ChassisType::Laptop,
                0x0A => ChassisType::Notebook,
                0x0B => ChassisType::HandHeld,
                0x0C => ChassisType::DockingStation,
                0x0D => ChassisType::AllInOne,
                0x0E => ChassisType::SubNotebook,
                0x0F => ChassisType::SpaceSaving,
                0x10 => ChassisType::LunchBox,
                0x11 => ChassisType::MainServerChassis,
                0x12 => ChassisType::ExpansionChassis,
                0x13 => ChassisType::SubChassis,
                0x14 => ChassisType::BusExpansionChassis,
                0x15 => ChassisType::PeripheralChassis,
                0x16 => ChassisType::RaidChassis,
                0x17 => ChassisType::RackMountChassis,
                0x18 => ChassisType::SealedCasePC,
                0x19 => ChassisType::MultiSystemChassis,
                0x1A => ChassisType::CompactPci,
                0x1B => ChassisType::AdvancedTca,
                0x1C => ChassisType::Blade,
                0x1D => ChassisType::BladeEnclosure,
                0x1E => ChassisType::Tablet,
                0x1F => ChassisType::Convertible,
                0x20 => ChassisType::Detachable,
                0x21 => ChassisType::IoTGateway,
                0x22 => ChassisType::EmbeddedPC,
                0x23 => ChassisType::MiniPC,
                0x24 => ChassisType::StickPC,
                _ => ChassisType::None,
            },
            raw,
        }
    }
}

/// # Chassis State Data
pub struct ChassisStateData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    raw: u8,
    /// The contained [ChassisState] value
    value: ChassisState,
}

impl fmt::Debug for ChassisStateData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ChassisStateData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ChassisStateData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ChassisStateData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for ChassisStateData {
    type Target = ChassisState;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Chassis Statue
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ChassisState {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Safe
    Safe,
    /// Warning
    Warning,
    /// Critical
    Critical,
    /// Non-recoverable
    NonRecoverable,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ChassisStateData {
    fn from(raw: u8) -> Self {
        ChassisStateData {
            value: match raw {
                0x01 => ChassisState::Other,
                0x02 => ChassisState::Unknown,
                0x03 => ChassisState::Safe,
                0x04 => ChassisState::Warning,
                0x05 => ChassisState::Critical,
                0x06 => ChassisState::NonRecoverable,
                _ => ChassisState::None,
            },
            raw,
        }
    }
}

/// # Chassis Security Status Data
pub struct ChassisSecurityStatusData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    raw: u8,
    /// The contained [ChassisSecurityStatus] value
    value: ChassisSecurityStatus,
}

impl fmt::Debug for ChassisSecurityStatusData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ChassisSecurityStatusData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ChassisSecurityStatusData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ChassisSecurityStatusData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for ChassisSecurityStatusData {
    type Target = ChassisSecurityStatus;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Chassis Security Status
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ChassisSecurityStatus {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// None
    StatusNone,
    /// External interface locked out
    ExternalInterfaceLockedOut,
    /// External interface enabled
    ExternalInterfaceEnabled,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ChassisSecurityStatusData {
    fn from(raw: u8) -> Self {
        ChassisSecurityStatusData {
            value: match raw {
                0x01 => ChassisSecurityStatus::Other,
                0x02 => ChassisSecurityStatus::Unknown,
                0x03 => ChassisSecurityStatus::StatusNone,
                0x04 => ChassisSecurityStatus::ExternalInterfaceLockedOut,
                0x05 => ChassisSecurityStatus::ExternalInterfaceEnabled,
                _ => ChassisSecurityStatus::None,
            },
            raw,
        }
    }
}

/// # Contained Elements
pub struct ContainedElements<'a> {
    raw: &'a [u8],
    record_count: usize,
    record_length: usize,
}

impl<'a> ContainedElements<'a> {
    fn new(chassis_information: &'a SMBiosSystemChassisInformation<'a>) -> Option<Self> {
        chassis_information
            .contained_element_record_length()
            .and_then(|record_length| {
                chassis_information
                    .contained_element_count()
                    .and_then(|record_count| {
                        chassis_information
                            .parts()
                            .get_field_data(
                                SMBiosSystemChassisInformation::CONTAINED_ELEMENTS_OFFSET,
                                SMBiosSystemChassisInformation::CONTAINED_ELEMENTS_OFFSET
                                    + (record_length as usize * record_count as usize),
                            )
                            .and_then(|raw| {
                                Some(Self {
                                    raw,
                                    record_count: record_count as usize,
                                    record_length: record_length as usize,
                                })
                            })
                    })
            })
    }
}

impl<'a> fmt::Debug for ContainedElements<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ContainedElements<'_>>())
            .field("records", &self.into_iter())
            .finish()
    }
}

impl<'a> Serialize for ContainedElements<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.record_count))?;
        for e in self {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

/// # Contained Chassis Element
pub struct ChassisElement<'a> {
    /// Raw byte slice for this chassis element
    pub raw: &'a [u8],
}

impl<'a> ChassisElement<'a> {
    const MINIMUM_RAW_SIZE: usize = 3usize;
    const ELEMENT_TYPE_OFFSET: usize = 0usize;
    const ELEMENT_MINIMUM_OFFSET: usize = 1usize;
    const ELEMENT_MAXIMUM_OFFSET: usize = 2usize;

    fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < Self::MINIMUM_RAW_SIZE {
            None
        } else {
            Some(Self { raw })
        }
    }

    /// Contained Element Type
    pub fn element_type(&self) -> ElementType {
        ElementType::from(self.raw[Self::ELEMENT_TYPE_OFFSET])
    }

    /// Contained Element Minimum
    pub fn element_minimum(&self) -> ElementMinimum {
        ElementMinimum::from(self.raw[Self::ELEMENT_MINIMUM_OFFSET])
    }

    /// Contained Element Maximum
    pub fn element_maximum(&self) -> ElementMaximum {
        ElementMaximum::from(self.raw[Self::ELEMENT_MAXIMUM_OFFSET])
    }
}

impl fmt::Debug for ChassisElement<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ChassisElement<'_>>())
            .field("raw", &self.raw)
            .field("element_type", &self.element_type())
            .field("element_minimum", &self.element_minimum())
            .field("element_maximum", &self.element_maximum())
            .finish()
    }
}

impl Serialize for ChassisElement<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ChassisElement", 4)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("element_type", &self.element_type())?;
        state.serialize_field("element_minimum", &self.element_minimum())?;
        state.serialize_field("element_maximum", &self.element_maximum())?;
        state.end()
    }
}

/// # Contained Element Type
#[derive(Serialize, Debug)]
pub enum ElementType {
    /// SMBIOS Baseboard Type enumeration
    BaseboardType(BoardTypeData),
    /// SMBIOS structure type enumeration
    SMBiosType(SMBiosType),
}

impl From<u8> for ElementType {
    fn from(raw: u8) -> Self {
        if raw & 0b1000_0000 == 0b1000_0000 {
            ElementType::SMBiosType(SMBiosType(raw & 0b0111_1111))
        } else {
            ElementType::BaseboardType(BoardTypeData::from(raw))
        }
    }
}

/// # Contained Element Minimum
///
/// Specifies the minimum number of the 'element_type' that can be
/// installed in the chassis for the chassis to properly operate,
/// in the range 0 to 254.
#[derive(Serialize, Debug)]
pub enum ElementMinimum {
    /// Specifies the minimum number of the 'element_type' that can be
    /// installed in the chassis for the chassis to properly operate,
    ///  in the range 0 to 254.
    Count(u8),
    /// The value 255 (0FFh) is reserved for future definition by this specification.
    Reserved,
}

impl From<u8> for ElementMinimum {
    fn from(raw: u8) -> Self {
        match raw {
            0xFF => ElementMinimum::Reserved,
            _ => ElementMinimum::Count(raw),
        }
    }
}

/// # Contained Element Maximum
///
/// Specifies the minimum number of the 'element_type' that can be
/// installed in the chassis in the range 0 to 254.
#[derive(Serialize, Debug)]
pub enum ElementMaximum {
    /// Specifies the maximum number of the 'element_type' that can be
    /// installed in the chassis for the chassis to properly operate,
    ///  in the range 1 to 255.
    Count(u8),
    /// The value 0 is reserved for future definition by this specification.
    Reserved,
}

impl From<u8> for ElementMaximum {
    fn from(raw: u8) -> Self {
        match raw {
            0x00 => ElementMaximum::Reserved,
            _ => ElementMaximum::Count(raw),
        }
    }
}

/// # Iterates over the [ChassisElement] entries within [ContainedElements]
pub struct ContainedElementsIterator<'a> {
    contained_elements: &'a ContainedElements<'a>,
    current_index: usize,
    current_entry: usize,
}

impl<'a> ContainedElementsIterator<'a> {
    fn reset(&mut self) {
        self.current_index = 0;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a ContainedElements<'a> {
    type Item = ChassisElement<'a>;
    type IntoIter = ContainedElementsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ContainedElementsIterator {
            contained_elements: self,
            current_index: 0,
            current_entry: 0,
        }
    }
}

impl<'a> Iterator for ContainedElementsIterator<'a> {
    type Item = ChassisElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.contained_elements.record_count {
            self.reset();
            return None;
        }

        let next_index = self.current_index + self.contained_elements.record_length;
        match ChassisElement::new(&self.contained_elements.raw[self.current_index..next_index]) {
            Some(chassis_element) => {
                self.current_index = next_index;
                self.current_entry += 1;
                Some(chassis_element)
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for ContainedElementsIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list()
            .entries(self.contained_elements.into_iter())
            .finish()
    }
}

impl<'a> Serialize for ContainedElementsIterator<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.count()))?;
        for e in self.into_iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::types::{
        BoardType, ChassisHeight, ChassisSecurityStatus, ChassisState, ChassisType, ElementType,
        PowerCords, SMBiosSystemChassisInformation,
    };

    #[test]
    fn unit_test() {
        let struct_type3 = vec![
            0x03, 0x1C, 0x03, 0x00, 0x01, 0x03, 0x02, 0x03, 0x04, 0x03, 0x03, 0x03, 0x03, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x85, 0x00, 0x02, 0x05, 0x00, 0x02, 0x05,
            b'L', b'E', b'N', b'O', b'V', b'O', 0x00, b'N', b'o', b'n', b'e', 0x00, b'M', b'J',
            b'0', b'6', b'U', b'R', b'D', b'Z', 0x00, b'4', b'0', b'8', b'9', b'9', b'8', b'5',
            0x00, b'D', b'e', b'f', b'a', b'u', b'l', b't', b' ', b's', b't', b'r', b'i', b'n',
            b'g', 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type3);
        let test_struct = SMBiosSystemChassisInformation::new(&parts);

        assert_eq!(test_struct.manufacturer(), Some("LENOVO".to_string()));
        assert_eq!(*test_struct.chassis_type().unwrap(), ChassisType::Desktop);
        assert_eq!(test_struct.version(), Some("None".to_string()));
        assert_eq!(test_struct.serial_number(), Some("MJ06URDZ".to_string()));
        assert_eq!(test_struct.asset_tag_number(), Some("4089985".to_string()));
        assert_eq!(*test_struct.bootup_state().unwrap(), ChassisState::Safe);
        assert_eq!(
            *test_struct.power_supply_state().unwrap(),
            ChassisState::Safe
        );
        assert_eq!(*test_struct.thermal_state().unwrap(), ChassisState::Safe);
        assert_eq!(
            *test_struct.security_status().unwrap(),
            ChassisSecurityStatus::StatusNone
        );
        assert_eq!(test_struct.oem_defined(), Some(0));
        match test_struct.height().unwrap() {
            ChassisHeight::U(_) => panic!("expected no height specified"),
            ChassisHeight::Unspecified => (),
        }
        match test_struct.number_of_power_cords().unwrap() {
            PowerCords::Count(count) => assert_eq!(count, 1),
            PowerCords::Unspecified => panic!("expected a count"),
        }
        assert_eq!(test_struct.contained_element_count(), Some(2));
        assert_eq!(test_struct.contained_element_record_length(), Some(3));
        let contained_elements = test_struct.contained_elements().unwrap();
        let mut iterator = contained_elements.into_iter();
        let first = iterator.next().unwrap();
        match first.element_type() {
            ElementType::SMBiosType(bios_type) => {
                assert_eq!(*bios_type, 5)
            }
            _ => panic!("expected SMBIOS type"),
        }
        let second = iterator.next().unwrap();
        match second.element_type() {
            ElementType::BaseboardType(baseboard_type) => {
                assert_eq!(*baseboard_type, BoardType::SystemManagementModule)
            }
            _ => panic!("expected baseboard type"),
        }
        assert_eq!(test_struct.sku_number(), Some("Default string".to_string()));
    }
}
