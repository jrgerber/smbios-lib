use super::*;

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
    pub fn max_power_capacity(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Power supply characteristics
    pub fn power_supply_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
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
