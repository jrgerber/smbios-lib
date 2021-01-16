use super::*;

/// # Management Controller Host Interface (Type 42)
///
/// The information in this structure defines the attributes of a Management Controller Host Interface that is
/// not discoverable by “Plug and Play” mechanisms. The Type 42 structure can
/// be used to describe a physical management controller host interface and one or more protocols that
/// share that interface.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosManagementControllerHostInterface<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementControllerHostInterface<'a> {
    const STRUCT_TYPE: u8 = 42u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosManagementControllerHostInterface<'a> {
    /// Management Controller Interface Type
    pub fn interface_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Interface Type Specific Data Length
    pub fn interface_type_specific_data_length(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    // fn interface_type_specific_data(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x06)
    // }
}

impl fmt::Debug for SMBiosManagementControllerHostInterface<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosManagementControllerHostInterface>())
            .field("header", &self.parts.header)
            .field("interface_type", &self.interface_type())
            .field(
                "interface_type_specific_data_length",
                &self.interface_type_specific_data_length(),
            )
            // .field("interface_type_specific_data", &self.interface_type_specific_data())
            .finish()
    }
}
