use crate::*;

/// # On Board Devices Information (Type 10, Obsolete)
///
///  The information in this structure defines the attributes of devices that are onboard
/// (soldered onto) a system element, usually the baseboard. In general, an entry in this table implies that the
/// BIOS has some level of control over the enabling of the associated device for use by the system.
///
/// NOTE This structure is obsolete starting with version 2.6 of this specification; the [SMBiosOnboardDevicesExtendedInformation]
/// (Type 41) structure should be used instead. BIOS providers can choose to implement
/// both types to allow existing SMBIOS browsers to properly display the system’s onboard devices information.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosOnBoardDeviceInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosOnBoardDeviceInformation<'a> {
    const STRUCT_TYPE: u8 = 10u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosOnBoardDeviceInformation<'a> {
    /// The number of [OnBoardDevice] entries
    pub fn number_of_devices(&self) -> usize {
        let struct_length = self.parts().header.length() as usize;

        (struct_length - Header::SIZE) / OnBoardDevice::SIZE
    }

    /// Iterates over the [OnBoardDevice] entries
    pub fn onboard_device_iterator(&'a self) -> OnBoardDeviceIterator<'a> {
        OnBoardDeviceIterator::new(self)
    }
}

impl fmt::Debug for SMBiosOnBoardDeviceInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosOnBoardDeviceInformation>())
            .field("header", &self.parts.header)
            .field("number_of_devices", &self.number_of_devices())
            .field("onboard_device_iterator", &self.onboard_device_iterator())
            .finish()
    }
}

/// # On Board Device entry within [SMBiosOnBoardDeviceInformation]
pub struct OnBoardDevice<'a> {
    onboard_device_information: &'a SMBiosOnBoardDeviceInformation<'a>,
    entry_offset: usize,
}

impl<'a> OnBoardDevice<'a> {
    /// Size in bytes for this structure
    ///
    /// This structure is composed of:
    /// _device_type_ (byte) at offset 0,
    /// and _description_ (byte) at offset 1
    /// for a total size of two bytes.
    const SIZE: usize = 2;

    fn new(
        onboard_device_information: &'a SMBiosOnBoardDeviceInformation<'a>,
        entry_offset: usize,
    ) -> Self {
        Self {
            onboard_device_information,
            entry_offset,
        }
    }

    /// Device type
    pub fn device_type(&self) -> Option<OnBoardDeviceType> {
        self.onboard_device_information
            .parts()
            .get_field_byte(self.entry_offset)
            .and_then(|raw| Some(OnBoardDeviceType::from(raw)))
    }

    /// Device description
    pub fn description(&self) -> Option<String> {
        self.onboard_device_information
            .parts()
            .get_field_string(self.entry_offset + 1)
    }
}

impl fmt::Debug for OnBoardDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<OnBoardDevice>())
            .field("device_type", &self.device_type())
            .field("description", &self.description())
            .finish()
    }
}

/// # On Board Device Type
pub struct OnBoardDeviceType {
    /// Raw value
    pub raw: u8,
}

impl OnBoardDeviceType {
    /// One of the onboard device types
    pub fn type_of_device(&self) -> TypeOfDevice {
        let result = self.raw & 0x7F;
        match result {
            0x01 => TypeOfDevice::Other,
            0x02 => TypeOfDevice::Unknown,
            0x03 => TypeOfDevice::Video,
            0x04 => TypeOfDevice::ScsiController,
            0x05 => TypeOfDevice::Ethernet,
            0x06 => TypeOfDevice::TokenRing,
            0x07 => TypeOfDevice::Sound,
            0x08 => TypeOfDevice::PataController,
            0x09 => TypeOfDevice::SataController,
            0x0A => TypeOfDevice::SasController,
            _ => TypeOfDevice::None,
        }
    }

    /// Enabled/disabled device status
    pub fn status(&self) -> DeviceStatus {
        if self.raw & 0x80 == 0x80 {
            DeviceStatus::Enabled
        } else {
            DeviceStatus::Disabled
        }
    }
}

impl From<u8> for OnBoardDeviceType {
    fn from(raw: u8) -> Self {
        OnBoardDeviceType { raw }
    }
}

impl fmt::Debug for OnBoardDeviceType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<OnBoardDevice>())
            .field("raw", &self.raw)
            .field("type_of_device", &self.type_of_device())
            .field("status", &self.status())
            .finish()
    }
}

/// # Onboard Device Types
#[derive(Debug, PartialEq, Eq)]
pub enum TypeOfDevice {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Video
    Video,
    /// SCSI Controller
    ScsiController,
    /// Ethernet
    Ethernet,
    /// Token Ring
    TokenRing,
    /// Sound
    Sound,
    /// PATA Controller
    PataController,
    /// SATA Controller
    SataController,
    /// SAS Controller
    SasController,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Enabled/Disabled Device Status
#[derive(Debug, PartialEq, Eq)]
pub enum DeviceStatus {
    /// Device is enabled
    Enabled,
    /// Device is disabled
    Disabled,
}

/// # On-board Device Itereator for [OnBoardDevice]s contained within [SMBiosOnBoardDeviceInformation]
pub struct OnBoardDeviceIterator<'a> {
    data: &'a SMBiosOnBoardDeviceInformation<'a>,
    current_index: usize,
    current_entry: usize,
    number_of_entries: usize,
}

impl<'a> OnBoardDeviceIterator<'a> {
    const DEVICES_OFFSET: usize = 4usize;

    fn new(data: &'a SMBiosOnBoardDeviceInformation<'a>) -> Self {
        OnBoardDeviceIterator {
            data: data,
            current_index: Self::DEVICES_OFFSET,
            current_entry: 0,
            number_of_entries: data.number_of_devices(),
        }
    }

    fn reset(&mut self) {
        self.current_index = Self::DEVICES_OFFSET;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a OnBoardDeviceIterator<'a> {
    type Item = OnBoardDevice<'a>;
    type IntoIter = OnBoardDeviceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        OnBoardDeviceIterator {
            data: self.data,
            current_index: OnBoardDeviceIterator::DEVICES_OFFSET,
            current_entry: 0,
            number_of_entries: self.data.number_of_devices(),
        }
    }
}

impl<'a> Iterator for OnBoardDeviceIterator<'a> {
    type Item = OnBoardDevice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_entries {
            self.reset();
            return None;
        }

        let next_index = self.current_index + OnBoardDevice::SIZE;
        match self
            .data
            .parts()
            .get_field_data(self.current_index, next_index)
        {
            Some(_) => {
                let result = OnBoardDevice::new(self.data, self.current_index);
                self.current_index = next_index;
                self.current_entry = self.current_entry + 1;
                Some(result)
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for OnBoardDeviceIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type10 = vec![
            0x0A, 0x06, 0x21, 0x00, 0x83, 0x01, 0x20, 0x20, 0x20, 0x54, 0x6F, 0x20, 0x42, 0x65,
            0x20, 0x46, 0x69, 0x6C, 0x6C, 0x65, 0x64, 0x20, 0x42, 0x79, 0x20, 0x4F, 0x2E, 0x45,
            0x2E, 0x4D, 0x2E, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type10.as_slice());
        let test_struct = SMBiosOnBoardDeviceInformation::new(&parts);

        println!("{:?}", test_struct);

        assert_eq!(test_struct.number_of_devices(), 1);

        let mut iterator = test_struct.onboard_device_iterator().into_iter();

        let item = iterator.next().unwrap();

        assert_eq!(
            item.description(),
            Some("   To Be Filled By O.E.M.".to_string())
        );

        let device_type = item.device_type().unwrap();
        assert_eq!(device_type.type_of_device(), TypeOfDevice::Video);
        assert_eq!(device_type.status(), DeviceStatus::Enabled);

        assert!(iterator.next().is_none());
    }
}
