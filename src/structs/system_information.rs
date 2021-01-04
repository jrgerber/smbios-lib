use super::*;

pub struct SMBiosSystemInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemInformation<'a> {
    const STRUCT_TYPE: u8 = 1u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemInformation<'a> {
    fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn product_name(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    // fn uuid(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x08)
    // }

    fn wakeup_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x18)
    }

    fn sku_number(&self) -> Option<String> {
        self.parts.get_field_string(0x19)
    }

    fn family(&self) -> Option<String> {
        self.parts.get_field_string(0x1A)
    }
}

impl fmt::Debug for SMBiosSystemInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemInformation>())
        .field("header", &self.parts.header)
        .field("manufacturer", &self.manufacturer())
        .field("product_name", &self.product_name())
        .field("version", &self.version())
        .field("serial_number", &self.serial_number())
        // .field("uuid", &self.uuid())
        .field("wakeup_type", &self.wakeup_type())
        .field("sku_number", &self.sku_number())
        .field("family", &self.family())
        .finish()
    }
}

