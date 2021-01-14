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

