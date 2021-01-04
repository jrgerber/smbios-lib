use super::*;

pub struct SMBiosPortConnectorInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosPortConnectorInformation<'a> {
    const STRUCT_TYPE: u8 = 8u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosPortConnectorInformation<'a> {
    fn internal_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    fn internal_connector_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    fn external_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    fn external_connector_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    fn port_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }
}

impl fmt::Debug for SMBiosPortConnectorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPortConnectorInformation>())
        .field("header", &self.parts.header)
        .field("internal_reference_designator", &self.internal_reference_designator())
        .field("internal_connector_type", &self.internal_connector_type())
        .field("external_reference_designator", &self.external_reference_designator())
        .field("external_connector_type", &self.external_connector_type())
        .field("port_type", &self.port_type())
        .finish()
    }
}

