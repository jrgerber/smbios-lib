use crate::core::{strings::*, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use core::ops::Deref;

/// # Management Device (Type 34)
///
/// The information in this structure defines the attributes of a Management Device.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosManagementDevice<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementDevice<'a> {
    const STRUCT_TYPE: u8 = 34u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosManagementDevice<'a> {
    /// Additional descriptive information about the device or its location
    pub fn description(&self) -> SMBiosString {
        self.parts.get_field_string(0x04)
    }

    /// Device's type
    pub fn device_type(&self) -> Option<ManagementDeviceTypeData> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| ManagementDeviceTypeData::from(raw))
    }

    /// Device's address
    pub fn address(&self) -> Option<u32> {
        self.parts.get_field_dword(0x06)
    }

    /// Type of addressing used to access the device
    pub fn address_type(&self) -> Option<ManagementDeviceAddressTypeData> {
        self.parts
            .get_field_byte(0x0A)
            .map(|raw| ManagementDeviceAddressTypeData::from(raw))
    }
}

impl fmt::Debug for SMBiosManagementDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosManagementDevice<'_>>())
            .field("header", &self.parts.header)
            .field("description", &self.description())
            .field("device_type", &self.device_type())
            .field("address", &self.address())
            .field("address_type", &self.address_type())
            .finish()
    }
}

impl Serialize for SMBiosManagementDevice<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosManagementDevice", 5)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("device_type", &self.device_type())?;
        state.serialize_field("address", &self.address())?;
        state.serialize_field("address_type", &self.address_type())?;
        state.end()
    }
}

/// # Management Device - Type Data
pub struct ManagementDeviceTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ManagementDeviceType] value
    pub value: ManagementDeviceType,
}

impl fmt::Debug for ManagementDeviceTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<ManagementDeviceTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ManagementDeviceTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ManagementDeviceTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ManagementDeviceTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ManagementDeviceType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ManagementDeviceTypeData {
    type Target = ManagementDeviceType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Management Device - Type
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ManagementDeviceType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// National Semiconductor LM75
    NationalSemiconductorLM75,
    /// National Semiconductor LM78
    NationalSemiconductorLM78,
    /// National Semiconductor LM79
    NationalSemiconductorLM79,
    /// National Semiconductor LM80
    NationalSemiconductorLM80,
    /// National Semiconductor LM81
    NationalSemiconductorLM81,
    /// Analog Devices ADM9240
    AnalogDevicesADM9240,
    /// Dallas Semiconductor DS1780
    DallasSemiconductorDS1780,
    /// Maxim 1617
    Maxim1617,
    /// Genesys GL518SM
    GenesysGL518SM,
    /// Winbond W83781D
    WinbondW83781D,
    /// Holtek HT82H791
    HoltekHT82H791,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ManagementDeviceTypeData {
    fn from(raw: u8) -> Self {
        ManagementDeviceTypeData {
            value: match raw {
                0x01 => ManagementDeviceType::Other,
                0x02 => ManagementDeviceType::Unknown,
                0x03 => ManagementDeviceType::NationalSemiconductorLM75,
                0x04 => ManagementDeviceType::NationalSemiconductorLM78,
                0x05 => ManagementDeviceType::NationalSemiconductorLM79,
                0x06 => ManagementDeviceType::NationalSemiconductorLM80,
                0x07 => ManagementDeviceType::NationalSemiconductorLM81,
                0x08 => ManagementDeviceType::AnalogDevicesADM9240,
                0x09 => ManagementDeviceType::DallasSemiconductorDS1780,
                0x0A => ManagementDeviceType::Maxim1617,
                0x0B => ManagementDeviceType::GenesysGL518SM,
                0x0C => ManagementDeviceType::WinbondW83781D,
                0x0D => ManagementDeviceType::HoltekHT82H791,
                _ => ManagementDeviceType::None,
            },
            raw,
        }
    }
}

/// # Management Device — Address Type Data
pub struct ManagementDeviceAddressTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [ManagementDeviceAddressType] value
    pub value: ManagementDeviceAddressType,
}

impl fmt::Debug for ManagementDeviceAddressTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<ManagementDeviceAddressTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for ManagementDeviceAddressTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ManagementDeviceAddressTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for ManagementDeviceAddressTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ManagementDeviceAddressType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for ManagementDeviceAddressTypeData {
    type Target = ManagementDeviceAddressType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Management Device — Address Type
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum ManagementDeviceAddressType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// I/O Port
    IOPort,
    /// Memory
    Memory,
    /// SM Bus
    SMBus,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for ManagementDeviceAddressTypeData {
    fn from(raw: u8) -> Self {
        ManagementDeviceAddressTypeData {
            value: match raw {
                0x01 => ManagementDeviceAddressType::Other,
                0x02 => ManagementDeviceAddressType::Unknown,
                0x03 => ManagementDeviceAddressType::IOPort,
                0x04 => ManagementDeviceAddressType::Memory,
                0x05 => ManagementDeviceAddressType::SMBus,
                _ => ManagementDeviceAddressType::None,
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
        let struct_type34 = vec![
            0x22, 0x0B, 0x26, 0x00, 0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x03, 0x4C, 0x4D, 0x37,
            0x38, 0x2D, 0x31, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type34);
        let test_struct = SMBiosManagementDevice::new(&parts);

        assert_eq!(test_struct.description().to_string(), "LM78-1".to_string());
        assert_eq!(
            *test_struct.device_type().unwrap(),
            ManagementDeviceType::NationalSemiconductorLM78
        );
        assert_eq!(test_struct.address(), Some(0));
        assert_eq!(
            *test_struct.address_type().unwrap(),
            ManagementDeviceAddressType::IOPort
        );
    }
}
