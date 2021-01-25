use crate::*;

/// # Memory Module Information (Type 6, Obsolete)
///
/// One Memory Module Information structure is included for each memory-module socket in the system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryModuleInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryModuleInformation<'a> {
    const STRUCT_TYPE: u8 = 6u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryModuleInformation<'a> {
    /// Socket reference designation
    /// EXAMPLE: ‘J202’,0
    pub fn socket_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Each nibble indicates a bank (RAS#) connection; 0xF
    /// means no connection.
    /// EXAMPLE: If banks 1 & 3 (RAS# 1 & 3) were connected to a
    /// SIMM socket the byte for that socket would be 13h. If only bank 2
    /// (RAS 2) were connected, the byte for that socket would be 2Fh.
    pub fn bank_connections(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Speed of the memory module, in ns (for example, 70d for
    /// a 70ns module)
    /// If the speed is unknown, the field is set to 0.
    pub fn current_speed(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Bit field for the current memory type
    pub fn current_memory_type(&self) -> Option<u16> {
        self.parts.get_field_word(0x07)
    }

    /// Installed size
    pub fn installed_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    /// Enabled size
    pub fn enabled_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0A)
    }

    /// Bit fields error status
    pub fn error_status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0B)
    }
}

impl fmt::Debug for SMBiosMemoryModuleInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryModuleInformation>())
            .field("header", &self.parts.header)
            .field("socket_designation", &self.socket_designation())
            .field("bank_connections", &self.bank_connections())
            .field("current_speed", &self.current_speed())
            .field("current_memory_type", &self.current_memory_type())
            .field("installed_size", &self.installed_size())
            .field("enabled_size", &self.enabled_size())
            .field("error_status", &self.error_status())
            .finish()
    }
}
