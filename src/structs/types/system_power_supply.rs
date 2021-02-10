use crate::*;

/// # System Power Supply (Type 39)
///
/// This structure identifies attributes of a system power supply. Table 116 provides details. One instance of
/// this structure is present for each possible power supply in a system.
///
/// NOTE This structure type was added in version 2.3.1 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemPowerSupply<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemPowerSupply<'a> {
    const STRUCT_TYPE: u8 = 39u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemPowerSupply<'a> {
    /// Power unit group
    ///
    /// Power unit group to which this power supply is
    /// associated
    ///
    /// Specifying the same Power Unit Group value for more
    /// than one System Power Supply structure indicates a
    /// redundant power supply configuration. The field’s value is
    /// 00h if the power supply is not a member of a redundant
    /// power unit. Non-zero values imply redundancy and that
    /// at least one other power supply will be enumerated with
    /// the same value.
    pub fn power_unit_group(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Location
    ///
    /// Identifies the location of the power supply.
    ///
    /// EXAMPLES: "in the back, on the left-hand side" or
    /// "Left Supply Bay"
    pub fn location(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    /// Device name
    ///
    /// Number of the string that names the power supply device
    ///
    /// EXAMPLE: "DR-36"
    pub fn device_name(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// Manufacturer
    ///
    /// Names the company that manufactured the supply
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Serial number
    ///
    /// The serial number for the power supply
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    /// Asset tag number
    pub fn asset_tag_number(&self) -> Option<String> {
        self.parts.get_field_string(0x09)
    }

    /// Model part number
    ///
    /// The OEM part order number
    pub fn model_part_number(&self) -> Option<String> {
        self.parts.get_field_string(0x0A)
    }

    /// Revision level
    ///
    /// Power supply revision string
    ///
    /// EXAMPLE: "2.30"
    pub fn revision_level(&self) -> Option<String> {
        self.parts.get_field_string(0x0B)
    }

    /// Max power capacity
    ///
    /// Maximum sustained power output in Watts
    ///
    /// Set to 0x8000 if unknown. Note that the units specified by
    /// the DMTF for this field are milliWatts.
    pub fn max_power_capacity(&self) -> Option<MaxPowerCapacity> {
        self.parts
            .get_field_word(0x0C)
            .and_then(|raw| Some(MaxPowerCapacity::from(raw)))
    }

    /// Power supply characteristics
    pub fn power_supply_characteristics(&self) -> Option<PowerSupplyCharacteristics> {
        self.parts
            .get_field_word(0x0E)
            .and_then(|raw| Some(PowerSupplyCharacteristics::from(raw)))
    }

    /// Input voltage probe handle
    ///
    /// Handle, or instance number, of a [SMBiosVoltageProbe] (Type 26)
    /// monitoring this power supply's input voltage
    ///
    /// A value of 0xFFFF indicates that no probe is provided
    pub fn input_voltage_probe_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x10)
    }

    /// Cooling device handle
    ///
    /// Handle, or instance number, of a [SMBiosCoolingDevice] (Type
    /// 27) associated with this power supply
    ///
    /// A value of 0xFFFF indicates that no cooling device is
    /// provided.
    pub fn cooling_device_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x12)
    }

    /// Input current probe handle
    ///
    /// Handle, or instance number, of the [SMBiosElectricalCurrentProbe]
    /// (Type 29) monitoring this power supply’s input
    /// current
    ///
    /// A value of 0xFFFF indicates that no current probe is
    /// provided.
    pub fn input_current_probe_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x14)
    }
}

impl fmt::Debug for SMBiosSystemPowerSupply<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemPowerSupply>())
            .field("header", &self.parts.header)
            .field("power_unit_group", &self.power_unit_group())
            .field("location", &self.location())
            .field("device_name", &self.device_name())
            .field("manufacturer", &self.manufacturer())
            .field("serial_number", &self.serial_number())
            .field("asset_tag_number", &self.asset_tag_number())
            .field("model_part_number", &self.model_part_number())
            .field("revision_level", &self.revision_level())
            .field("max_power_capacity", &self.max_power_capacity())
            .field(
                "power_supply_characteristics",
                &self.power_supply_characteristics(),
            )
            .field(
                "input_voltage_probe_handle",
                &self.input_voltage_probe_handle(),
            )
            .field("cooling_device_handle", &self.cooling_device_handle())
            .field(
                "input_current_probe_handle",
                &self.input_current_probe_handle(),
            )
            .finish()
    }
}

/// # Power Supply Characteristics
#[derive(PartialEq, Eq)]
pub struct PowerSupplyCharacteristics {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u16,
}

impl From<u16> for PowerSupplyCharacteristics {
    fn from(raw: u16) -> Self {
        PowerSupplyCharacteristics { raw }
    }
}

impl PowerSupplyCharacteristics {
    /// Power Supply Types
    pub fn power_supply_type(&self) -> PowerSupplyType {
        PowerSupplyType::from(self.raw)
    }

    /// Power Supply Status
    pub fn power_supply_status(&self) -> PowerSupplyStatus {
        PowerSupplyStatus::from(self.raw)
    }

    /// DMTF Input Voltage Range Switching
    pub fn input_voltage_range_switching(&self) -> InputVoltageRangeSwitching {
        InputVoltageRangeSwitching::from(self.raw)
    }

    /// Power supply is unplugged from the wall
    pub fn unplugged_from_wall(&self) -> bool {
        self.raw & 0b0000_0000_0000_0100 == 0b0000_0000_0000_0100
    }

    /// Power supply is present
    pub fn is_present(&self) -> bool {
        self.raw & 0b0000_0000_0000_0010 == 0b0000_0000_0000_0010
    }

    /// Power supply is hot-replaceable
    pub fn hot_replaceable(&self) -> bool {
        self.raw & 0b0000_0000_0000_0001 == 0b0000_0000_0000_0001
    }
}

impl fmt::Debug for PowerSupplyCharacteristics {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<PowerSupplyCharacteristics>())
            .field("raw", &self.raw)
            .field("power_supply_type", &self.power_supply_type())
            .field("power_supply_status", &self.power_supply_status())
            .field(
                "input_voltage_range_switching",
                &self.input_voltage_range_switching(),
            )
            .field("unplugged_from_wall", &self.unplugged_from_wall())
            .field("is_present", &self.is_present())
            .field("hot_replaceable", &self.hot_replaceable())
            .finish()
    }
}

/// # DMTF Power Supply Type
#[derive(Debug, PartialEq, Eq)]
pub enum PowerSupplyType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Linear
    Linear,
    /// Switching
    Switching,
    /// Battery
    Battery,
    /// UPS
    Ups,
    /// Converter
    Converter,
    /// Regulator
    Regulator,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u16> for PowerSupplyType {
    fn from(raw: u16) -> Self {
        match raw & 0b0011_1100_0000_0000 {
            0b0000_0100_0000_0000 => PowerSupplyType::Other,
            0b0000_1000_0000_0000 => PowerSupplyType::Unknown,
            0b0000_1100_0000_0000 => PowerSupplyType::Linear,
            0b0001_0000_0000_0000 => PowerSupplyType::Switching,
            0b0001_0100_0000_0000 => PowerSupplyType::Battery,
            0b0001_1000_0000_0000 => PowerSupplyType::Ups,
            0b0001_1100_0000_0000 => PowerSupplyType::Converter,
            0b0010_0000_0000_0000 => PowerSupplyType::Regulator,
            _ => PowerSupplyType::None,
        }
    }
}

/// # Power Supply Status
#[derive(Debug, PartialEq, Eq)]
pub enum PowerSupplyStatus {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// OK
    OK,
    /// Non-critical
    NonCritical,
    /// Critical; power supply has failed and has been taken off-line.
    Critical,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u16> for PowerSupplyStatus {
    fn from(raw: u16) -> Self {
        match raw & 0b0000_0011_1000_0000 {
            0b0000_0000_1000_0000 => PowerSupplyStatus::Other,
            0b0000_0001_0000_0000 => PowerSupplyStatus::Unknown,
            0b0000_0001_1000_0000 => PowerSupplyStatus::OK,
            0b0000_0010_0000_0000 => PowerSupplyStatus::NonCritical,
            0b0000_0010_1000_0000 => PowerSupplyStatus::Critical,
            _ => PowerSupplyStatus::None,
        }
    }
}

/// # DMTF Input Voltage Range Switching
#[derive(Debug, PartialEq, Eq)]
pub enum InputVoltageRangeSwitching {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Manual,
    Manual,
    /// Auto-switch
    AutoSwitch,
    /// Wide range
    WideRange,
    /// Not applicable
    NotApplicable,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u16> for InputVoltageRangeSwitching {
    fn from(raw: u16) -> Self {
        match raw & 0b0000_0000_0111_1000 {
            0b0000_0000_0000_1000 => InputVoltageRangeSwitching::Other,
            0b0000_0000_0001_0000 => InputVoltageRangeSwitching::Unknown,
            0b0000_0000_0001_1000 => InputVoltageRangeSwitching::Manual,
            0b0000_0000_0010_0000 => InputVoltageRangeSwitching::AutoSwitch,
            0b0000_0000_0010_1000 => InputVoltageRangeSwitching::WideRange,
            0b0000_0000_0011_0000 => InputVoltageRangeSwitching::NotApplicable,
            _ => InputVoltageRangeSwitching::None,
        }
    }
}

/// # Max Power Capacity
///
/// Maximum sustained power output in Watts
#[derive(Debug, PartialEq, Eq)]
pub enum MaxPowerCapacity {
    /// Maximum sustained power output in Watts
    Watts(u16),
    /// Maximum sustained power output is unknown
    Unknown,
}

impl From<u16> for MaxPowerCapacity {
    fn from(raw: u16) -> Self {
        if raw == 0x8000 {
            MaxPowerCapacity::Unknown
        } else {
            MaxPowerCapacity::Watts(raw)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type39 = vec![
            0x27, 0x16, 0x3A, 0x00, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x80,
            0xA2, 0x11, 0x36, 0x00, 0x38, 0x00, 0x39, 0x00, b'T', b'o', b' ', b'B', b'e', b' ',
            b'F', b'i', b'l', b'l', b'e', b'd', b' ', b'B', b'y', b' ', b'O', b'.', b'E', b'.',
            b'M', b'.', 0x00, b'T', b'o', b' ', b'B', b'e', b' ', b'F', b'i', b'l', b'l', b'e',
            b'd', b' ', b'B', b'y', b' ', b'O', b'.', b'E', b'.', b'M', b'.', 0x00, b'T', b'o',
            b' ', b'B', b'e', b' ', b'F', b'i', b'l', b'l', b'e', b'd', b' ', b'B', b'y', b' ',
            b'O', b'.', b'E', b'.', b'M', b'.', 0x00, b'T', b'o', b' ', b'B', b'e', b' ', b'F',
            b'i', b'l', b'l', b'e', b'd', b' ', b'B', b'y', b' ', b'O', b'.', b'E', b'.', b'M',
            b'.', 0x00, b'T', b'o', b' ', b'B', b'e', b' ', b'F', b'i', b'l', b'l', b'e', b'd',
            b' ', b'B', b'y', b' ', b'O', b'.', b'E', b'.', b'M', b'.', 0x00, b'T', b'o', b' ',
            b'B', b'e', b' ', b'F', b'i', b'l', b'l', b'e', b'd', b' ', b'B', b'y', b' ', b'O',
            b'.', b'E', b'.', b'M', b'.', 0x00, b'T', b'o', b' ', b'B', b'e', b' ', b'F', b'i',
            b'l', b'l', b'e', b'd', b' ', b'B', b'y', b' ', b'O', b'.', b'E', b'.', b'M', b'.',
            0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type39.as_slice());
        let test_struct = SMBiosSystemPowerSupply::new(&parts);

        println!("{:?}", test_struct);
        assert_eq!(test_struct.power_unit_group(), Some(1));
        assert_eq!(
            test_struct.location(),
            Some("To Be Filled By O.E.M.".to_string())
        );
        assert_eq!(
            test_struct.device_name(),
            Some("To Be Filled By O.E.M.".to_string())
        );
        assert_eq!(
            test_struct.manufacturer(),
            Some("To Be Filled By O.E.M.".to_string())
        );
        assert_eq!(
            test_struct.serial_number(),
            Some("To Be Filled By O.E.M.".to_string())
        );
        assert_eq!(
            test_struct.asset_tag_number(),
            Some("To Be Filled By O.E.M.".to_string())
        );
        assert_eq!(
            test_struct.model_part_number(),
            Some("To Be Filled By O.E.M.".to_string())
        );
        assert_eq!(
            test_struct.revision_level(),
            Some("To Be Filled By O.E.M.".to_string())
        );
        assert_eq!(
            test_struct.max_power_capacity(),
            Some(MaxPowerCapacity::Unknown)
        );
        assert_eq!(
            test_struct.power_supply_characteristics(),
            Some(PowerSupplyCharacteristics::from(4514))
        );
        assert_eq!(*test_struct.input_voltage_probe_handle().unwrap(), 54);
        assert_eq!(*test_struct.cooling_device_handle().unwrap(), 56);
        assert_eq!(*test_struct.input_current_probe_handle().unwrap(), 57);
    }
}
