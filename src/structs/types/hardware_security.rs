use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};

/// # Hardware Security (Type 24)
///
/// This structure describes the system-wide hardware security settings.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosHardwareSecurity<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosHardwareSecurity<'a> {
    const STRUCT_TYPE: u8 = 24u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosHardwareSecurity<'a> {
    /// Bit field that identifies the password and reset status for the system
    pub fn hardware_security_settings(&self) -> Option<HardwareSecuritySettings> {
        self.parts
            .get_field_byte(0x4)
            .map(|raw| HardwareSecuritySettings::from(raw))
    }
}

impl fmt::Debug for SMBiosHardwareSecurity<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosHardwareSecurity<'_>>())
            .field("header", &self.parts.header)
            .field(
                "hardware_security_settings",
                &self.hardware_security_settings(),
            )
            .finish()
    }
}

impl Serialize for SMBiosHardwareSecurity<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosHardwareSecurity", 2)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field(
            "hardware_security_settings",
            &self.hardware_security_settings(),
        )?;
        state.end()
    }
}

/// # Hardware Security Settings
#[derive(PartialEq, Eq)]
pub struct HardwareSecuritySettings {
    /// Raw value
    pub raw: u8,
    /// Power-on Password Status
    pub power_on_password_status: HardwareSecurityStatus,
    /// Keyboard Password Status
    pub keyboard_password_status: HardwareSecurityStatus,
    /// Administrator Password Status
    pub administrator_password_status: HardwareSecurityStatus,
    /// Front Panel Reset Status
    pub front_panel_reset_status: HardwareSecurityStatus,
}

impl fmt::Debug for HardwareSecuritySettings {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<HardwareSecuritySettings>())
            .field("raw", &self.raw)
            .field("power_on_password_status", &self.power_on_password_status)
            .field("keyboard_password_status", &self.keyboard_password_status)
            .field(
                "administrator_password_status",
                &self.administrator_password_status,
            )
            .field("front_panel_reset_status", &self.front_panel_reset_status)
            .finish()
    }
}

impl Serialize for HardwareSecuritySettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HardwareSecuritySettings", 5)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("power_on_password_status", &self.power_on_password_status)?;
        state.serialize_field("keyboard_password_status", &self.keyboard_password_status)?;
        state.serialize_field(
            "administrator_password_status",
            &self.administrator_password_status,
        )?;
        state.serialize_field("front_panel_reset_status", &self.front_panel_reset_status)?;
        state.end()
    }
}

/// # Hardware Security Status
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum HardwareSecurityStatus {
    /// Disabled
    Disabled,
    /// Enabled
    Enabled,
    /// Not implemented
    NotImplemented,
    /// Unknown status
    Unknown,
}

impl From<u8> for HardwareSecuritySettings {
    fn from(raw: u8) -> Self {
        HardwareSecuritySettings {
            power_on_password_status: match raw & 0b11_000000 {
                0b00_000000 => HardwareSecurityStatus::Disabled,
                0b01_000000 => HardwareSecurityStatus::Enabled,
                0b10_000000 => HardwareSecurityStatus::NotImplemented,
                0b11_000000 => HardwareSecurityStatus::Unknown,
                _ => panic!("Impossible value"),
            },
            keyboard_password_status: match raw & 0b00_11_0000 {
                0b00_00_0000 => HardwareSecurityStatus::Disabled,
                0b00_01_0000 => HardwareSecurityStatus::Enabled,
                0b00_10_0000 => HardwareSecurityStatus::NotImplemented,
                0b00_11_0000 => HardwareSecurityStatus::Unknown,
                _ => panic!("Impossible value"),
            },
            administrator_password_status: match raw & 0b0000_11_00 {
                0b0000_00_00 => HardwareSecurityStatus::Disabled,
                0b0000_01_00 => HardwareSecurityStatus::Enabled,
                0b0000_10_00 => HardwareSecurityStatus::NotImplemented,
                0b0000_11_00 => HardwareSecurityStatus::Unknown,
                _ => panic!("Impossible value"),
            },
            front_panel_reset_status: match raw & 0b000000_11 {
                0b000000_00 => HardwareSecurityStatus::Disabled,
                0b000000_01 => HardwareSecurityStatus::Enabled,
                0b000000_10 => HardwareSecurityStatus::NotImplemented,
                0b000000_11 => HardwareSecurityStatus::Unknown,
                _ => panic!("Impossible value"),
            },
            raw,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type24 = vec![0x18, 0x05, 0x24, 0x00, 0x16, 0x00, 0x00];

        let parts = UndefinedStruct::new(&struct_type24);
        let test_struct = SMBiosHardwareSecurity::new(&parts);

        assert_eq!(
            test_struct.hardware_security_settings(),
            Some(HardwareSecuritySettings::from(22))
        );
    }
}
