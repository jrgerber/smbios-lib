use crate::*;

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
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemChassisInformation<'a> {
    const STRUCT_TYPE: u8 = 3u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemChassisInformation<'a> {
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
            .and_then(|raw| Some(ChassisTypeData::from(raw)))
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
            .and_then(|raw| Some(ChassisStateData::from(raw)))
    }

    /// Power supply state
    ///
    /// State of the enclosure’s power supply (or
    /// supplies) when last booted
    pub fn power_supply_state(&self) -> Option<ChassisStateData> {
        self.parts
            .get_field_byte(0x0A)
            .and_then(|raw| Some(ChassisStateData::from(raw)))
    }

    /// Thermal state
    ///
    /// Thermal state of the enclosure when last
    /// booted.
    pub fn thermal_state(&self) -> Option<ChassisStateData> {
        self.parts
            .get_field_byte(0x0B)
            .and_then(|raw| Some(ChassisStateData::from(raw)))
    }

    /// Security status
    ///
    /// Physical security status of the enclosure when
    /// last booted.
    pub fn security_status(&self) -> Option<ChassisSecurityStatusData> {
        self.parts
            .get_field_byte(0x0C)
            .and_then(|raw| Some(ChassisSecurityStatusData::from(raw)))
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
    /// A U is a standard unit of measure for the
    /// height of a rack or rack-mountable component
    /// and is equal to 1.75 inches or 4.445 cm. A
    /// value of 00h indicates that the enclosure
    /// height is unspecified.
    pub fn height(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    /// Number of power cords
    ///
    /// Number of power cords associated with the
    /// enclosure or chassis
    /// A value of 00h indicates that the number is
    /// unspecified.
    pub fn number_of_power_cords(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
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

    //TODO:

    // fn contained_elements(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x15)
    // }

    // TODO: This sku_number offset is incorrect, if follows the elements

    /// SKU number
    ///
    /// Number of null-terminated string describing the
    /// chassis or enclosure SKU number
    fn sku_number(&self) -> Option<String> {
        self.parts.get_field_string(0x15)
    }
}

impl fmt::Debug for SMBiosSystemChassisInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemChassisInformation>())
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
            // TODO:
            // .field("contained_elements", &self.contained_elements())
            .field("sku_number", &self.sku_number())
            .finish()
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
    raw: u8,
    /// The contained [ChassisType] value
    value: ChassisType,
}

impl fmt::Debug for ChassisTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ChassisTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for ChassisTypeData {
    type Target = ChassisType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Chassis Type
#[derive(Debug, PartialEq, Eq)]
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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ChassisStateData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for ChassisStateData {
    type Target = ChassisState;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Chassis Statue
#[derive(Debug, PartialEq, Eq)]
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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ChassisSecurityStatusData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for ChassisSecurityStatusData {
    type Target = ChassisSecurityStatus;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Chassis Security Status
#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type3 = vec![
            0x03, 0x16, 0x03, 0x00, 0x01, 0x03, 0x02, 0x03, 0x04, 0x03, 0x03, 0x03, 0x03, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x03, 0x05, 0x4C, 0x45, 0x4E, 0x4F, 0x56, 0x4F,
            0x00, 0x4E, 0x6F, 0x6E, 0x65, 0x00, 0x4D, 0x4A, 0x30, 0x36, 0x55, 0x52, 0x44, 0x5A,
            0x00, 0x34, 0x30, 0x38, 0x39, 0x39, 0x38, 0x35, 0x00, 0x44, 0x65, 0x66, 0x61, 0x75,
            0x6C, 0x74, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type3.as_slice());
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
        assert_eq!(test_struct.height(), Some(0));
        assert_eq!(test_struct.number_of_power_cords(), Some(1));
        assert_eq!(test_struct.contained_element_count(), Some(0));
        assert_eq!(test_struct.contained_element_record_length(), Some(3));
        assert_eq!(test_struct.sku_number(), Some("Default string".to_string()));
    }
}
