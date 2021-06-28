use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use core::ops::Deref;

/// # Built-in Pointing Device (Type 21)
///
/// This structure describes the attributes of the built-in pointing device for the system.
/// Details are provided in Table 87.
/// The presence of this structure does not imply that the built-in pointing device is active
/// for the system’s use.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.5.0 (DSP0134)
/// Document Date: 2021-09-15
pub struct SMBiosBuiltInPointingDevice<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosBuiltInPointingDevice<'a> {
    const STRUCT_TYPE: u8 = 21u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosBuiltInPointingDevice<'a> {
    /// Type of pointing device.
    pub fn device_type(&self) -> Option<PointingDeviceTypeData> {
        self.parts
            .get_field_byte(0x04)
            .map(|raw| PointingDeviceTypeData::from(raw))
    }

    /// Interface type for the pointing device.
    pub fn interface(&self) -> Option<PointingDeviceInterfaceData> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| PointingDeviceInterfaceData::from(raw))
    }

    /// Number of buttons on the pointing device.
    /// If the device has 3 buttons, the field value is 3.
    pub fn number_of_buttons(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }
}

impl fmt::Debug for SMBiosBuiltInPointingDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosBuiltInPointingDevice<'_>>())
            .field("header", &self.parts.header)
            .field("device_type", &self.device_type())
            .field("interface", &self.interface())
            .field("number_of_buttons", &self.number_of_buttons())
            .finish()
    }
}

impl Serialize for SMBiosBuiltInPointingDevice<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosBuiltInPointingDevice", 4)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("device_type", &self.device_type())?;
        state.serialize_field("interface", &self.interface())?;
        state.serialize_field("number_of_buttons", &self.number_of_buttons())?;
        state.end()
    }
}

/// # Built-in Pointing Device Type Data
pub struct PointingDeviceTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [PointingDeviceType] value
    pub value: PointingDeviceType,
}

impl fmt::Debug for PointingDeviceTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<PointingDeviceTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for PointingDeviceTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PointingDeviceTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for PointingDeviceTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            PointingDeviceType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for PointingDeviceTypeData {
    type Target = PointingDeviceType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Built-in Pointing Device Type
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PointingDeviceType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Mouse
    Mouse,
    /// Track Ball
    TrackBall,
    /// Track Point
    TrackPoint,
    /// Glide Point
    GlidePoint,
    /// Touch Pad
    TouchPad,
    /// Touch Screen
    TouchScreen,
    /// Optical Sensor
    OpticalSensor,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for PointingDeviceTypeData {
    fn from(raw: u8) -> Self {
        PointingDeviceTypeData {
            value: match raw {
                0x01 => PointingDeviceType::Other,
                0x02 => PointingDeviceType::Unknown,
                0x03 => PointingDeviceType::Mouse,
                0x04 => PointingDeviceType::TrackBall,
                0x05 => PointingDeviceType::TrackPoint,
                0x06 => PointingDeviceType::GlidePoint,
                0x07 => PointingDeviceType::TouchPad,
                0x08 => PointingDeviceType::TouchScreen,
                0x09 => PointingDeviceType::OpticalSensor,
                _ => PointingDeviceType::None,
            },
            raw,
        }
    }
}

/// # Built-in Pointing Device Interface Data
pub struct PointingDeviceInterfaceData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [PointingDeviceInterface] value
    pub value: PointingDeviceInterface,
}

impl fmt::Debug for PointingDeviceInterfaceData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<PointingDeviceInterfaceData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for PointingDeviceInterfaceData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PointingDeviceInterfaceData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for PointingDeviceInterfaceData {
    type Target = PointingDeviceInterface;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Built-in Pointing Device Interface
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PointingDeviceInterface {
    /// Other field
    Other,
    /// Unknown
    Unknown,
    /// Serial
    Serial,
    /// PS/2
    PS2,
    /// Infrared
    Infrared,
    /// HP-HIL
    HpHil,
    /// Bus mouse
    BusMouse,
    /// ADB (Apple Desktop Bus)
    Adb,
    /// Bus mouse DB-9
    BusMouseDB9,
    /// Bus mouse micro-DIN
    BusMouseMicroDin,
    /// USB
    USB,
    /// I2C
    ///
    /// Available in version 3.5.0 and later.
    I2C,
    /// SPI
    ///
    /// Available in version 3.5.0 and later.
    SPI,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for PointingDeviceInterfaceData {
    fn from(raw: u8) -> Self {
        PointingDeviceInterfaceData {
            value: match raw {
                0x01 => PointingDeviceInterface::Other,
                0x02 => PointingDeviceInterface::Unknown,
                0x03 => PointingDeviceInterface::Serial,
                0x04 => PointingDeviceInterface::PS2,
                0x05 => PointingDeviceInterface::Infrared,
                0x06 => PointingDeviceInterface::HpHil,
                0x07 => PointingDeviceInterface::BusMouse,
                0x08 => PointingDeviceInterface::Adb,
                0xA0 => PointingDeviceInterface::BusMouseDB9,
                0xA1 => PointingDeviceInterface::BusMouseMicroDin,
                0xA2 => PointingDeviceInterface::USB,
                0xA3 => PointingDeviceInterface::I2C,
                0xA4 => PointingDeviceInterface::SPI,
                _ => PointingDeviceInterface::None,
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
        let struct_type21 = vec![0x15, 0x07, 0x31, 0x00, 0x05, 0x04, 0x03, 0x00, 0x00];

        let parts = UndefinedStruct::new(&struct_type21);
        let test_struct = SMBiosBuiltInPointingDevice::new(&parts);

        assert_eq!(
            *test_struct.device_type().unwrap(),
            PointingDeviceType::TrackPoint
        );
        assert_eq!(
            *test_struct.interface().unwrap(),
            PointingDeviceInterface::PS2
        );
        assert_eq!(test_struct.number_of_buttons(), Some(3));
    }
}
