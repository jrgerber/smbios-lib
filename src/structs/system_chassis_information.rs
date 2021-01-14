use super::*;

/// # System Enclosure or Chassis (Type 3)
///
/// The information in this structure (see Table 16) defines attributes of the system’s mechanical
/// enclosure(s). For example, if a system included a separate enclosure for its peripheral devices, two
/// structures would be returned: one for the main system enclosure and the second for the peripheral device
/// enclosure. The additions to this structure in version 2.1 of this specification support the population of the
/// CIM_Chassis class.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemChassisInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemChassisInformation<'a> {
    const STRUCT_TYPE: u8 = 3u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemChassisInformation<'a> {
    /// Manufacturer
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Chassis type
    /// 
    /// Bit 7 Chassis lock is present if 1.
    /// Otherwise, either a lock is not present or it is
    /// unknown if the enclosure has a lock.
    /// Bits 6:0 Enumeration value.
    pub fn chassis_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Version
    pub fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// Serial number
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Asset tag number
    pub fn asset_tag_number(&self) -> Option<String> {
        self.parts.get_field_string(0x08)
    }

    /// Boot-up State
    /// 
    /// State of the enclosure when it was last booted.
    pub fn bootup_state(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    /// Power supply state
    /// 
    /// State of the enclosure’s power supply (or
    /// supplies) when last booted
    pub fn power_supply_state(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0A)
    }

    /// Thermal state
    /// 
    /// Thermal state of the enclosure when last
    /// booted.
    pub fn thermal_state(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0B)
    }

    /// Security status
    /// 
    /// Physical security status of the enclosure when
    /// last booted.
    pub fn security_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0C)
    }

    /// OEM-defined
    /// 
    /// OEM- or BIOS vendor-specific information
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0D)
    }

    /// Height
    /// 
    /// Height of the enclosure, in 'U's
    /// A U is a standard unit of measure for the
    /// height of a rack or rack-mountable component
    /// and is equal to 1.75 inches or 4.445 cm. A
    /// value of 00h indicates that the enclosure
    /// height is unspecified.
    pub fn height(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    /// Number of power cords
    /// 
    /// Number of power cords associated with the
    /// enclosure or chassis
    /// A value of 00h indicates that the number is
    /// unspecified.
    pub fn number_of_power_cords(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    /// Contained element count (n)
    /// 
    /// Number of Contained Element records that
    /// follow, in the range 0 to 255
    /// Each Contained Element group comprises m
    /// bytes, as specified by the Contained Element
    /// Record Length field that follows. If no
    /// Contained Elements are included, this field is
    /// set to 0.
    pub fn contained_element_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x13)
    }

    /// Contained element record length (m)
    /// 
    /// Byte length of each Contained Element record
    /// that follows, in the range 0 to 255
    /// If no Contained Elements are included, this
    /// field is set to 0. For version 2.3.2 and later of
    /// this specification, this field is set to at least 03h
    /// when Contained Elements are specified.
    pub fn contained_element_record_length(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    // fn contained_elements(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x15)
    // }

    /// SKU number
    /// 
    /// Number of null-terminated string describing the
    /// chassis or enclosure SKU number
    fn sku_number(&self) -> Option<String> {
        self.parts.get_field_string(0x15)
    }
}

impl fmt::Debug for SMBiosSystemChassisInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemChassisInformation>())
            .field("header", &self.parts.header)
            .field("manufacturer", &self.manufacturer())
            .field("chassis_type", &self.chassis_type())
            .field("version", &self.version())
            .field("serial_number", &self.serial_number())
            .field("asset_tag_number", &self.asset_tag_number())
            .field("bootup_state", &self.bootup_state())
            .field("power_supply_state", &self.power_supply_state())
            .field("thermal_state", &self.thermal_state())
            .field("security_status", &self.security_status())
            .field("oem_defined", &self.oem_defined())
            .field("height", &self.height())
            .field("number_of_power_cords", &self.number_of_power_cords())
            .field("contained_element_count", &self.contained_element_count())
            .field(
                "contained_element_record_length",
                &self.contained_element_record_length(),
            )
            // .field("contained_elements", &self.contained_elements())
            .field("sku_number", &self.sku_number())
            .finish()
    }
}
