use super::*;

/// #  BIOS Information (Type 0)
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
    /// BIOS vendor's name
    pub fn vendor(&self) -> Option<String> {
        self.parts.get_field_string(0x4)
    }

    /// BIOS version
    ///
    /// This value is a free-form string that may contain
    /// Core and OEM version information.
    pub fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x5)
    }

    /// BIOS starting address segment
    ///
    /// Segment location of BIOS starting address
    /// (for example, 0E800h).
    ///
    /// NOTE: The size of the runtime BIOS image can
    /// be computed by subtracting the Starting
    /// Address Segment from 10000h and
    /// multiplying the result by 16.
    pub fn starting_address_segment(&self) -> Option<u16> {
        self.parts.get_field_word(0x6)
    }

    /// BIOS release date
    ///
    /// The date string, if supplied, is in either
    /// mm/dd/yy or mm/dd/yyyy format. If the year
    /// portion of the string is two digits, the year is
    /// assumed to be 19yy.
    ///
    /// NOTE: The mm/dd/yyyy format is required for
    /// SMBIOS version 2.3 and later.
    pub fn release_date(&self) -> Option<String> {
        self.parts.get_field_string(0x8)
    }

    /// BIOS ROM size
    ///
    /// Size (n) where 64K * (n+1) is the size of the
    /// physical device containing the BIOS, in
    /// bytes.
    ///
    /// FFh - size is 16MB or greater, see Extended
    /// BIOS ROM Size for actual size
    pub fn rom_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x9)
    }

    /// BIOS characteristics
    ///
    /// Defines which functions the BIOS supports:
    /// PCI, PCMCIA, Flash, etc
    pub fn characteristics(&self) -> Option<u32> {
        self.parts.get_field_dword(0xA)
    }

    /// BIOS vendor reserved characteristics
    pub fn bios_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0xE)
    }

    /// System vendor reserved characteristics
    pub fn system_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x10)
    }

    /// Characteristics extension byte 0
    pub fn characteristics_extension0(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    /// Characteristics extension byte 1
    pub fn characteristics_extension1(&self) -> Option<u8> {
        self.parts.get_field_byte(0x13)
    }

    /// System BIOS major release
    ///
    /// Identifies the major release of the System
    /// BIOS; for example, the value is 0Ah for
    /// revision 10.22 and 02h for revision 2.1.
    ///
    /// This field or the System BIOS Minor
    /// Release field or both are updated each time
    /// a System BIOS update for a given system is
    /// released.
    ///
    /// If the system does not support the use of
    /// this field, the value is 0FFh for both this field
    /// and the System BIOS Minor Release field.
    pub fn system_bios_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    /// System BIOS minor release
    ///
    /// Identifies the minor release of the System
    /// BIOS; for example, the value is 16h for
    /// revision 10.22 and 01h for revision 2.1.
    pub fn system_bios_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    /// Embedded controller firmware major release
    ///
    /// Identifies the major release of the
    /// embedded controller firmware; for example,
    /// the value would be 0Ah for revision 10.22
    /// and 02h for revision 2.1.
    ///
    /// This field or the Embedded Controller
    /// Firmware Minor Release field or both are
    /// updated each time an embedded controller
    /// firmware update for a given system is
    /// released.
    ///
    /// If the system does not have field
    /// upgradeable embedded controller firmware,
    /// the value is 0FFh.
    pub fn e_c_firmware_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x16)
    }

    /// Embedded controller firmware minor release
    ///
    /// Identifies the minor release of the
    /// embedded controller firmware; for example,
    /// the value is 16h for revision 10.22 and 01h
    /// for revision 2.1.
    /// If the system does not have field
    /// upgradeable embedded controller firmware,
    /// the value is 0FFh.
    pub fn e_c_firmware_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x17)
    }

    /// Extended BIOS ROM size
    ///
    /// Extended size of the physical device(s)
    /// containing the BIOS, rounded up if needed.
    ///
    /// Bits 15:14 Unit
    /// 00b - megabytes
    /// 01b - gigabytes
    /// 10b - reserved
    /// 11b - reserved
    /// Bits 13:0 Size
    ///
    /// Examples: a 16 MB device would be
    /// represented as 0010h. A 48 GB device set
    /// would be represented as
    /// 0100_0000_0011_0000b or 4030h.
    pub fn extended_rom_size(&self) -> Option<u8> {
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
            .field(
                "bios_vendor_reserved_characteristics",
                &self.bios_vendor_reserved_characteristics(),
            )
            .field(
                "system_vendor_reserved_characteristics",
                &self.system_vendor_reserved_characteristics(),
            )
            .field(
                "characteristics_extension0",
                &self.characteristics_extension0(),
            )
            .field(
                "characteristics_extension1",
                &self.characteristics_extension1(),
            )
            .field(
                "system_bios_major_release",
                &self.system_bios_major_release(),
            )
            .field(
                "system_bios_minor_release",
                &self.system_bios_minor_release(),
            )
            .field(
                "e_c_firmware_major_release",
                &self.e_c_firmware_major_release(),
            )
            .field(
                "e_c_firmware_minor_release",
                &self.e_c_firmware_minor_release(),
            )
            .field("extended_rom_size", &self.extended_rom_size())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        // BIOS Information structure is sensitive to BIOS specification versions
        // and prone to bugs.  Therefore, it is important to test different
        // structure versions.
        //
        // The length field specifies:
        // 12h + number of BIOS Characteristics
        // Extension Bytes. If no Extension Bytes are
        // used the Length is 12h.
        //
        // For version 2.1 and 2.2 implementations, the length is 13h
        // because one extension byte is defined.
        //
        // For version 2.3 and later implementations, the
        // length is at least 14h because two extension
        // bytes are defined.
        //
        // For version 2.4 to 3.0, implementations, the length
        // is at least 18h because bytes 14-17h are defined.
        //
        // For version 3.1 and later implementations, the
        // length is at least 1Ah because bytes 14-19h
        // are defined.

        // 2.4 to 3.0 BIOS Information structure.  Does not include _extended_rom_size()_
        // field or fields beyond.
        let struct_type0 = vec![
            0x00, 0x18, 0x00, 0x00, 0x01, 0x02, 0x00, 0xF0, 0x03, 0xFF, 0x80, 0x98, 0x8B, 0x3F,
            0x01, 0x00, 0x11, 0x00, 0x03, 0x0D, 0x00, 0x21, 0x11, 0x2D, 0x4C, 0x45, 0x4E, 0x4F,
            0x56, 0x4F, 0x00, 0x53, 0x30, 0x33, 0x4B, 0x54, 0x33, 0x33, 0x41, 0x00, 0x30, 0x38,
            0x2F, 0x30, 0x36, 0x2F, 0x32, 0x30, 0x31, 0x39, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type0.as_slice());
        let test_struct = SMBiosInformation::new(&parts);

        assert_eq!(test_struct.vendor(), Some("LENOVO".to_string()));
        assert_eq!(test_struct.version(), Some("S03KT33A".to_string()));
        assert_eq!(test_struct.starting_address_segment(), Some(61440));
        assert_eq!(test_struct.release_date(), Some("08/06/2019".to_string()));
        assert_eq!(test_struct.rom_size(), Some(255));
        assert_eq!(test_struct.characteristics(), Some(1066113152));
        assert_eq!(test_struct.bios_vendor_reserved_characteristics(), Some(1));
        assert_eq!(
            test_struct.system_vendor_reserved_characteristics(),
            Some(17)
        );
        assert_eq!(test_struct.characteristics_extension0(), Some(3));
        assert_eq!(test_struct.characteristics_extension1(), Some(13));
        assert_eq!(test_struct.system_bios_major_release(), Some(0));
        assert_eq!(test_struct.system_bios_minor_release(), Some(33));
        assert_eq!(test_struct.e_c_firmware_major_release(), Some(17));
        assert_eq!(test_struct.e_c_firmware_minor_release(), Some(45));

        // 2.4 to 3.0 BIOS Information does not include _extended_rom_size()_ or
        // fields beyond.
        assert!(test_struct.extended_rom_size().is_none());
    }
}
