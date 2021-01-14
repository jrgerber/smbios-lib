use super::*;

pub struct SMBiosTpmDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosTpmDevice<'a> {
    const STRUCT_TYPE: u8 = 43u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosTpmDevice<'a> {
    fn vendor_id(&self) -> Option<u32> {
        self.parts.get_field_dword(0x04)
    }

    fn major_spec_version(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }

    fn minor_spec_version(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    fn firmware_version_1(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0A)
    }

    fn firmware_version_2(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0E)
    }

    fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x12)
    }

    fn characteristics(&self) -> Option<u64> {
        self.parts.get_field_qword(0x13)
    }

    fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x1B)
    }
}

impl fmt::Debug for SMBiosTpmDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosTpmDevice>())
            .field("header", &self.parts.header)
            .field("vendor_id", &self.vendor_id())
            .field("major_spec_version", &self.major_spec_version())
            .field("minor_spec_version", &self.minor_spec_version())
            .field("firmware_version_1", &self.firmware_version_1())
            .field("firmware_version_2", &self.firmware_version_2())
            .field("description", &self.description())
            .field("characteristics", &self.characteristics())
            .field("oem_defined", &self.oem_defined())
            .finish()
    }
}
