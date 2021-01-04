use super::*;

pub struct SMBiosProcessorInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosProcessorInformation<'a> {
    const STRUCT_TYPE: u8 = 4u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosProcessorInformation<'a> {
    fn socket_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn processor_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn processor_family(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    fn processor_manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    fn processor_id(&self) -> Option<u64> {
        self.parts.get_field_qword(0x08)
    }

    fn processor_version(&self) -> Option<String> {
        self.parts.get_field_string(0x10)
    }

    fn voltage(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    fn external_clock(&self) -> Option<u16> {
        self.parts.get_field_word(0x12)
    }

    fn max_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }

    fn current_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x16)
    }

    fn status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x18)
    }

    fn processor_upgrade(&self) -> Option<u8> {
        self.parts.get_field_byte(0x19)
    }

    fn l1cache_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x1A)
    }

    fn l2cache_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x1C)
    }

    fn l3cache_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x1E)
    }

    fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x20)
    }

    fn asset_tag(&self) -> Option<String> {
        self.parts.get_field_string(0x21)
    }

    fn part_number(&self) -> Option<String> {
        self.parts.get_field_string(0x22)
    }

    fn core_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x23)
    }

    fn core_enabled(&self) -> Option<u8> {
        self.parts.get_field_byte(0x24)
    }

    fn thread_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x25)
    }

    fn processor_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x26)
    }

    fn processor_family_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x28)
    }

    fn core_count_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2A)
    }

    fn core_enabled_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2C)
    }

    fn thread_count_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2E)
    }
}

impl fmt::Debug for SMBiosProcessorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosProcessorInformation>())
        .field("header", &self.parts.header)
        .field("socket_designation", &self.socket_designation())
        .field("processor_type", &self.processor_type())
        .field("processor_family", &self.processor_family())
        .field("processor_manufacturer", &self.processor_manufacturer())
        .field("processor_id", &self.processor_id())
        .field("processor_version", &self.processor_version())
        .field("voltage", &self.voltage())
        .field("external_clock", &self.external_clock())
        .field("max_speed", &self.max_speed())
        .field("current_speed", &self.current_speed())
        .field("status", &self.status())
        .field("processor_upgrade", &self.processor_upgrade())
        .field("l1cache_handle", &self.l1cache_handle())
        .field("l2cache_handle", &self.l2cache_handle())
        .field("l3cache_handle", &self.l3cache_handle())
        .field("serial_number", &self.serial_number())
        .field("asset_tag", &self.asset_tag())
        .field("part_number", &self.part_number())
        .field("core_count", &self.core_count())
        .field("core_enabled", &self.core_enabled())
        .field("thread_count", &self.thread_count())
        .field("processor_characteristics", &self.processor_characteristics())
        .field("processor_family_2", &self.processor_family_2())
        .field("core_count_2", &self.core_count_2())
        .field("core_enabled_2", &self.core_enabled_2())
        .field("thread_count_2", &self.thread_count_2())
        .finish()
    }
}

