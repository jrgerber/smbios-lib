use super::*;

pub struct SMBiosManagementDeviceThresholdData<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementDeviceThresholdData<'a> {
    const STRUCT_TYPE: u8 = 36u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosManagementDeviceThresholdData<'a> {
    fn lower_threshold_non_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x04)
    }

    fn upper_threshold_non_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    fn lower_threshold_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    fn upper_threshold_critical(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    fn lower_threshold_non_recoverable(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    fn upper_threshold_non_recoverable(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }
}

impl fmt::Debug for SMBiosManagementDeviceThresholdData<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosManagementDeviceThresholdData>())
        .field("header", &self.parts.header)
        .field("lower_threshold_non_critical", &self.lower_threshold_non_critical())
        .field("upper_threshold_non_critical", &self.upper_threshold_non_critical())
        .field("lower_threshold_critical", &self.lower_threshold_critical())
        .field("upper_threshold_critical", &self.upper_threshold_critical())
        .field("lower_threshold_non_recoverable", &self.lower_threshold_non_recoverable())
        .field("upper_threshold_non_recoverable", &self.upper_threshold_non_recoverable())
        .finish()
    }
}

