use super::*;

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
    fn power_unit_group(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn location(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    fn device_name(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    fn asset_tag_number(&self) -> Option<String> {
        self.parts.get_field_string(0x09)
    }

    fn model_part_number(&self) -> Option<String> {
        self.parts.get_field_string(0x0A)
    }

    fn revision_level(&self) -> Option<String> {
        self.parts.get_field_string(0x0B)
    }

    fn max_power_capacity(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    fn power_supply_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }

    fn input_voltage_probe_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x10)
    }

    fn cooling_device_handle(&self) -> Option<Handle> {
        self.parts.get_field_handle(0x12)
    }

    fn input_current_probe_handle(&self) -> Option<Handle> {
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
        .field("power_supply_characteristics", &self.power_supply_characteristics())
        .field("input_voltage_probe_handle", &self.input_voltage_probe_handle())
        .field("cooling_device_handle", &self.cooling_device_handle())
        .field("input_current_probe_handle", &self.input_current_probe_handle())
        .finish()
    }
}

