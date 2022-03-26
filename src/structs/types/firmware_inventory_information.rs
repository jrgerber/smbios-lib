use crate::core::{Handle, SMBiosStringError, UndefinedStruct};
use crate::SMBiosStruct;
use serde::{ser::SerializeSeq, ser::SerializeStruct, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

/// # Firmware Inventory Information (Type 45)
///
/// The information in this structure defines an inventory of firmware components in the system. This can
/// include firmware components such as BIOS, BMC, as well as firmware for other devices in the system.
///
/// The information can be used by software to display the firmware inventory in a uniform manner. It can
/// also be used by a management controller, such as a BMC, for remote system management. This
/// structure is not intended to replace other standard programmatic interfaces for firmware updates.
///
/// One Type 45 structure is provided for each firmware component.
///
/// NOTE: This structure type was added in version 3.5 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.5.0 (DSP0134)
/// Document Date: 2021-09-15
pub struct SMBiosFirmwareInventoryInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosFirmwareInventoryInformation<'a> {
    const STRUCT_TYPE: u8 = 45u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosFirmwareInventoryInformation<'a> {
    /// Firmware Component Name
    ///
    /// EXAMPLE: 'BMC Firmware',0
    pub fn firmware_component_name(&self) -> Result<String, SMBiosStringError> {
        self.parts.get_field_string(0x04)
    }

    /// Firmware Version
    ///
    /// The format of this value is defined by _version_format_
    pub fn firmware_version(&self) -> Result<String, SMBiosStringError> {
        self.parts.get_field_string(0x05)
    }

    /// Version Format
    pub fn version_format(&self) -> Option<VersionFormatData> {
        self.parts
            .get_field_byte(0x06)
            .map(|raw| VersionFormatData::from(raw))
    }

    /// Firmware ID
    ///
    /// The format of this value is defined by _firmware_id_format_
    pub fn firmware_id(&self) -> Result<String, SMBiosStringError> {
        self.parts.get_field_string(0x07)
    }

    /// Firmware ID Format
    pub fn firmware_id_format(&self) -> Option<FirmwareIdFormatData> {
        self.parts
            .get_field_byte(0x08)
            .map(|raw| FirmwareIdFormatData::from(raw))
    }

    /// Release Date
    pub fn release_date(&self) -> Result<String, SMBiosStringError> {
        self.parts.get_field_string(0x09)
    }

    /// Manufacturer
    pub fn manufacturer(&self) -> Result<String, SMBiosStringError> {
        self.parts.get_field_string(0x0A)
    }

    /// Lowest Supported Firmware Version
    ///
    /// The format of this value is defined by _version_format_
    pub fn lowest_supported_firmware_version(&self) -> Result<String, SMBiosStringError> {
        self.parts.get_field_string(0x0B)
    }

    /// Image Size
    ///
    /// Size of the firmware image that is currently programmed
    /// in the device, in bytes. If the Firmware Image Size is
    /// unknown, the field is set to FirmwareImageSize::Unknown.
    pub fn image_size(&self) -> Option<FirmwareImageSize> {
        self.parts
            .get_field_qword(0x0C)
            .map(|raw| FirmwareImageSize::from(raw))
    }

    /// Firmware characteristics information.
    pub fn characteristics(&self) -> Option<FirmwareInventoryCharacteristics> {
        self.parts
            .get_field_word(0x14)
            .map(|raw| FirmwareInventoryCharacteristics::from(raw))
    }

    /// Firmware state information.
    pub fn state(&self) -> Option<FirmwareInventoryStateInformationData> {
        self.parts
            .get_field_byte(0x16)
            .map(|raw| FirmwareInventoryStateInformationData::from(raw))
    }

    /// Defines how many Associated Component Handles are associated with this firmware.
    pub fn number_of_associated_components(&self) -> Option<u8> {
        self.parts.get_field_byte(0x17)
    }

    /// Lists the SMBIOS structure handles that are associated
    /// with this firmware, if any.
    ///
    /// Value of _number_of_associated_components_ field (n) defines the count.
    ///
    /// NOTE: This list may contain zero or more handles to any
    /// SMBIOS structure that represents a device with a
    /// firmware component.
    pub fn associated_component_handle_iterator(&'a self) -> AssociatedComponentHandleIterator<'a> {
        AssociatedComponentHandleIterator::new(self)
    }
}

impl fmt::Debug for SMBiosFirmwareInventoryInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosFirmwareInventoryInformation<'_>>())
            .field("header", &self.parts.header)
            .field("firmware_component_name", &self.firmware_component_name())
            .field("firmware_version", &self.firmware_version())
            .field("version_format", &self.version_format())
            .field("firmware_id", &self.firmware_id())
            .field("firmware_id_format", &self.firmware_id_format())
            .field("release_date", &self.release_date())
            .field("manufacturer", &self.manufacturer())
            .field(
                "lowest_supported_firmware_version",
                &self.lowest_supported_firmware_version(),
            )
            .field("image_size", &self.image_size())
            .field("characteristics", &self.characteristics())
            .field("state", &self.state())
            .field(
                "number_of_associated_components",
                &self.number_of_associated_components(),
            )
            .field(
                "associated_component_handle_iterator",
                &self.associated_component_handle_iterator(),
            )
            .finish()
    }
}

impl Serialize for SMBiosFirmwareInventoryInformation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosFirmwareInventoryInformation", 12)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("firmware_component_name", &self.firmware_component_name())?;
        state.serialize_field("firmware_version", &self.firmware_version())?;
        state.serialize_field("version_format", &self.version_format())?;
        state.serialize_field("firmware_id", &self.firmware_id())?;
        state.serialize_field("firmware_id_format", &self.firmware_id_format())?;
        state.serialize_field("release_date", &self.release_date())?;
        state.serialize_field("manufacturer", &self.manufacturer())?;
        state.serialize_field(
            "lowest_supported_firmware_version",
            &self.lowest_supported_firmware_version(),
        )?;
        state.serialize_field("image_size", &self.image_size())?;
        state.serialize_field("characteristics", &self.characteristics())?;
        state.serialize_field("state", &self.state())?;
        state.serialize_field(
            "number_of_associated_components",
            &self.number_of_associated_components(),
        )?;
        state.serialize_field(
            "associated_component_handle_iterator",
            &self.associated_component_handle_iterator(),
        )?;
        state.end()
    }
}

/// # Size of Image in Bytes
#[derive(Serialize, Debug)]
pub enum FirmwareImageSize {
    /// Image Size is Unknown
    Unknown,
    /// Size of Image (bytes)
    Bytes(u64),
}

impl From<u64> for FirmwareImageSize {
    fn from(raw: u64) -> Self {
        match raw {
            0xFFFFFFFFFFFFFFFF => FirmwareImageSize::Unknown,
            _ => FirmwareImageSize::Bytes(raw),
        }
    }
}

/// # Version Format Data of [SMBiosFirmwareInventoryInformation].
pub struct VersionFormatData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [VersionFormat] value
    pub value: VersionFormat,
}

impl fmt::Debug for VersionFormatData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<VersionFormatData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for VersionFormatData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("VersionFormatData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for VersionFormatData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            VersionFormat::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for VersionFormatData {
    type Target = VersionFormat;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for VersionFormatData {
    fn from(raw: u8) -> Self {
        VersionFormatData {
            value: match raw {
                0x00 => VersionFormat::FreeForm,
                0x01 => VersionFormat::MajorMinor,
                0x02 => VersionFormat::HexidecimalString32,
                0x03 => VersionFormat::HexidecimalString64,
                _ => match raw & 0x80 {
                    0x80 => VersionFormat::VendorOemSpecific,
                    _ => VersionFormat::None,
                },
            },
            raw,
        }
    }
}

/// # Version Format of [SMBiosFirmwareInventoryInformation]
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum VersionFormat {
    /// The format is a free-form string that is implementation specific.
    ///
    /// EXAMPLE: '1.45.455b66-rev4',0
    FreeForm,
    /// The format is “MAJOR.MINOR”, where MAJOR and MINOR are decimal string representations of the
    /// numeric values of the major/minor version numbers.
    ///
    /// EXAMPLE: '1.45',0
    MajorMinor,
    /// The format is a hexadecimal string representation of the 32-bit numeric value of the version, in the
    /// format of "0xhhhhhhhh". Each h represents a hexadecimal digit (0-f).
    ///
    /// EXAMPLE: '0x0001002d',0
    HexidecimalString32,
    /// The format is a hexadecimal string representation of the 64-bit numeric value of the version, in the
    /// format of "0xhhhhhhhhhhhhhhhh". Each h represents a hexadecimal digit (0-f).
    ///
    /// EXAMPLE: '0x000000010000002d',0
    HexidecimalString64,
    /// BIOS Vendor/OEM-specific
    VendorOemSpecific,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Firmware Id Format Data of [SMBiosFirmwareInventoryInformation].
pub struct FirmwareIdFormatData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [FirmwareIdFormat] value
    pub value: FirmwareIdFormat,
}

impl fmt::Debug for FirmwareIdFormatData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<FirmwareIdFormatData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for FirmwareIdFormatData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FirmwareIdFormatData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for FirmwareIdFormatData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            FirmwareIdFormat::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for FirmwareIdFormatData {
    type Target = FirmwareIdFormat;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for FirmwareIdFormatData {
    fn from(raw: u8) -> Self {
        FirmwareIdFormatData {
            value: match raw {
                0x00 => FirmwareIdFormat::FreeForm,
                0x01 => FirmwareIdFormat::UefiGuid,
                _ => match raw & 0x80 {
                    0x80 => FirmwareIdFormat::VendorOemSpecific,
                    _ => FirmwareIdFormat::None,
                },
            },
            raw,
        }
    }
}

/// # Firmware Id Format of [SMBiosFirmwareInventoryInformation]
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum FirmwareIdFormat {
    /// The format is a free-form string that is implementation specific.
    ///
    /// EXAMPLE: '35EQP72B',0
    FreeForm,
    /// The format is a string representation of the UEFI ESRT FwClass GUID or the UEFI Firmware
    /// Management Protocol ImageTypeId, as defined by the UEFI Specification. To represent the GUID, the
    /// string is formatted using the 36-character UUID string format specified in RFC4122: "xxxxxxxx-xxxxxxxx-xxxx-xxxxxxxxxxxx." Each x represents a hexadecimal digit (0-F).
    ///
    /// EXAMPLE: '1624a9df-5e13-47fc-874a-df3aff143089',0
    UefiGuid,
    /// BIOS Vendor/OEM-specific
    VendorOemSpecific,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Firmware Inventory Characteristics of [SMBiosFirmwareInventoryInformation]
#[derive(PartialEq, Eq)]
pub struct FirmwareInventoryCharacteristics {
    /// Raw value
    ///
    /// _raw_ is useful when there are values not yet defiend.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u16,
}

impl Deref for FirmwareInventoryCharacteristics {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl From<u16> for FirmwareInventoryCharacteristics {
    fn from(raw: u16) -> Self {
        FirmwareInventoryCharacteristics { raw }
    }
}

impl FirmwareInventoryCharacteristics {
    /// Updatable
    ///
    /// This firmware can be updated by software.
    pub fn updatable(&self) -> bool {
        self.raw & 0b0000_0000_0000_0001 == 0b0000_0000_0000_0001
    }

    /// Write-Protect
    ///
    /// This firmware is in a write-protected state.
    pub fn write_protect(&self) -> bool {
        self.raw & 0b0000_0000_0000_0010 == 0b0000_0000_0000_0010
    }
}

impl fmt::Debug for FirmwareInventoryCharacteristics {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<FirmwareInventoryCharacteristics>())
            .field("raw", &self.raw)
            .field("updatable", &self.updatable())
            .field("write_protect", &self.write_protect())
            .finish()
    }
}

impl Serialize for FirmwareInventoryCharacteristics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FirmwareInventoryCharacteristics", 6)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("updatable", &self.updatable())?;
        state.serialize_field("write_protect", &self.write_protect())?;
        state.end()
    }
}

/// # Firmware Inventory State Information Data of [SMBiosFirmwareInventoryInformation].
pub struct FirmwareInventoryStateInformationData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [FirmwareInventoryStateInformation] value
    pub value: FirmwareInventoryStateInformation,
}

impl fmt::Debug for FirmwareInventoryStateInformationData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<FirmwareInventoryStateInformationData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for FirmwareInventoryStateInformationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FirmwareInventoryStateInformationData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for FirmwareInventoryStateInformationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            FirmwareInventoryStateInformation::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for FirmwareInventoryStateInformationData {
    type Target = FirmwareInventoryStateInformation;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for FirmwareInventoryStateInformationData {
    fn from(raw: u8) -> Self {
        FirmwareInventoryStateInformationData {
            value: match raw {
                0x01 => FirmwareInventoryStateInformation::Other,
                0x02 => FirmwareInventoryStateInformation::Unknown,
                0x03 => FirmwareInventoryStateInformation::Disabled,
                0x04 => FirmwareInventoryStateInformation::Enabled,
                0x05 => FirmwareInventoryStateInformation::Absent,
                0x06 => FirmwareInventoryStateInformation::StandbyOffline,
                0x07 => FirmwareInventoryStateInformation::StandbySpare,
                0x08 => FirmwareInventoryStateInformation::UnavailableOffline,
                _ => FirmwareInventoryStateInformation::None,
            },
            raw,
        }
    }
}

/// # Firmware Inventory State Information of [SMBiosFirmwareInventoryInformation]
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum FirmwareInventoryStateInformation {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Disabled
    ///
    /// This firmware component is disabled
    Disabled,
    /// Enabled
    ///
    /// This firmware component is enabled
    Enabled,
    /// Absent
    ///
    /// This firmware component is either not present or not detected
    Absent,
    /// Standby Offline
    ///
    /// This firmware is enabled but awaits an external action to activate it
    StandbyOffline,
    /// Standby Spare
    ///
    /// This firmware is part of a redundancy set and awaits a failover or other external action to
    /// activate it
    StandbySpare,
    /// Unavailable Offline
    ///
    /// This firmware component is present but cannot be used
    UnavailableOffline,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Associated Component Handle Iterator
///
/// Iterates over the associated component handles contained within the [SMBiosFirmwareInventoryInformation] structure
pub struct AssociatedComponentHandleIterator<'a> {
    data: &'a SMBiosFirmwareInventoryInformation<'a>,
    current_index: usize,
    current_entry: u8,
    number_of_associated_components: u8,
}

impl<'a> AssociatedComponentHandleIterator<'a> {
    const HANDLES_OFFSET: usize = 0x18usize;

    /// Creates an instance of the associated component handle iterator.
    pub fn new(data: &'a SMBiosFirmwareInventoryInformation<'a>) -> Self {
        AssociatedComponentHandleIterator {
            data: data,
            current_index: Self::HANDLES_OFFSET,
            current_entry: 0,
            number_of_associated_components: data.number_of_associated_components().unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::HANDLES_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a AssociatedComponentHandleIterator<'a> {
    type Item = Handle;
    type IntoIter = AssociatedComponentHandleIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AssociatedComponentHandleIterator {
            data: self.data,
            current_index: AssociatedComponentHandleIterator::HANDLES_OFFSET,
            current_entry: 0,
            number_of_associated_components: self
                .data
                .number_of_associated_components()
                .unwrap_or(0),
        }
    }
}

impl<'a> Iterator for AssociatedComponentHandleIterator<'a> {
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_associated_components {
            self.reset();
            return None;
        }

        match self.data.parts().get_field_handle(self.current_index) {
            Some(current_handle) => {
                self.current_index = self.current_index + Handle::SIZE;
                self.current_entry = self.current_entry + 1;
                Some(current_handle)
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for AssociatedComponentHandleIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

impl<'a> Serialize for AssociatedComponentHandleIterator<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let handles: Vec<Handle> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(handles.len()))?;
        for e in handles {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_inventory_information() {
        let firmware_inventory_information_bytes = vec![
            // struct_type(45), length(0x1A), handle(0x10)
            0x2D, 0x1A, 0x10, 0x00,
            // firmware_component_name(1), firmware_version(2), version_format (FreeForm - 0), firmware_id(3), firmware_id_format (FreeForm - 0), release_date(4), manufacturer(5), lowest_supported_firmware_version(6)
            0x01, 0x02, 0x00, 0x03, 0x00, 0x04, 0x05, 0x06,
            // image_size(0xFFFFFFFFFFFFFFFF), characteristics(0x0001), state(StandbySpare - 0x07), number_of_components(1)
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x07, 0x01,
            // handle[0] == 0x0005
            0x05, 0x00, // firmware_component_name: "BMC Firmware" (1)
            b'B', b'M', b'C', b' ', b'F', b'i', b'r', b'm', b'w', b'a', b'r', b'e', 0x00,
            // firmware_version: "1.45.455b66-rev4" (2)
            b'1', b'.', b'4', b'5', b'.', b'4', b'5', b'5', b'b', b'6', b'6', b'-', b'r', b'e',
            b'v', b'4', 0x00, // firmware_id: "35EQP72B" (3)
            b'3', b'5', b'E', b'Q', b'P', b'7', b'2', b'B', 0x00,
            // release_date: "2021-05-15T04:14:33+06:00" (4)
            b'2', b'0', b'2', b'1', b'-', b'0', b'5', b'-', b'1', b'5', b'T', b'0', b'4', b':',
            b'1', b'4', b':', b'3', b'3', b'+', b'0', b'6', b':', b'0', b'0', 0x00,
            // manufacturer: "Apple" (5)
            b'A', b'p', b'p', b'l', b'e', 0x00,
            // lowest_supported_firmware_version: "1.23.456b78-rev9" (6)
            b'1', b'.', b'2', b'3', b'.', b'4', b'5', b'6', b'b', b'7', b'8', b'-', b'r', b'e',
            b'v', b'9', 0x00, // end of structure
            0x00,
        ];

        let parts = UndefinedStruct::new(&firmware_inventory_information_bytes);
        let firmware_inventory_information = SMBiosFirmwareInventoryInformation::new(&parts);

        // basic field tests
        assert_eq!(
            firmware_inventory_information
                .firmware_component_name()
                .unwrap(),
            "BMC Firmware".to_string()
        );
        assert_eq!(
            firmware_inventory_information.firmware_version().unwrap(),
            "1.45.455b66-rev4".to_string()
        );
        assert_eq!(
            firmware_inventory_information
                .version_format()
                .unwrap()
                .value,
            VersionFormat::FreeForm
        );
        assert_eq!(
            firmware_inventory_information.firmware_id().unwrap(),
            "35EQP72B".to_string()
        );
        assert_eq!(
            firmware_inventory_information
                .firmware_id_format()
                .unwrap()
                .value,
            FirmwareIdFormat::FreeForm
        );
        assert_eq!(
            firmware_inventory_information.release_date().unwrap(),
            "2021-05-15T04:14:33+06:00".to_string()
        );
        assert_eq!(
            firmware_inventory_information.manufacturer().unwrap(),
            "Apple".to_string()
        );
        assert_eq!(
            firmware_inventory_information
                .lowest_supported_firmware_version()
                .unwrap(),
            "1.23.456b78-rev9".to_string()
        );
        match firmware_inventory_information.image_size().unwrap() {
            FirmwareImageSize::Unknown => (),
            FirmwareImageSize::Bytes(_) => panic!("expected unknown"),
        }
        assert_eq!(
            firmware_inventory_information
                .characteristics()
                .unwrap()
                .updatable(),
            true
        );
        assert_eq!(
            firmware_inventory_information.state().unwrap().value,
            FirmwareInventoryStateInformation::StandbySpare
        );
        assert_eq!(
            firmware_inventory_information
                .number_of_associated_components()
                .unwrap(),
            1u8
        );

        let mut iterator = firmware_inventory_information.associated_component_handle_iterator();

        let first_entry = iterator.next().expect("has a first entry");
        assert_eq!(*first_entry, 0x0005);

        assert!(iterator.next().is_none());

        let mut counter = 0;

        for _entry in firmware_inventory_information.associated_component_handle_iterator() {
            counter = counter + 1;
        }

        assert_eq!(counter, 1);

        // debug print test
        println!(
            "firmware_inventory_information: {:?}",
            firmware_inventory_information
        );
    }
}
