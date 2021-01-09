use super::*;
use std::{convert::TryInto, ops::Deref};
use crate::fields::*;
use std::fmt;

// use super::SMBiosUnknown;

pub struct Handle(u16);

impl Deref for Handle {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Handle>())
            .field("handle", &self.0)
            .finish()
    }
}

pub fn get_field_handle(offset:usize, data:&[u8]) -> Option<Handle> {
    match data.get(offset .. offset + 2) {
        Some(val) => Some(Handle(u16::from_le_bytes(val.try_into().expect("array length does not match type width")))),
        None => None,
    }    
}

#[derive(Debug)]
pub enum DefinedStruct<'a>
{
    Information(SMBiosInformation<'a>),
    SystemInformation(SMBiosSystemInformation<'a>),
    BaseBoardInformation(SMBiosBaseboardInformation<'a>),
    SystemChassisInformation(SMBiosSystemChassisInformation<'a>),
    ProcessorInformation(SMBiosProcessorInformation<'a>),
    MemoryControllerInformation(SMBiosMemoryControllerInformation<'a>),
    MemoryModuleInformation(SMBiosMemoryModuleInformation<'a>),
    CacheInformation(SMBiosCacheInformation<'a>),
    PortConnectorInformation(SMBiosPortConnectorInformation<'a>),
    SystemSlot(SMBiosSystemSlot<'a>),
    OnBoardDeviceInformation(SMBiosOnBoardDeviceInformation<'a>),
    OemStrings(SMBiosOemStrings<'a>),
    SystemConfigurationOptions(SMBiosSystemConfigurationOptions<'a>),
    LanguageInformation(SMBiosBiosLanguageInformation<'a>),
    GroupAssociations(SMBiosGroupAssociations<'a>),
    EventLog(SMBiosSystemEventLog<'a>),
    PhysicalMemoryArray(SMBiosPhysicalMemoryArray<'a>),
    MemoryDevice(SMBiosMemoryDevice<'a>),
    MemoryErrorInformation32Bit(SMBiosMemoryErrorInformation32<'a>),
    MemoryArrayMappedAddress(SMBiosMemoryArrayMappedAddress<'a>),
    MemoryDeviceMappedAddress(SMBiosMemoryDeviceMappedAddress<'a>),
    BuiltInPointingDevice(SMBiosBuiltInPointingDevice<'a>),
    PortableBattery(SMBiosPortableBattery<'a>),
    SystemReset(SMBiosSystemReset<'a>),
    HardwareSecurity(SMBiosHardwareSecurity<'a>),
    SystemPowerControls(SMBiosSystemPowerControls<'a>),
    VoltageProbe(SMBiosVoltageProbe<'a>),
    CoolingDevice(SMBiosCoolingDevice<'a>),
    TemperatureProbe(SMBiosTemperatureProbe<'a>),
    ElectricalCurrentProbe(SMBiosElectricalCurrentProbe<'a>),
    OutOfBandRemoteAccess(SMBiosOutOfBandRemoteAccess<'a>),
    BisEntryPoint(SMBiosBisEntryPoint<'a>),
    SystemBootInformation(SMBiosSystemBootInformation<'a>),
    MemoryErrorInformation64Bit(SMBiosMemoryErrorInformation64<'a>),
    ManagementDevice(SMBiosManagementDevice<'a>),
    ManagementDeviceComponent(SMBiosManagementDeviceComponent<'a>),
    ManagementDeviceThresholdData(SMBiosManagementDeviceThresholdData<'a>),
    MemoryChannel(SMBiosMemoryChannel<'a>),
    IpmiDeviceInformation(SMBiosIpmiDeviceInformation<'a>),
    SystemPowerSupply(SMBiosSystemPowerSupply<'a>),
    AdditionalInformation(SMBiosAdditionalInformation<'a>),
    OnboardDevicesExtendedInformation(SMBiosOnboardDevicesExtendedInformation<'a>),
    ManagementControllerHostInterface(SMBiosManagementControllerHostInterface<'a>),
    TpmDevice(SMBiosTpmDevice<'a>),
    ProcessorAdditionalInformation(SMBiosProcessorAdditionalInformation<'a>),
    Inactive(SMBiosInactive<'a>),
    EndOfTable(SMBiosEndOfTable<'a>),
    Unknown(SMBiosUnknown<'a>),
}

// impl fmt::Debug for StructName<'_> {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.debug_struct(std::any::type_name::<StructName>())
//         .field("header", &self)
//         .finish()
//     }
// }

pub struct SMBiosStructParts<'a> {
    pub header: Header<'a>,
    data: &'a [u8],
    strings: Strings<'a>,
}

impl<'a> SMBiosStructParts<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        SMBiosStructParts { 
            header: Header::new(data.get(..Header::SIZE).expect("A minimum of Header::SIZE bytes are required.")), 
            data, 
            strings: Strings::new(data) 
        }
    }

    pub fn get_field_byte(&self, offset:usize) -> Option<u8> {
        match self.data.get(offset .. offset + 1) {
            Some(val) => Some(val[0]),
            None => None,
        }
    }

    pub fn get_field_word(&self, offset:usize) -> Option<u16> {
        match self.data.get(offset .. offset + 2) {
            Some(val) => Some(u16::from_le_bytes(val.try_into().expect("array length does not match type width"))),
            None => None,
        }    
    }

    pub fn get_field_handle(&self, offset:usize) -> Option<Handle> {
        match self.data.get(offset .. offset + 2) {
            Some(val) => Some(Handle(u16::from_le_bytes(val.try_into().expect("array length does not match type width")))),
            None => None,
        }    
    }

    pub fn get_field_dword(&self, offset:usize) -> Option<u32> {
        match self.data.get(offset .. offset + 4) {
            Some(val) => Some(u32::from_le_bytes(val.try_into().expect("array length does not match type width"))),
            None => None,
        }    
    }

    pub fn get_field_qword(&self, offset:usize) -> Option<u64> {
        match self.data.get(offset .. offset + 8) {
            Some(val) => Some(u64::from_le_bytes(val.try_into().expect("array length does not match type width"))),
            None => None,
        }    
    }

    pub fn get_field_string(&self, offset:usize) -> Option<String> {
        match self.get_field_byte(offset) {
            Some(val) => self.strings.get_string(val),
            None => None,
        }    
    }

    // todo: learn how to pass an index range (SliceIndex?) rather than start/end indices.
    // This would better conform to the Rust design look and feel.
    pub fn get_field_data(&self, start_index:usize, end_index:usize) -> Option<&[u8]> {
        return self.data.get(start_index .. end_index)
    }

    pub fn as_type<T : SMBiosStruct<'a>>(&'a self) -> Option<T> {
        if T::STRUCT_TYPE == self.header.struct_type() {
            Some(T::new(self))
        }
        else {
            None
        }
    }

    pub fn struct_type_name(&self) -> DefinedStruct {
        match self.header.struct_type() {
            SMBiosInformation::STRUCT_TYPE => DefinedStruct::Information(SMBiosInformation::new(self)),
            SMBiosSystemInformation::STRUCT_TYPE => DefinedStruct::SystemInformation(SMBiosSystemInformation::new(self)),
            SMBiosBaseboardInformation::STRUCT_TYPE => DefinedStruct::BaseBoardInformation(SMBiosBaseboardInformation::new(self)),
            SMBiosSystemChassisInformation::STRUCT_TYPE => DefinedStruct::SystemChassisInformation(SMBiosSystemChassisInformation::new(self)),
            SMBiosProcessorInformation::STRUCT_TYPE => DefinedStruct::ProcessorInformation(SMBiosProcessorInformation::new(self)),
            SMBiosMemoryControllerInformation::STRUCT_TYPE => DefinedStruct::MemoryControllerInformation(SMBiosMemoryControllerInformation::new(self)),
            SMBiosMemoryModuleInformation::STRUCT_TYPE => DefinedStruct::MemoryModuleInformation(SMBiosMemoryModuleInformation::new(self)),
            SMBiosCacheInformation::STRUCT_TYPE => DefinedStruct::CacheInformation(SMBiosCacheInformation::new(self)),
            SMBiosPortConnectorInformation::STRUCT_TYPE => DefinedStruct::PortConnectorInformation(SMBiosPortConnectorInformation::new(self)),
            SMBiosSystemSlot::STRUCT_TYPE => DefinedStruct::SystemSlot(SMBiosSystemSlot::new(self)),
            SMBiosOnBoardDeviceInformation::STRUCT_TYPE => DefinedStruct::OnBoardDeviceInformation(SMBiosOnBoardDeviceInformation::new(self)),
            SMBiosOemStrings::STRUCT_TYPE => DefinedStruct::OemStrings(SMBiosOemStrings::new(self)),
            SMBiosSystemConfigurationOptions::STRUCT_TYPE => DefinedStruct::SystemConfigurationOptions(SMBiosSystemConfigurationOptions::new(self)),
            SMBiosBiosLanguageInformation::STRUCT_TYPE => DefinedStruct::LanguageInformation(SMBiosBiosLanguageInformation::new(self)),
            SMBiosGroupAssociations::STRUCT_TYPE => DefinedStruct::GroupAssociations(SMBiosGroupAssociations::new(self)),
            SMBiosSystemEventLog::STRUCT_TYPE => DefinedStruct::EventLog(SMBiosSystemEventLog::new(self)),
            SMBiosPhysicalMemoryArray::STRUCT_TYPE => DefinedStruct::PhysicalMemoryArray(SMBiosPhysicalMemoryArray::new(self)),
            SMBiosMemoryDevice::STRUCT_TYPE => DefinedStruct::MemoryDevice(SMBiosMemoryDevice::new(self)),
            SMBiosMemoryErrorInformation32::STRUCT_TYPE => DefinedStruct::MemoryErrorInformation32Bit(SMBiosMemoryErrorInformation32::new(self)),
            SMBiosMemoryArrayMappedAddress::STRUCT_TYPE => DefinedStruct::MemoryArrayMappedAddress(SMBiosMemoryArrayMappedAddress::new(self)),
            SMBiosMemoryDeviceMappedAddress::STRUCT_TYPE => DefinedStruct::MemoryDeviceMappedAddress(SMBiosMemoryDeviceMappedAddress::new(self)),
            SMBiosBuiltInPointingDevice::STRUCT_TYPE => DefinedStruct::BuiltInPointingDevice(SMBiosBuiltInPointingDevice::new(self)),
            SMBiosPortableBattery::STRUCT_TYPE => DefinedStruct::PortableBattery(SMBiosPortableBattery::new(self)),
            SMBiosSystemReset::STRUCT_TYPE => DefinedStruct::SystemReset(SMBiosSystemReset::new(self)),
            SMBiosHardwareSecurity::STRUCT_TYPE => DefinedStruct::HardwareSecurity(SMBiosHardwareSecurity::new(self)),
            SMBiosSystemPowerControls::STRUCT_TYPE => DefinedStruct::SystemPowerControls(SMBiosSystemPowerControls::new(self)),
            SMBiosVoltageProbe::STRUCT_TYPE => DefinedStruct::VoltageProbe(SMBiosVoltageProbe::new(self)),
            SMBiosCoolingDevice::STRUCT_TYPE => DefinedStruct::CoolingDevice(SMBiosCoolingDevice::new(self)),
            SMBiosTemperatureProbe::STRUCT_TYPE => DefinedStruct::TemperatureProbe(SMBiosTemperatureProbe::new(self)),
            SMBiosElectricalCurrentProbe::STRUCT_TYPE => DefinedStruct::ElectricalCurrentProbe(SMBiosElectricalCurrentProbe::new(self)),
            SMBiosOutOfBandRemoteAccess::STRUCT_TYPE => DefinedStruct::OutOfBandRemoteAccess(SMBiosOutOfBandRemoteAccess::new(self)),
            SMBiosBisEntryPoint::STRUCT_TYPE => DefinedStruct::BisEntryPoint(SMBiosBisEntryPoint::new(self)),
            SMBiosSystemBootInformation::STRUCT_TYPE => DefinedStruct::SystemBootInformation(SMBiosSystemBootInformation::new(self)),
            SMBiosMemoryErrorInformation64::STRUCT_TYPE => DefinedStruct::MemoryErrorInformation64Bit(SMBiosMemoryErrorInformation64::new(self)),
            SMBiosManagementDevice::STRUCT_TYPE => DefinedStruct::ManagementDevice(SMBiosManagementDevice::new(self)),
            SMBiosManagementDeviceComponent::STRUCT_TYPE => DefinedStruct::ManagementDeviceComponent(SMBiosManagementDeviceComponent::new(self)),
            SMBiosManagementDeviceThresholdData::STRUCT_TYPE => DefinedStruct::ManagementDeviceThresholdData(SMBiosManagementDeviceThresholdData::new(self)),
            SMBiosMemoryChannel::STRUCT_TYPE => DefinedStruct::MemoryChannel(SMBiosMemoryChannel::new(self)),
            SMBiosIpmiDeviceInformation::STRUCT_TYPE => DefinedStruct::IpmiDeviceInformation(SMBiosIpmiDeviceInformation::new(self)),
            SMBiosSystemPowerSupply::STRUCT_TYPE => DefinedStruct::SystemPowerSupply(SMBiosSystemPowerSupply::new(self)),
            SMBiosAdditionalInformation::STRUCT_TYPE => DefinedStruct::AdditionalInformation(SMBiosAdditionalInformation::new(self)),
            SMBiosOnboardDevicesExtendedInformation::STRUCT_TYPE => DefinedStruct::OnboardDevicesExtendedInformation(SMBiosOnboardDevicesExtendedInformation::new(self)),
            SMBiosManagementControllerHostInterface::STRUCT_TYPE => DefinedStruct::ManagementControllerHostInterface(SMBiosManagementControllerHostInterface::new(self)),
            SMBiosTpmDevice::STRUCT_TYPE => DefinedStruct::TpmDevice(SMBiosTpmDevice::new(self)),
            SMBiosProcessorAdditionalInformation::STRUCT_TYPE => DefinedStruct::ProcessorAdditionalInformation(SMBiosProcessorAdditionalInformation::new(self)),
            SMBiosInactive::STRUCT_TYPE => DefinedStruct::Inactive(SMBiosInactive::new(self)),
            SMBiosEndOfTable::STRUCT_TYPE => DefinedStruct::EndOfTable(SMBiosEndOfTable::new(self)),
            _ => DefinedStruct::Unknown(SMBiosUnknown::new(self)),
        }
    }
}

impl fmt::Debug for SMBiosStructParts<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosStructParts>())
            .field("header", &self.header)
            .finish()
    }
}

pub struct Strings<'a> {
    data: &'a [u8],
}

impl<'a> Strings<'a> {
    fn new(data: &'a [u8]) -> Self {
        Strings { data }
    }

    pub fn get_string(&self, index: u8) -> Option<String> {
        if index < 1 { 
            // BIOS strings are 1 based indexing, ignore bad input
            return None;
        }

        let data_length = self.data.len();
        match get_field_byte(1, self.data) {
            Some(string_area_start_index) => {
                match self.data.get(string_area_start_index as usize .. data_length - 2) {
                    Some(string_area) => {
                        match string_area.split(|num| *num == 0).skip(index as usize - 1).next() {
                            Some(string_as_slice) => {
                                let mut bios_string: Vec<char> = Vec::new();
                                for a in string_as_slice {
                                    bios_string.push(*a as char); // byte to Windows-1252 (ISO-8859-1 superset)
                                };
                                Some(bios_string.into_iter().collect())
                            },
                            None => None
                        }
                    },
                    None => None
                }
            },
            None => None
        }
    }
}

pub struct Header<'a> {
    data: &'a [u8],
}

impl fmt::Debug for Header<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Header>())
            .field("struct_type", &self.struct_type())
            // .field("struct_type_name", &self.struct_type_name())
            .field("length", &self.length())
            .field("handle", &self.handle())
            .finish()
    }
}

impl<'a> Header<'a> {
    const SIZE: usize = 4;

    fn new(data: &'a [u8]) -> Self {
        assert!(data.len() == Self::SIZE, "Header must be 4 bytes in length, 1 for struct_type, 1 for length, and 2 for handle.");
        Header { data }
    }

    pub fn struct_type(&self) -> u8 {
        self.data[0] // struct_type is 1 byte at offset 0
    }

    pub fn length(&self) -> u8 {
        self.data[1] // length is 1 byte at offset 1
    }

    pub fn handle(&self) -> Handle {
        // handle is 2 bytes at offset 2
        Handle(u16::from_le_bytes(self.data[2..4].try_into().expect("array length does not match type width")))
    }
}

pub struct SMBiosTableData<'a> {
    data: &'a [u8],
}

impl<'a> SMBiosTableData<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data } }
}

impl<'a> IntoIterator for SMBiosTableData<'a> {
    type Item = SMBiosStructParts<'a>;

    type IntoIter = RawStructIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RawStructIterator::new(self.data)
    }
}

impl<'a> IntoIterator for &'a SMBiosTableData<'a> {
    type Item = SMBiosStructParts<'a>;

    type IntoIter = RawStructIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RawStructIterator::new(self.data)
    }
}
impl<'a> fmt::Debug for SMBiosTableData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: format as an array, make a function on SMBiosStructParts to return an enum of variants of the struct types
        self.into_iter().map(|x| writeln!(f, "{:?}", x)).collect()
    }
}

pub trait SMBiosStruct<'a> {
    const STRUCT_TYPE: u8;

    fn new(parts: &'a SMBiosStructParts<'a>) -> Self;

    fn parts(&self) -> &'a SMBiosStructParts<'a>;
}

pub struct RawStructIterator<'a> {
    data: &'a [u8],
    current_index : usize,
}

impl<'a> RawStructIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        RawStructIterator{data: data, current_index: 0}
    }
}

impl<'a> Iterator for RawStructIterator<'a> {
    type Item = SMBiosStructParts<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_index = self.current_index;
        let len = self.data.len();

        // We are done iterating if current_index points beyond the end of "data".
        if next_index >= len {
            self.current_index = 0;
            return None;
        }

        // When calling "next()" the first time, ensure "data" is valid before attempting iteration.
        // A valid structure has:
        // - At least 6 bytes.  A header of 4 bytes plus the terminating two bytes (\0\0) in the string area.
        // - The second byte indicates the structure length (header plus structure data).
        //   The length does not include the string area (which at a minimum the last two bytes of zero)
        // - The last two bytes are 0 (the end of the string area)
        if next_index == 0 
        && (len < Header::SIZE + 2 // struct is too short
            || (self.data[next_index + 1] as usize) > len - 2 // struct header specifies a length too long
            || self.data[len - 2] != 0 // 2nd to last byte should be zero and it is not
            || self.data[len - 1] != 0) { // Last byte should be zero and it is not
            return None;
        }

        // next_index is pointing at the start of the structure header.
        // Read the struct header length at offset 1 of the header (next_index + 1) and advance to the
        // string area which follows the stucture.
        next_index = next_index + self.data[next_index + 1] as usize;

        // next_index is pointing at the start of the string area.
        // The string area is terminated with \0\0.  If no strings exist then its contents is \0\0.
        // Search for \0\0 and point at the byte immediately after it.  That point is either the start of the 
        // next structure header or one byte beyond the end of "data".
        let mut a:bool;
        let mut b = true;
        loop {
            a = self.data[next_index] != 0;
            next_index = next_index + 1;
            if a || b {
                b = self.data[next_index] != 0;
                next_index = next_index + 1;
            }
            if !(a || b) { break; }
        }

        let previous_index = self.current_index;
        self.current_index = next_index;

        match self.data.get(previous_index..self.current_index) {
            Some(val) => Some(SMBiosStructParts::new(val)),
            None => None
        }
    }
}
