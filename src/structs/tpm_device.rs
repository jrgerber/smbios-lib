use super::*;

/// # TPM Device (Type 43)
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
    /// Vendor Id
    ///
    /// Specified as four ASCII characters, as defined by TCG
    /// Vendor ID (see CAP_VID in TCG Vendor ID Registry).
    ///
    /// For example:
    /// Vendor ID string of "ABC" = (41 42 43 00)
    /// Vendor ID string of "ABCD" = (41 42 43 44)
    pub fn vendor_id(&self) -> Option<u32> {
        self.parts.get_field_dword(0x04)
    }

    /// Major spec version
    ///
    /// Major TPM version supported by the TPM device. For
    /// example, the value is 01h for TPM v1.2 and is 02h for
    /// TPM v2.0.
    pub fn major_spec_version(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }

    /// Minor spec version
    ///
    /// Minor TPM version supported by the TPM device. For
    /// example, the value is 02h for TPM v1.2 and is 00h for
    /// TPM v2.0.
    pub fn minor_spec_version(&self) -> Option<u8> {
        self.parts.get_field_byte(0x09)
    }

    /// Firmware version 1
    ///
    /// For Major Spec Version 01h, this field contains the
    /// TPM_VERSION structure defined in the TPM Main
    /// Specification, Part 2, Section 5.3.
    ///
    /// For Major Spec Version 02h, this field contains the
    /// most significant 32 bits of a TPM vendor-specific value
    /// for firmware version (see
    /// TPM_PT_FIRMWARE_VERSION_1 in TPM Structures
    /// specification).
    pub fn firmware_version_1(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0A)
    }

    /// Firmware version 2
    ///
    /// For Major Spec Version 01h, this field contains 00h.
    ///
    /// For Major Spec Version 02h, this field contains the
    /// least significant 32 bits of a TPM vendor-specific value
    /// for firmware version (see
    /// TPM_PT_FIRMWARE_VERSION_2 in TPM Structures
    /// specification).
    pub fn firmware_version_2(&self) -> Option<u32> {
        self.parts.get_field_dword(0x0E)
    }

    /// Description
    ///
    /// Descriptive information of the TPM device.
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x12)
    }

    /// Characteristics
    ///
    /// TPM device characteristics information.
    pub fn characteristics(&self) -> Option<u64> {
        self.parts.get_field_qword(0x13)
    }

    /// OEM defined
    ///
    /// OEM- or BIOS vendor-specific information
    pub fn oem_defined(&self) -> Option<u32> {
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
