use super::*;

pub struct SMBiosInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosInformation<'a> {
    const STRUCT_TYPE: u8 = 0u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosInformation<'a> {
    fn vendor(&self) -> Option<String> {
        self.parts.get_field_string(0x4)
    }

    fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x5)
    }

    fn starting_address_segment(&self) -> Option<u16> {
        self.parts.get_field_word(0x6)
    }

    fn release_date(&self) -> Option<String> {
        self.parts.get_field_string(0x8)
    }

    fn rom_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x9)
    }

    fn characteristics(&self) -> Option<u32> {
        self.parts.get_field_dword(0xA)
    }

    fn bios_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0xE)
    }

    fn system_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x10)
    }

    fn characteristics_extension0(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    fn characteristics_extension1(&self) -> Option<u8> {
        self.parts.get_field_byte(0x13)
    }

    fn system_bios_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    fn system_bios_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    fn e_c_firmware_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x16)
    }

    fn e_c_firmware_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x17)
    }

    fn extended_rom_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x18)
    }
}

impl fmt::Debug for SMBiosInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosInformation>())
        .field("header", &self.parts.header)
        .field("vendor", &self.vendor())
        .field("version", &self.version())
        .field("starting_address_segment", &self.starting_address_segment())
        .field("release_date", &self.release_date())
        .field("rom_size", &self.rom_size())
        .field("characteristics", &self.characteristics())
        .field("bios_vendor_reserved_characteristics", &self.bios_vendor_reserved_characteristics())
        .field("system_vendor_reserved_characteristics", &self.system_vendor_reserved_characteristics())
        .field("characteristics_extension0", &self.characteristics_extension0())
        .field("characteristics_extension1", &self.characteristics_extension1())
        .field("system_bios_major_release", &self.system_bios_major_release())
        .field("system_bios_minor_release", &self.system_bios_minor_release())
        .field("e_c_firmware_major_release", &self.e_c_firmware_major_release())
        .field("e_c_firmware_minor_release", &self.e_c_firmware_minor_release())
        .field("extended_rom_size", &self.extended_rom_size())
        .finish()
    }
}
