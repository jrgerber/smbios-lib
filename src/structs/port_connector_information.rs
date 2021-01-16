use super::*;

/// # Port Connector Information (Type 8)
///
/// The information in this structure defines the attributes of a system port connector
/// (for example, parallel, serial, keyboard, or mouse ports). The port’s type and connector information are
/// provided. One structure is present for each port provided by the system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
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
    ///  Internal reference designator, that is,
    /// internal to the system enclosure
    ///
    /// EXAMPLE: "J101"
    pub fn internal_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Internal connector type
    pub fn internal_connector_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// External reference designation,
    /// external to the system enclosure
    ///
    /// EXAMPLE: "COM A"
    pub fn external_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// External connector type
    pub fn external_connector_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    /// Describes the function of the port
    pub fn port_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }
}

impl fmt::Debug for SMBiosPortConnectorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPortConnectorInformation>())
            .field("header", &self.parts.header)
            .field(
                "internal_reference_designator",
                &self.internal_reference_designator(),
            )
            .field("internal_connector_type", &self.internal_connector_type())
            .field(
                "external_reference_designator",
                &self.external_reference_designator(),
            )
            .field("external_connector_type", &self.external_connector_type())
            .field("port_type", &self.port_type())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type8 = vec![
            0x08, 0x09, 0x04, 0x00, 0x01, 0x00, 0x02, 0x0F, 0x0E, 0x4A, 0x31, 0x41, 0x31, 0x00,
            0x50, 0x53, 0x32, 0x4D, 0x6F, 0x75, 0x73, 0x65, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type8.as_slice());
        let test_struct = SMBiosPortConnectorInformation::new(&parts);

        assert_eq!(
            test_struct.internal_reference_designator(),
            Some("J1A1".to_string())
        );
        assert_eq!(test_struct.internal_connector_type(), Some(0));
        assert_eq!(
            test_struct.external_reference_designator(),
            Some("PS2Mouse".to_string())
        );
        assert_eq!(test_struct.external_connector_type(), Some(15));
        assert_eq!(test_struct.port_type(), Some(14));
    }
}
