use super::*;

pub struct SMBiosBisEntryPoint<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosBisEntryPoint<'a> {
    const STRUCT_TYPE: u8 = 31u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosBisEntryPoint<'a> {
    fn checksum(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    fn reserved_1(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn reserved_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    fn bis_entry_16(&self) -> Option<u32> {
        self.parts.get_field_dword(0x08)
    }

    fn bis_entry_32(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0C)
    }

    fn reserved_3(&self) -> Option<u64> {
        self.parts.get_field_qword(0x10)
    }

    fn reserved_4(&self) -> Option<u32> {
        self.parts.get_field_dword(0x18)
    }
}

impl fmt::Debug for SMBiosBisEntryPoint<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosBisEntryPoint>())
        .field("header", &self.parts.header)
        .field("checksum", &self.checksum())
        .field("reserved_1", &self.reserved_1())
        .field("reserved_2", &self.reserved_2())
        .field("bis_entry_16", &self.bis_entry_16())
        .field("bis_entry_32", &self.bis_entry_32())
        .field("reserved_3", &self.reserved_3())
        .field("reserved_4", &self.reserved_4())
        .finish()
    }
}

