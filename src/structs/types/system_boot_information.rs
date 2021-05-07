use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;

/// # System Boot Information (Type 32)
///
/// The client system firmware (for example, BIOS) communicates the
/// System Boot Status to the client’s Pre1864 boot Execution Environment
/// (PXE) boot image or OS-present management application through this
/// structure.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemBootInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemBootInformation<'a> {
    const STRUCT_TYPE: u8 = 32u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosSystemBootInformation<'a> {
    /// offset of boot_status field
    const BOOT_STATUS_OFFSET: usize = 0x0A;

    /// Boot status can be a size from 1 to 10
    const BOOT_STATUS_MAX_SIZE: usize = 0x0A;

    /// Status and Additional Data fields that identify the boot status
    pub fn boot_status_data(&self) -> Option<SystemBootStatusData<'_>> {
        // boot_status is from 1 to 10 bytes in length.  The entire structure must be at least 0xB in length
        // and boot_status starts at offset 0xA;
        // meaning, at least 1 byte of boot_status exists, but not more than 10 bytes total.
        let struct_length = self.parts.header.length() as usize;
        if struct_length < Self::BOOT_STATUS_OFFSET + 1 {
            return None;
        }

        let end_index: usize;
        if struct_length < Self::BOOT_STATUS_OFFSET + Self::BOOT_STATUS_MAX_SIZE {
            end_index = struct_length;
        } else {
            end_index = Self::BOOT_STATUS_OFFSET + Self::BOOT_STATUS_MAX_SIZE;
        }

        self.parts
            .get_field_data(Self::BOOT_STATUS_OFFSET, end_index)
            .map(|raw| SystemBootStatusData { raw })
    }
}

impl fmt::Debug for SMBiosSystemBootInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemBootInformation<'_>>())
            .field("header", &self.parts.header)
            .field("boot_status_data", &self.boot_status_data())
            .finish()
    }
}

impl Serialize for SMBiosSystemBootInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosSystemBootInformation", 2)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("boot_status_data", &self.boot_status_data())?;
        state.end()
    }
}

/// # Boot Status data of [SMBiosSystemBootInformation]
pub struct SystemBootStatusData<'a> {
    /// Raw data
    pub raw: &'a [u8],
}

impl<'a> SystemBootStatusData<'a> {
    /// System boot status
    pub fn system_boot_status(&self) -> SystemBootStatus {
        debug_assert!(self.raw.len() > 0);
        match self.raw[0] {
            0x00 => SystemBootStatus::NoErrors,
            0x01 => SystemBootStatus::NoBootableMedia,
            0x02 => SystemBootStatus::NormalOSFailedToLoad,
            0x03 => SystemBootStatus::FirmwareDetectedFailure,
            0x04 => SystemBootStatus::OSDetectedFailure,
            0x05 => SystemBootStatus::UserRequestedBoot,
            0x06 => SystemBootStatus::SystemSecurityViolation,
            0x07 => SystemBootStatus::PreviouslyRequestedImage,
            0x08 => SystemBootStatus::SystemWatchdogTimerExpired,
            _ => SystemBootStatus::None,
        }
    }
}

impl fmt::Debug for SystemBootStatusData<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemBootInformation<'_>>())
            .field("system_boot_status", &self.system_boot_status())
            .finish()
    }
}

impl Serialize for SystemBootStatusData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SystemBootStatusData", 1)?;
        state.serialize_field("system_boot_status", &self.system_boot_status())?;
        state.end()
    }
}

/// # System Boot Status
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum SystemBootStatus {
    /// No errors detected
    NoErrors,
    /// No bootable media
    NoBootableMedia,
    /// “normal” operating system failed to load
    NormalOSFailedToLoad,
    /// Firmware-detected hardware failure, including “unknown” failure types
    FirmwareDetectedFailure,
    /// Operating system-detected hardware failure
    /// For ACPI operating systems, the system firmware might set this reason code
    /// when the OS reports a boot failure through interfaces defined in the Simple
    /// Boot Flag Specification.
    OSDetectedFailure,
    /// User-requested boot, usually through a keystroke
    UserRequestedBoot,
    /// System security violation
    SystemSecurityViolation,
    /// Previously-requested image
    /// This reason code allows coordination between OS-present software and the
    /// OS-absent environment. For example, an OS-present application might
    /// enable (through a platform-specific interface) the system to boot to the PXE
    /// and request a specific boot-image.
    PreviouslyRequestedImage,
    /// System watchdog timer expired, causing the system to reboot
    SystemWatchdogTimerExpired,
    /// A value unknown to this standard, check the raw value
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        // test a normal structure with a 10 byte boot status field with "NoErrors"
        let struct_type32 = vec![
            0x20, 0x14, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type32);
        let test_struct = SMBiosSystemBootInformation::new(&parts);

        let boot_status_data = test_struct.boot_status_data().unwrap();
        assert_eq!(
            boot_status_data.raw,
            &[0x00u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] as &[u8]
        );
        assert_eq!(
            boot_status_data.system_boot_status(),
            SystemBootStatus::NoErrors
        );

        // test a structure with a 2 byte boot status field with "NormalOSFailedToLoad"
        let struct_type32 = vec![
            0x20, 0x0C, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01,
        ];

        let parts = UndefinedStruct::new(&struct_type32);
        let test_struct = SMBiosSystemBootInformation::new(&parts);

        let boot_status_data = test_struct.boot_status_data().unwrap();
        assert_eq!(boot_status_data.raw, &[0x02u8, 0x01] as &[u8]);
        assert_eq!(
            boot_status_data.system_boot_status(),
            SystemBootStatus::NormalOSFailedToLoad
        );

        // test a structure with a 2 byte boot status field but an incorrect header length
        // extending beyond the end of the structure
        let struct_type32 = vec![
            0x20, 0x0F, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01,
        ];

        let parts = UndefinedStruct::new(&struct_type32);
        let test_struct = SMBiosSystemBootInformation::new(&parts);

        assert!(test_struct.boot_status_data().is_none());
    }
}
