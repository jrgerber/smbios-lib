use crate::core::{strings::*, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

/// # Portable Battery (Type 22)
///
/// This structure describes the attributes of the portable battery or batteries for the system. The structure
/// contains the static attributes for the group. Each structure describes a single battery pack’s attributes.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosPortableBattery<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosPortableBattery<'a> {
    const STRUCT_TYPE: u8 = 22u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosPortableBattery<'a> {
    /// Identifies the location of the battery
    pub fn location(&self) -> SMBiosString {
        self.parts.get_field_string(0x04)
    }

    /// Names the company that manufactured the battery
    pub fn manufacturer(&self) -> SMBiosString {
        self.parts.get_field_string(0x05)
    }

    /// The date on which the battery was manufactured.
    ///
    /// Version 2.2+ implementations that use a Smart
    /// Battery set this field to 0 (no string) to indicate
    /// that the SBDS Manufacture Date field contains
    /// the information.
    pub fn manufacture_date(&self) -> SMBiosString {
        self.parts.get_field_string(0x06)
    }

    /// The serial number for the battery
    ///
    /// Version 2.2+ implementations that use a Smart
    /// Battery set this field to 0 (no string) to indicate
    /// that the SBDS Serial Number field contains the
    /// information.
    pub fn serial_number(&self) -> SMBiosString {
        self.parts.get_field_string(0x07)
    }

    /// Names the battery device
    ///
    /// EXAMPLE: "DR-36"
    pub fn device_name(&self) -> SMBiosString {
        self.parts.get_field_string(0x08)
    }

    /// Identifies the battery chemistry
    ///
    /// Version 2.2+ implementations that use a Smart
    /// Battery set this field to 02h (Unknown) to
    /// indicate that the SBDS Device Chemistry field
    /// contains the information.
    pub fn device_chemistry(&self) -> Option<PortableBatteryDeviceChemistryData> {
        self.parts
            .get_field_byte(0x09)
            .map(|raw| PortableBatteryDeviceChemistryData::from(raw))
    }

    /// Design capacity of the battery in mWatt-hours
    ///
    /// If the value is unknown, the field contains 0.
    ///
    /// For version 2.2+ implementations, this value is
    /// multiplied by the 'design_capacity_multiplier' to
    /// produce the actual value.
    pub fn design_capacity(&self) -> Option<PortableBatteryDesignCapacity> {
        self.parts
            .get_field_word(0x0A)
            .map(|raw| PortableBatteryDesignCapacity::from(raw))
    }

    /// Design voltage of the battery in mVolts
    ///
    /// If the value is unknown, the field contains 0.
    pub fn design_voltage(&self) -> Option<PortableBatteryDesignVoltage> {
        self.parts
            .get_field_word(0x0C)
            .map(|raw| PortableBatteryDesignVoltage::from(raw))
    }

    /// Contains the Smart Battery Data Specification version number
    /// supported by this battery
    ///
    /// If the battery does not support the function, no
    /// string is supplied.
    pub fn sbds_version_number(&self) -> SMBiosString {
        self.parts.get_field_string(0x0E)
    }

    /// Maximum error (as a percentage in the range 0
    /// to 100) in the Watt-hour data reported by the
    /// battery, indicating an upper bound on how much
    /// additional energy the battery might have above
    /// the energy it reports having
    ///
    /// If the value is unknown, the field contains FFh.
    pub fn maximum_error_in_battery_data(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    /// 16-bit value that identifies the battery’s serial
    /// number
    ///
    /// This value, when combined with the
    /// Manufacturer, Device Name, and Manufacture
    /// Date, uniquely identifies the battery. The Serial
    /// Number field must be set to 0 (no string) for this
    /// field to be valid.
    pub fn sbds_serial_number(&self) -> Option<u16> {
        self.parts.get_field_word(0x10)
    }

    /// Date the cell pack was manufactured, in packed
    /// format
    pub fn sbds_manufacture_date(&self) -> Option<u16> {
        self.parts.get_field_word(0x12)
    }

    /// Number of the string that identifies the battery
    /// chemistry (for example, “PbAc”)
    /// The Device Chemistry field must be set to 02h
    /// (Unknown) for this field to be valid.
    pub fn sbds_device_chemistry(&self) -> SMBiosString {
        self.parts.get_field_string(0x14)
    }

    /// Multiplication factor of the Design Capacity
    /// value, which assures that the mWatt hours value
    /// does not overflow for SBDS implementations
    ///
    /// The multiplier default is 1, SBDS
    /// implementations use the value 10 to correspond
    /// to the data as returned from the SBDS Function
    /// 18h.
    pub fn design_capacity_multiplier(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    /// Contains OEM- or BIOS vendor-specific
    /// information
    pub fn oem_specific(&self) -> Option<u32> {
        self.parts.get_field_dword(0x16)
    }
}

impl fmt::Debug for SMBiosPortableBattery<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPortableBattery<'_>>())
            .field("header", &self.parts.header)
            .field("location", &self.location())
            .field("manufacturer", &self.manufacturer())
            .field("manufacture_date", &self.manufacture_date())
            .field("serial_number", &self.serial_number())
            .field("device_name", &self.device_name())
            .field("device_chemistry", &self.device_chemistry())
            .field("design_capacity", &self.design_capacity())
            .field("design_voltage", &self.design_voltage())
            .field("sbds_version_number", &self.sbds_version_number())
            .field(
                "maximum_error_in_battery_data",
                &self.maximum_error_in_battery_data(),
            )
            .field("sbds_serial_number", &self.sbds_serial_number())
            .field("sbds_manufacture_date", &self.sbds_manufacture_date())
            .field("sbds_device_chemistry", &self.sbds_device_chemistry())
            .field(
                "design_capacity_multiplier",
                &self.design_capacity_multiplier(),
            )
            .field("oem_specific", &self.oem_specific())
            .finish()
    }
}

impl Serialize for SMBiosPortableBattery<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosPortableBattery", 16)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("location", &self.location())?;
        state.serialize_field("manufacturer", &self.manufacturer())?;
        state.serialize_field("manufacture_date", &self.manufacture_date())?;
        state.serialize_field("serial_number", &self.serial_number())?;
        state.serialize_field("device_name", &self.device_name())?;
        state.serialize_field("device_chemistry", &self.device_chemistry())?;
        state.serialize_field("design_capacity", &self.design_capacity())?;
        state.serialize_field("design_voltage", &self.design_voltage())?;
        state.serialize_field("sbds_version_number", &self.sbds_version_number())?;
        state.serialize_field(
            "maximum_error_in_battery_data",
            &self.maximum_error_in_battery_data(),
        )?;
        state.serialize_field("sbds_serial_number", &self.sbds_serial_number())?;
        state.serialize_field("sbds_manufacture_date", &self.sbds_manufacture_date())?;
        state.serialize_field("sbds_device_chemistry", &self.sbds_device_chemistry())?;
        state.serialize_field(
            "design_capacity_multiplier",
            &self.design_capacity_multiplier(),
        )?;
        state.serialize_field("oem_specific", &self.oem_specific())?;
        state.end()
    }
}

/// # Portable Battery - Device Chemistry Data
pub struct PortableBatteryDeviceChemistryData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [PortableBatteryDeviceChemistry] value
    pub value: PortableBatteryDeviceChemistry,
}

impl fmt::Debug for PortableBatteryDeviceChemistryData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<PortableBatteryDeviceChemistryData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for PortableBatteryDeviceChemistryData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PortableBatteryDeviceChemistryData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Deref for PortableBatteryDeviceChemistryData {
    type Target = PortableBatteryDeviceChemistry;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Portable Battery - Device Chemistry
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PortableBatteryDeviceChemistry {
    /// Other
    Other,
    /// Unknown
    ///
    /// Version 2.2+ implementations that use a Smart Battery
    /// set this field to 02h (Unknown) to indicate that the
    /// 'sbds_device_chemistry' field contains the information.
    Unknown,
    /// Lead Acid
    LeadAcid,
    /// Nickel Cadmium
    NickelCadmium,
    /// Nickel metal hydride
    NickelMetalHydride,
    /// Lithium-ion
    LithiumIon,
    /// Zinc air
    ZincAir,
    /// Lithium Polymer
    LithiumPolymer,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for PortableBatteryDeviceChemistryData {
    fn from(raw: u8) -> Self {
        PortableBatteryDeviceChemistryData {
            value: match raw {
                0x01 => PortableBatteryDeviceChemistry::Other,
                0x02 => PortableBatteryDeviceChemistry::Unknown,
                0x03 => PortableBatteryDeviceChemistry::LeadAcid,
                0x04 => PortableBatteryDeviceChemistry::NickelCadmium,
                0x05 => PortableBatteryDeviceChemistry::NickelMetalHydride,
                0x06 => PortableBatteryDeviceChemistry::LithiumIon,
                0x07 => PortableBatteryDeviceChemistry::ZincAir,
                0x08 => PortableBatteryDeviceChemistry::LithiumPolymer,
                _ => PortableBatteryDeviceChemistry::None,
            },
            raw,
        }
    }
}

/// # Portable Battery - Design Capacity
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PortableBatteryDesignCapacity {
    /// Design capacity of the battery in mWatt-hours
    ///
    /// For version 2.2+ implementations, this value is
    /// multiplied by the 'design_capacity_multiplier' to
    /// produce the actual value.
    MilliWattHours(u16),
    /// Design capacity of the battery in mWatt-hours is unknown.
    Unknown,
}

impl From<u16> for PortableBatteryDesignCapacity {
    fn from(raw: u16) -> Self {
        match raw {
            0 => PortableBatteryDesignCapacity::Unknown,
            _ => PortableBatteryDesignCapacity::MilliWattHours(raw),
        }
    }
}

/// # Portable Battery - Design Voltage
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum PortableBatteryDesignVoltage {
    /// Design voltage of the battery in mVolts.
    MilliVolts(u16),
    /// Design voltage of the battery in mVolts is unknown.
    Unknown,
}

impl From<u16> for PortableBatteryDesignVoltage {
    fn from(raw: u16) -> Self {
        match raw {
            0 => PortableBatteryDesignVoltage::Unknown,
            _ => PortableBatteryDesignVoltage::MilliVolts(raw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type22 = vec![
            0x16, 0x1A, 0x2E, 0x00, 0x01, 0x02, 0x00, 0x00, 0x03, 0x02, 0xFB, 0x11, 0xD0, 0x39,
            0x04, 0xFF, 0xC7, 0x02, 0x7A, 0x42, 0x05, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x52, 0x65,
            0x61, 0x72, 0x00, 0x53, 0x4D, 0x50, 0x00, 0x34, 0x35, 0x4E, 0x31, 0x30, 0x37, 0x31,
            0x00, 0x30, 0x33, 0x2E, 0x30, 0x31, 0x00, 0x4C, 0x69, 0x50, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type22);
        let test_struct = SMBiosPortableBattery::new(&parts);

        assert_eq!(test_struct.location().to_string(), "Rear".to_string());
        assert_eq!(test_struct.manufacturer().to_string(), "SMP".to_string());
        assert_eq!(test_struct.manufacture_date().to_string(), "".to_string());
        assert_eq!(test_struct.serial_number().to_string(), "".to_string());
        assert_eq!(test_struct.device_name().to_string(), "45N1071".to_string());
        assert_eq!(
            *test_struct.device_chemistry().unwrap(),
            PortableBatteryDeviceChemistry::Unknown
        );
        match test_struct.design_capacity().unwrap() {
            PortableBatteryDesignCapacity::MilliWattHours(mwh) => assert_eq!(mwh, 4603),
            PortableBatteryDesignCapacity::Unknown => panic!("expected a value in mWH"),
        }
        match test_struct.design_voltage().unwrap() {
            PortableBatteryDesignVoltage::MilliVolts(mv) => assert_eq!(mv, 14800),
            PortableBatteryDesignVoltage::Unknown => panic!("expected a value in mWH"),
        }
        assert_eq!(
            test_struct.sbds_version_number().to_string(),
            "03.01".to_string()
        );
        assert_eq!(test_struct.maximum_error_in_battery_data(), Some(255));
        assert_eq!(test_struct.sbds_serial_number(), Some(711));
        assert_eq!(test_struct.sbds_manufacture_date(), Some(17018));
        assert_eq!(
            test_struct.sbds_device_chemistry().to_string(),
            "LiP".to_string()
        );
        assert_eq!(test_struct.design_capacity_multiplier(), Some(10));
        assert_eq!(test_struct.oem_specific(), Some(0));
    }
}
