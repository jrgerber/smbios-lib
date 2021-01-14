use super::*;

pub struct SMBiosMemoryDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryDevice<'a> {
    const STRUCT_TYPE: u8 = 17u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryDevice<'a> {
    fn physical_memory_array_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x04)
    }

    fn memory_error_information_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    fn total_width(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    fn data_width(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    fn size(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    fn form_factor(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0E)
    }

    fn device_set(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    fn device_locator(&self) -> Option<String> {
        self.parts.get_field_string(0x10)
    }

    fn bank_locator(&self) -> Option<String> {
        self.parts.get_field_string(0x11)
    }

    fn memory_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    fn type_detail(&self) -> Option<u16> {
        self.parts.get_field_word(0x13)
    }

    fn speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x15)
    }

    fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x17)
    }

    fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x18)
    }

    fn asset_tag(&self) -> Option<String> {
        self.parts.get_field_string(0x19)
    }

    fn part_number(&self) -> Option<String> {
        self.parts.get_field_string(0x1A)
    }

    fn attributes(&self) -> Option<u8> {
        self.parts.get_field_byte(0x1B)
    }

    fn extended_size(&self) -> Option<u32> {
        self.parts.get_field_dword(0x1C)
    }

    fn configured_memory_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x20)
    }

    fn minimum_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x22)
    }

    fn maximum_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x24)
    }

    fn configured_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x26)
    }

    fn memory_technology(&self) -> Option<u8> {
        self.parts.get_field_byte(0x28)
    }

    fn memory_operating_mode_capability(&self) -> Option<u16> {
        self.parts.get_field_word(0x29)
    }

    fn firmware_version(&self) -> Option<String> {
        self.parts.get_field_string(0x2B)
    }

    fn module_manufacturer_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x2C)
    }

    fn module_product_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x2E)
    }

    fn memory_subsystem_controller_manufacturer_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x30)
    }

    fn memory_subsystem_controller_product_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x32)
    }

    fn non_volatile_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x34)
    }

    fn volatile_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x3C)
    }

    fn cache_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x44)
    }

    fn logical_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x4C)
    }

    fn extended_speed(&self) -> Option<u32> {
        self.parts.get_field_dword(0x54)
    }

    fn extended_configured_memory_speed(&self) -> Option<u32> {
        self.parts.get_field_dword(0x58)
    }
}

impl fmt::Debug for SMBiosMemoryDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryDevice>())
            .field("header", &self.parts.header)
            .field(
                "physical_memory_array_handle",
                &self.physical_memory_array_handle(),
            )
            .field(
                "memory_error_information_handle",
                &self.memory_error_information_handle(),
            )
            .field("total_width", &self.total_width())
            .field("data_width", &self.data_width())
            .field("size", &self.size())
            .field("form_factor", &self.form_factor())
            .field("device_set", &self.device_set())
            .field("device_locator", &self.device_locator())
            .field("bank_locator", &self.bank_locator())
            .field("memory_type", &self.memory_type())
            .field("type_detail", &self.type_detail())
            .field("speed", &self.speed())
            .field("manufacturer", &self.manufacturer())
            .field("serial_number", &self.serial_number())
            .field("asset_tag", &self.asset_tag())
            .field("part_number", &self.part_number())
            .field("attributes", &self.attributes())
            .field("extended_size", &self.extended_size())
            .field("configured_memory_speed", &self.configured_memory_speed())
            .field("minimum_voltage", &self.minimum_voltage())
            .field("maximum_voltage", &self.maximum_voltage())
            .field("configured_voltage", &self.configured_voltage())
            .field("memory_technology", &self.memory_technology())
            .field(
                "memory_operating_mode_capability",
                &self.memory_operating_mode_capability(),
            )
            .field("firmware_version", &self.firmware_version())
            .field("module_manufacturer_id", &self.module_manufacturer_id())
            .field("module_product_id", &self.module_product_id())
            .field(
                "memory_subsystem_controller_manufacturer_id",
                &self.memory_subsystem_controller_manufacturer_id(),
            )
            .field(
                "memory_subsystem_controller_product_id",
                &self.memory_subsystem_controller_product_id(),
            )
            .field("non_volatile_size", &self.non_volatile_size())
            .field("volatile_size", &self.volatile_size())
            .field("cache_size", &self.cache_size())
            .field("logical_size", &self.logical_size())
            .field("extended_speed", &self.extended_speed())
            .field(
                "extended_configured_memory_speed",
                &self.extended_configured_memory_speed(),
            )
            .finish()
    }
}
