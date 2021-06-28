use crate::SMBiosStruct;
use crate::{strings::*, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, ops::Deref, any};
use alloc::string::String;

/// # Out-of-Band Remote Access (Type 30)
///
/// This structure describes the attributes and policy settings of a hardware facility that may be used to gain
/// remote access to a hardware system when the operating system is not available due to power-down
/// status, hardware failures, or boot failures.
///
/// NOTE: This structure type was added in version 2.2 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOutOfBandRemoteAccess<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosOutOfBandRemoteAccess<'a> {
    const STRUCT_TYPE: u8 = 30u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosOutOfBandRemoteAccess<'a> {
    ///  The manufacturer of the out-of-band access facility
    pub fn manufacturer_name(&self) -> SMBiosString {
        self.parts.get_field_string(0x04)
    }

    /// Current remote-access connections (bit field)
    pub fn connections(&self) -> Option<Connections> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| Connections::from(raw))
    }
}

/// # Connections
#[derive(PartialEq, Eq)]
pub struct Connections {
    /// Raw value
    pub raw: u8,
}

impl Deref for Connections {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u8> for Connections {
    fn from(raw: u8) -> Self {
        Connections { raw }
    }
}

impl Connections {
    /// Inbound Connection Enabled (Bit 0)
    ///
    /// Identifies whether (1) or not (0) the facility is
    /// allowed to initiate outbound connections to receive
    /// incoming connections for the purpose of remote
    /// operations or problem management
    pub fn inbound_connection_enabled(&self) -> bool {
        self.raw & 0x01 == 0x01
    }

    /// Outbound Connection Enabled (Bit 1)
    ///
    /// Identifies whether (1) or not (0) the facility is
    /// allowed to initiate outbound connections to contact
    /// an alert management facility when critical conditions
    /// occur
    pub fn outbound_connection_enabled(&self) -> bool {
        self.raw & 0x02 == 0x02
    }
}

impl fmt::Debug for Connections {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<Connections>())
            .field("raw", &self.raw)
            .field(
                "inbound_connection_enabled",
                &self.inbound_connection_enabled(),
            )
            .field(
                "outbound_connection_enabled",
                &self.outbound_connection_enabled(),
            )
            .finish()
    }
}

impl Serialize for Connections {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Connections", 3)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field(
            "inbound_connection_enabled",
            &self.inbound_connection_enabled(),
        )?;
        state.serialize_field(
            "outbound_connection_enabled",
            &self.outbound_connection_enabled(),
        )?;
        state.end()
    }
}

impl fmt::Debug for SMBiosOutOfBandRemoteAccess<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosOutOfBandRemoteAccess<'_>>())
            .field("header", &self.parts.header)
            .field("manufacturer_name", &self.manufacturer_name())
            .field("connections", &self.connections())
            .finish()
    }
}

impl Serialize for SMBiosOutOfBandRemoteAccess<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosOutOfBandRemoteAccess", 3)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("manufacturer_name", &self.manufacturer_name())?;
        state.serialize_field("connections", &self.connections())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type41 = vec![
            30, 0x06, 0x3B, 0x00, 0x01, 0x03, 0x69, 0x6A, 0x6B, 0x6C, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type41);
        let test_struct = SMBiosOutOfBandRemoteAccess::new(&parts);

        assert_eq!(
            test_struct.manufacturer_name().to_string(),
            "ijkl".to_string()
        );

        let connections = test_struct.connections().unwrap();
        assert!(connections.inbound_connection_enabled());
        assert!(connections.outbound_connection_enabled());
        assert_eq!(connections.raw, 0x03);
    }
}
