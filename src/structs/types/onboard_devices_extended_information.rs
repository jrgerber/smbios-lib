use super::system_slot::{BusNumber, DeviceFunctionNumber, SegmentGroupNumber};
use crate::{OnBoardDeviceType, SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};
use alloc::string::String;

/// # Onboard Devices Extended Information (Type 41)
///
/// The information in this structure defines the attributes of devices that are onboard (soldered onto) a
/// system element, usually the baseboard.
///
/// In general, an entry in this table implies that the BIOS has some level of control over the enablement of
/// the associated device for use by the system.
///
/// NOTE: This structure replaces Onboard Device Information (Type 10) starting with version 2.6 of this specification.
/// BIOS providers can choose to implement both types to allow existing SMBIOS browsers to properly display
/// the system’s onboard devices information.
///  
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOnboardDevicesExtendedInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosOnboardDevicesExtendedInformation<'a> {
    const STRUCT_TYPE: u8 = 41u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosOnboardDevicesExtendedInformation<'a> {
    /// The onboard device reference designation
    pub fn reference_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x4)
    }

    /// Device type bit field and enum
    pub fn device_type(&self) -> Option<OnBoardDeviceType> {
        self.parts
            .get_field_byte(0x5)
            .map(|raw| OnBoardDeviceType::from(raw))
    }

    /// Device type instance
    pub fn device_type_instance(&self) -> Option<u8> {
        self.parts.get_field_byte(0x6)
    }

    /// Segment group number
    pub fn segment_group_number(&self) -> Option<SegmentGroupNumber> {
        self.parts
            .get_field_word(0x7)
            .map(|raw| SegmentGroupNumber::from(raw))
    }

    /// Bus number
    pub fn bus_number(&self) -> Option<BusNumber> {
        self.parts
            .get_field_byte(0x9)
            .map(|raw| BusNumber::from(raw))
    }

    /// Device/Function number
    pub fn device_function_number(&self) -> Option<DeviceFunctionNumber> {
        self.parts
            .get_field_byte(0xA)
            .map(|raw| DeviceFunctionNumber::from(raw))
    }
}

impl fmt::Debug for SMBiosOnboardDevicesExtendedInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<
            SMBiosOnboardDevicesExtendedInformation<'_>,
        >())
        .field("header", &self.parts.header)
        .field("reference_designation", &self.reference_designation())
        .field("device_type", &self.device_type())
        .field("device_type_instance", &self.device_type_instance())
        .field("segment_group_number", &self.segment_group_number())
        .field("bus_number", &self.bus_number())
        .field("device_function_number", &self.device_function_number())
        .finish()
    }
}

impl Serialize for SMBiosOnboardDevicesExtendedInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state =
            serializer.serialize_struct("SMBiosOnboardDevicesExtendedInformation", 7)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("reference_designation", &self.reference_designation())?;
        state.serialize_field("device_type", &self.device_type())?;
        state.serialize_field("device_type_instance", &self.device_type_instance())?;
        state.serialize_field("segment_group_number", &self.segment_group_number())?;
        state.serialize_field("bus_number", &self.bus_number())?;
        state.serialize_field("device_function_number", &self.device_function_number())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceStatus, SMBiosStruct, TypeOfDevice, UndefinedStruct};

    #[test]
    fn unit_test() {
        let struct_type41 = vec![
            0x29, 0x0B, 0x3B, 0x00, 0x01, 0x85, 0x01, 0x00, 0x00, 0x00, 0xFE, 0x69, 0x32, 0x31,
            0x39, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type41);
        let test_struct = SMBiosOnboardDevicesExtendedInformation::new(&parts);

        assert_eq!(
            test_struct.reference_designation(),
            Some("i219".to_string())
        );
        let device_type = test_struct.device_type().unwrap();
        assert_eq!(device_type.type_of_device(), TypeOfDevice::Ethernet);
        assert_eq!(device_type.status(), DeviceStatus::Enabled);
        assert_eq!(test_struct.device_type_instance(), Some(1));
        match test_struct.segment_group_number().unwrap() {
            SegmentGroupNumber::SingleSegment => (),
            SegmentGroupNumber::NotApplicable => panic!("expected SingleSegment"),
            SegmentGroupNumber::Number(_) => panic!("expected SingleSegment"),
        }
        match test_struct.bus_number().unwrap() {
            BusNumber::Number(number) => assert_eq!(number, 0),
            BusNumber::NotApplicable => panic!("expected Number"),
        }
        match test_struct.device_function_number().unwrap() {
            DeviceFunctionNumber::Number { device, function } => {
                assert_eq!(device, 30);
                assert_eq!(function, 6);
            }
            _ => panic!("expected device and function values"),
        }
    }
}
