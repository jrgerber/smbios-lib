use super::*;

/// # System Information (Type 1)
///
/// The information in this structure defines attributes of the overall system and is intended to be associated
/// with the Component ID group of the system’s MIF. An SMBIOS implementation is associated with a single
/// system instance and contains one and only one System Information (Type 1) structure.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
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
    /// Manufacturer
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Product name
    pub fn product_name(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    /// Version
    pub fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// Serial number
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    // fn uuid(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x08)
    // }

    /// Wake-up type
    ///
    /// Identifies the event that caused the system to power up.
    pub fn wakeup_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x18)
    }

    /// SKU Number
    ///
    /// This text string identifies a particular computer
    /// configuration for sale. It is sometimes also
    /// called a product ID or purchase order number.
    /// This number is frequently found in existing
    /// fields, but there is no standard format.
    /// Typically for a given system board from a
    /// given OEM, there are tens of unique
    /// processor, memory, hard drive, and optical
    /// drive configurations.
    pub fn sku_number(&self) -> Option<String> {
        self.parts.get_field_string(0x19)
    }

    /// Family
    ///
    /// This text string identifies the family to which a
    /// particular computer belongs. A family refers to
    /// a set of computers that are similar but not
    /// identical from a hardware or software point of
    /// view. Typically, a family is composed of
    /// different computer models, which have
    /// different configurations and pricing points.
    /// Computers in the same family often have
    /// similar branding and cosmetic features.
    pub fn family(&self) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type1 = vec![
            0x01, 0x1B, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04, 0xD2, 0x01, 0x25, 0x3E, 0x48, 0xE6,
            0x11, 0xE8, 0xBA, 0xD3, 0x70, 0x20, 0x84, 0x0F, 0x9D, 0x47, 0x06, 0x05, 0x06, 0x4C,
            0x45, 0x4E, 0x4F, 0x56, 0x4F, 0x00, 0x33, 0x30, 0x42, 0x46, 0x53, 0x30, 0x37, 0x35,
            0x30, 0x30, 0x00, 0x54, 0x68, 0x69, 0x6E, 0x6B, 0x53, 0x74, 0x61, 0x74, 0x69, 0x6F,
            0x6E, 0x20, 0x50, 0x35, 0x32, 0x30, 0x00, 0x4D, 0x4A, 0x30, 0x36, 0x55, 0x52, 0x44,
            0x5A, 0x00, 0x4C, 0x45, 0x4E, 0x4F, 0x56, 0x4F, 0x5F, 0x4D, 0x54, 0x5F, 0x33, 0x30,
            0x42, 0x46, 0x5F, 0x42, 0x55, 0x5F, 0x54, 0x68, 0x69, 0x6E, 0x6B, 0x5F, 0x46, 0x4D,
            0x5F, 0x54, 0x68, 0x69, 0x6E, 0x6B, 0x53, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
            0x50, 0x35, 0x32, 0x30, 0x00, 0x54, 0x68, 0x69, 0x6E, 0x6B, 0x53, 0x74, 0x61, 0x74,
            0x69, 0x6F, 0x6E, 0x20, 0x50, 0x35, 0x32, 0x30, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type1.as_slice());
        let test_struct = SMBiosSystemInformation::new(&parts);

        assert_eq!(test_struct.manufacturer(), Some("LENOVO".to_string()));
        assert_eq!(test_struct.product_name(), Some("30BFS07500".to_string()));
        assert_eq!(test_struct.version(), Some("ThinkStation P520".to_string()));
        assert_eq!(test_struct.serial_number(), Some("MJ06URDZ".to_string()));
        assert_eq!(test_struct.wakeup_type(), Some(6));
        assert_eq!(
            test_struct.sku_number(),
            Some("LENOVO_MT_30BF_BU_Think_FM_ThinkStation P520".to_string())
        );
        assert_eq!(test_struct.family(), Some("ThinkStation P520".to_string()));
    }
}
