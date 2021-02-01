use std::{array::TryFromSliceError, convert::TryFrom};

use crate::*;

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
    pub fn vendor_id(&self) -> Option<VendorId> {
        self.parts
            .get_field_data(0x04, 0x08)
            .and_then(|array| Some(VendorId::try_from(array).expect("Vendor Id is 4 bytes")))
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
    pub fn characteristics(&self) -> Option<TpmDeviceCharacteristics> {
        self.parts
            .get_field_qword(0x13)
            .and_then(|raw| Some(TpmDeviceCharacteristics::from(raw)))
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

/// # Vendor Id
///
/// Specified as four ASCII characters,
/// as defined by TCG Vendor ID
/// (see CAP_VID in TCG Vendor ID Registry)
#[derive(PartialEq, Eq)]
pub struct VendorId<'a> {
    /// Raw array
    ///
    /// Example: Vendor Id string of "ABC" = (41 42 43 00)
    pub array: &'a [u8; 4],
}

impl<'a> TryFrom<&'a [u8]> for VendorId<'a> {
    type Error = TryFromSliceError;

    fn try_from(raw: &'a [u8]) -> Result<Self, Self::Error> {
        <&[u8; 4]>::try_from(raw).and_then(|array| Ok(VendorId { array }))
    }
}

impl<'a> fmt::Debug for VendorId<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<VendorId>())
            .field("array", &self.array)
            .field("string", &String::from_utf8_lossy(self.array))
            .finish()
    }
}

/// # TPM Device Characteristics
#[derive(PartialEq, Eq)]
pub struct TpmDeviceCharacteristics {
    raw: u64,
}

impl Deref for TpmDeviceCharacteristics {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u64> for TpmDeviceCharacteristics {
    fn from(raw: u64) -> Self {
        TpmDeviceCharacteristics { raw }
    }
}

impl TpmDeviceCharacteristics {
    /// Bit 0 - reserved
    pub fn reserved_0(&self) -> bool {
        self.raw & 0x0000000000000001 == 0x0000000000000001
    }

    /// Bit 1 - reserved
    pub fn reserved_1(&self) -> bool {
        self.raw & 0x0000000000000002 == 0x0000000000000002
    }

    /// Bit 2 - TPM Device Characteristics are not supported.
    pub fn not_supported(&self) -> bool {
        self.raw & 0x0000000000000004 == 0x0000000000000004
    }

    /// Bit 3 - Family configurable via firmware update; for example, switching between TPM 1.2
    pub fn family_configurable_via_firmware(&self) -> bool {
        self.raw & 0x0000000000000008 == 0x0000000000000008
    }

    /// Bit 4 - Family configurable via platform software support, such as BIOS Setup; for example,
    pub fn family_configurable_via_software(&self) -> bool {
        self.raw & 0x0000000000000010 == 0x0000000000000010
    }

    /// Bit 5 - Family configurable via OEM proprietary mechanism; for example, switching between TPM 1.2 and TPM 2.0.
    pub fn family_configurable_via_oem(&self) -> bool {
        self.raw & 0x0000000000000020 == 0x0000000000000020
    }
}

impl fmt::Debug for TpmDeviceCharacteristics {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<TpmDeviceCharacteristics>())
            .field("raw", &self.raw)
            .field("reserved_0", &self.reserved_0())
            .field("reserved_1", &self.reserved_1())
            .field("not_supported", &self.not_supported())
            .field(
                "family_configurable_via_firmware",
                &self.family_configurable_via_firmware(),
            )
            .field(
                "family_configurable_via_software",
                &self.family_configurable_via_software(),
            )
            .field(
                "family_configurable_via_oem",
                &self.family_configurable_via_oem(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type43 = vec![
            0x2B, 0x1F, 0x3C, 0x00, 0x00, 0x58, 0x46, 0x49, 0x02, 0x00, 0x3E, 0x00, 0x05, 0x00,
            0x00, 0x36, 0x0C, 0x00, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x54, 0x50, 0x4D, 0x20, 0x32, 0x2E, 0x30, 0x00, 0x49, 0x4E, 0x46,
            0x49, 0x4E, 0x45, 0x4F, 0x4E, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type43.as_slice());
        let test_struct = SMBiosTpmDevice::new(&parts);

        println!("{:?}", test_struct);
        assert_eq!(
            test_struct.vendor_id().unwrap().array,
            &[0, b'X', b'F', b'I']
        );
        assert_eq!(test_struct.major_spec_version(), Some(2));
        assert_eq!(test_struct.minor_spec_version(), Some(0));
        assert_eq!(test_struct.firmware_version_1(), Some(327742));
        assert_eq!(test_struct.firmware_version_2(), Some(800256));
        assert_eq!(test_struct.description(), Some("INFINEON".to_string()));
        assert_eq!(
            test_struct.characteristics(),
            Some(TpmDeviceCharacteristics::from(16))
        );
        assert_eq!(test_struct.oem_defined(), Some(0));
    }
}
