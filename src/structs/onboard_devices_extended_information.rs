use super::*;

pub struct SMBiosOnboardDevicesExtendedInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOnboardDevicesExtendedInformation<'a> {
    const STRUCT_TYPE: u8 = 41u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOnboardDevicesExtendedInformation<'a> {
    fn reference_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x4)
    }

    fn device_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x5)
    }

    fn device_type_instance(&self) -> Option<u8> {
        self.parts.get_field_byte(0x6)
    }

    fn segment_group_number(&self) -> Option<u16> {
        self.parts.get_field_word(0x7)
    }

    fn bus_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0x9)
    }

    fn device_function_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0xA)
    }
}

impl fmt::Debug for SMBiosOnboardDevicesExtendedInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOnboardDevicesExtendedInformation>())
        .field("header", &self.parts.header)
        .field("reference_designation", &self.reference_designation())
        .field("device_type", &self.device_type())
        .field("device_type_instance", &self.device_type_instance())
        .field("segment_group_number", &self.segment_group_number())
        .field("bus_number", &self.bus_number())
        .field("device_function_number", &self.device_function_number())
        .finish()
    }
}

