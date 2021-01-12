use super::*;

pub struct SMBiosSystemPowerControls<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemPowerControls<'a> {
    const STRUCT_TYPE: u8 = 25u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemPowerControls<'a> {
    fn next_scheduled_power_on_month(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn next_scheduled_power_on_day_of_month(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn next_scheduled_power_on_hour(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn next_scheduled_power_on_minute(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    fn next_scheduled_power_on_second(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }
}

impl fmt::Debug for SMBiosSystemPowerControls<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemPowerControls>())
        .field("header", &self.parts.header)
        .field("next_scheduled_power_on_month", &self.next_scheduled_power_on_month())
        .field("next_scheduled_power_on_day_of_month", &self.next_scheduled_power_on_day_of_month())
        .field("next_scheduled_power_on_hour", &self.next_scheduled_power_on_hour())
        .field("next_scheduled_power_on_minute", &self.next_scheduled_power_on_minute())
        .field("next_scheduled_power_on_second", &self.next_scheduled_power_on_second())
        .finish()
    }
}

