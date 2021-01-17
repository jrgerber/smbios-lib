use super::*;
use std::fmt;
use std::{convert::TryInto, ops::Deref};

// use super::SMBiosUnknown;

/// # Structure Handle
///
/// Each SMBIOS structure has a handle or instance value associated with it.
/// Some structures will reference other structures by using this value.
///
/// Dereference a handle (*handle) to access its u16 value.
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

/// Retrieves a handle at the given offset
pub fn get_field_handle(offset: usize, data: &[u8]) -> Option<Handle> {
    match data.get(offset..offset + 2) {
        Some(val) => Some(Handle(u16::from_le_bytes(
            val.try_into()
                .expect("array length does not match type width"),
        ))),
        None => None,
    }
}

/// # SMBIOS Standard Defined Structure
///
/// Represents one of the SMBIOS defined structures or, in the case
/// of an OEM defined structure, as a generically defined Unknown variant
#[derive(Debug)]
pub enum DefinedStruct<'a> {
    /// BIOS Information (Type 0)
    Information(SMBiosInformation<'a>),
    /// System Information (Type 1)
    SystemInformation(SMBiosSystemInformation<'a>),
    /// Baseboard (or Module) Information (Type 2)
    BaseBoardInformation(SMBiosBaseboardInformation<'a>),
    /// System Enclosure or Chassis (Type 3)
    SystemChassisInformation(SMBiosSystemChassisInformation<'a>),
    /// Processor Information (Type 4)
    ProcessorInformation(SMBiosProcessorInformation<'a>),
    /// Memory Controller Information (Type 5, Obsolete)
    MemoryControllerInformation(SMBiosMemoryControllerInformation<'a>),
    /// Memory Module Information (Type 6, Obsolete)
    MemoryModuleInformation(SMBiosMemoryModuleInformation<'a>),
    /// Cache Informaiton (Type 7)
    CacheInformation(SMBiosCacheInformation<'a>),
    /// Port Connector Information (Type 8)
    PortConnectorInformation(SMBiosPortConnectorInformation<'a>),
    /// System Slot Information (Type 9)
    SystemSlot(SMBiosSystemSlot<'a>),
    /// On Board Devices Information (Type 10, Obsolete)
    OnBoardDeviceInformation(SMBiosOnBoardDeviceInformation<'a>),
    /// OEM Strings (Type 11)
    OemStrings(SMBiosOemStrings<'a>),
    /// System Configuration Options (Type 12)
    SystemConfigurationOptions(SMBiosSystemConfigurationOptions<'a>),
    /// BIOS Language Information (Type 13)
    LanguageInformation(SMBiosBiosLanguageInformation<'a>),
    /// Group Associations (Type 14)
    GroupAssociations(SMBiosGroupAssociations<'a>),
    /// System Event Log (Type 15)
    EventLog(SMBiosSystemEventLog<'a>),
    /// Physical Memory Array (Type 16)
    PhysicalMemoryArray(SMBiosPhysicalMemoryArray<'a>),
    /// Memory Device (Type 17)
    MemoryDevice(SMBiosMemoryDevice<'a>),
    /// 32-Bit Memory Error Information (Type 18)
    MemoryErrorInformation32Bit(SMBiosMemoryErrorInformation32<'a>),
    /// Memory Array Mapped Address (Type 19)
    MemoryArrayMappedAddress(SMBiosMemoryArrayMappedAddress<'a>),
    /// Memory Device Mapped Address (Type 20)
    MemoryDeviceMappedAddress(SMBiosMemoryDeviceMappedAddress<'a>),
    /// Built-in Pointing Device (Type 21)
    BuiltInPointingDevice(SMBiosBuiltInPointingDevice<'a>),
    /// Portable Battery (Type 22)
    PortableBattery(SMBiosPortableBattery<'a>),
    /// System Reset (Type 23)
    SystemReset(SMBiosSystemReset<'a>),
    /// Hardware Security (Type 24)
    HardwareSecurity(SMBiosHardwareSecurity<'a>),
    /// System Power Controls (Type 25)
    SystemPowerControls(SMBiosSystemPowerControls<'a>),
    /// Voltage Probe (Type 26)
    VoltageProbe(SMBiosVoltageProbe<'a>),
    /// Cooling Device (Type 27)
    CoolingDevice(SMBiosCoolingDevice<'a>),
    /// Temperature Probe (Type 28)
    TemperatureProbe(SMBiosTemperatureProbe<'a>),
    /// Electrical Current Probe (Type 29)
    ElectricalCurrentProbe(SMBiosElectricalCurrentProbe<'a>),
    /// Out-of-Band Remote Access (Type 30)
    OutOfBandRemoteAccess(SMBiosOutOfBandRemoteAccess<'a>),
    /// Boot Integrity Services (BIS) (Type 31)
    BisEntryPoint(SMBiosBisEntryPoint<'a>),
    /// System Boot Information (Type 32)
    SystemBootInformation(SMBiosSystemBootInformation<'a>),
    /// 64-Bit Memory Error Information (Type 33)
    MemoryErrorInformation64Bit(SMBiosMemoryErrorInformation64<'a>),
    /// Management Device (Type 34)
    ManagementDevice(SMBiosManagementDevice<'a>),
    /// Management Device Component (Type 35)
    ManagementDeviceComponent(SMBiosManagementDeviceComponent<'a>),
    /// Management Device Threshold Data (Type 36)
    ManagementDeviceThresholdData(SMBiosManagementDeviceThresholdData<'a>),
    /// Memory Channel (Type 37)
    MemoryChannel(SMBiosMemoryChannel<'a>),
    /// IPMI Device Information (Type 38)
    IpmiDeviceInformation(SMBiosIpmiDeviceInformation<'a>),
    /// Power Supply (Type 39)
    SystemPowerSupply(SMBiosSystemPowerSupply<'a>),
    /// Additional Information (Type 40)
    AdditionalInformation(SMBiosAdditionalInformation<'a>),
    /// Onboard Devices Extended Information (Type 41)
    OnboardDevicesExtendedInformation(SMBiosOnboardDevicesExtendedInformation<'a>),
    /// Management Controller Host Interface (Type 42)
    ManagementControllerHostInterface(SMBiosManagementControllerHostInterface<'a>),
    /// TPM Device (Type 43)
    TpmDevice(SMBiosTpmDevice<'a>),
    /// Processor Additional Information (Type 44)
    ProcessorAdditionalInformation(SMBiosProcessorAdditionalInformation<'a>),
    /// Inactive (Type 126)
    Inactive(SMBiosInactive<'a>),
    /// End-of-Table (Type 127)
    EndOfTable(SMBiosEndOfTable<'a>),
    /// OEM-Defined or Unknown Structure
    ///
    /// - A structure with a type value not yet defined, such as by a DMTF specification
    /// that supercedes the types known by this library
    /// - An OEM type with a value > 127.
    Unknown(SMBiosUnknown<'a>),
}

// impl fmt::Debug for StructName<'_> {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.debug_struct(std::any::type_name::<StructName>())
//         .field("header", &self)
//         .finish()
//     }
// }

/// # The three basic parts of an SMBIOS structure
///
/// Every SMBIOS structure contains three parts or sections: A header,
/// structure data, and string data.
pub struct SMBiosStructParts<'a> {
    /// The [Header] of the structure
    pub header: Header<'a>,
    /// The raw data for the entire structure including header and strings
    pub data: &'a [u8],
    /// The strings of the structure
    pub strings: Strings<'a>,
}

impl<'a> SMBiosStructParts<'a> {
    /// Creates a structure instance of the given byte array slice
    pub fn new(data: &'a [u8]) -> Self {
        SMBiosStructParts {
            header: Header::new(
                data.get(..Header::SIZE)
                    .expect("A minimum of Header::SIZE bytes are required."),
            ),
            data,
            strings: {
                //let string_area_start_index = data[1];
                let string_area_start_index = data
                    .get(Header::LENGTH_OFFSET..Header::LENGTH_OFFSET + 1)
                    .unwrap_or(&[0])[0];
                Strings::new(
                    data.get(string_area_start_index as usize..data.len() - 2)
                        .unwrap_or(&[]),
                )
            }, //strings: Strings::new(data)
        }
    }

    /// Retrieve a byte at the given offset from the structure's data section
    pub fn get_field_byte(&self, offset: usize) -> Option<u8> {
        match self.data.get(offset..offset + 1) {
            Some(val) => Some(val[0]),
            None => None,
        }
    }

    /// Retrieve a WORD at the given offset from the structure's data section
    pub fn get_field_word(&self, offset: usize) -> Option<u16> {
        match self.data.get(offset..offset + 2) {
            Some(val) => Some(u16::from_le_bytes(
                val.try_into()
                    .expect("array length does not match type width"),
            )),
            None => None,
        }
    }

    /// Retrieve a [Handle] at the given offset from the structure's data section
    pub fn get_field_handle(&self, offset: usize) -> Option<Handle> {
        match self.data.get(offset..offset + 2) {
            Some(val) => Some(Handle(u16::from_le_bytes(
                val.try_into()
                    .expect("array length does not match type width"),
            ))),
            None => None,
        }
    }

    /// Retrieve a DWORD at the given offset from the structure's data section
    pub fn get_field_dword(&self, offset: usize) -> Option<u32> {
        match self.data.get(offset..offset + 4) {
            Some(val) => Some(u32::from_le_bytes(
                val.try_into()
                    .expect("array length does not match type width"),
            )),
            None => None,
        }
    }

    /// Retrieve a QWORD at the given offset from the structure's data section
    pub fn get_field_qword(&self, offset: usize) -> Option<u64> {
        match self.data.get(offset..offset + 8) {
            Some(val) => Some(u64::from_le_bytes(
                val.try_into()
                    .expect("array length does not match type width"),
            )),
            None => None,
        }
    }

    /// Retrieve a String of the given offset
    ///
    /// Retrieval of strings is a two part operation. The given offset
    /// contains a byte whose value is a 1 based index into the strings section.
    /// The string is thus retrieved from the strings section based on the
    /// byte value at the given offset.
    pub fn get_field_string(&self, offset: usize) -> Option<String> {
        match self.get_field_byte(offset) {
            Some(val) => self.strings.get_string(val),
            None => None,
        }
    }

    // todo: learn how to pass an index range (SliceIndex?) rather than start/end indices.
    // This would better conform to the Rust design look and feel.

    /// Retrieve a block of bytes from the structure's data section
    pub fn get_field_data(&self, start_index: usize, end_index: usize) -> Option<&[u8]> {
        return self.data.get(start_index..end_index);
    }

    /// Cast to a given structure
    ///
    /// When this library does not contain a [DefinedStruct] variant
    /// matching the SMBiosStruct::STRUCT_TYPE, this function affords a cast to the
    /// given type. Such would be the case with OEM structure type T
    /// (which implements the [SMBiosStruct] trait).
    ///
    /// TODO: This should panic (not be Option) when the STRUCT_TYPE does not match because
    /// this would be a logic error in code, not a runtime error.
    pub fn as_type<T: SMBiosStruct<'a>>(&'a self) -> Option<T> {
        if T::STRUCT_TYPE == self.header.struct_type() {
            Some(T::new(self))
        } else {
            None
        }
    }

    /// Casts the current structure to its specific defined BIOS structure type
    pub fn struct_type_name(&self) -> DefinedStruct {
        match self.header.struct_type() {
            SMBiosInformation::STRUCT_TYPE => {
                DefinedStruct::Information(SMBiosInformation::new(self))
            }
            SMBiosSystemInformation::STRUCT_TYPE => {
                DefinedStruct::SystemInformation(SMBiosSystemInformation::new(self))
            }
            SMBiosBaseboardInformation::STRUCT_TYPE => {
                DefinedStruct::BaseBoardInformation(SMBiosBaseboardInformation::new(self))
            }
            SMBiosSystemChassisInformation::STRUCT_TYPE => {
                DefinedStruct::SystemChassisInformation(SMBiosSystemChassisInformation::new(self))
            }
            SMBiosProcessorInformation::STRUCT_TYPE => {
                DefinedStruct::ProcessorInformation(SMBiosProcessorInformation::new(self))
            }
            SMBiosMemoryControllerInformation::STRUCT_TYPE => {
                DefinedStruct::MemoryControllerInformation(SMBiosMemoryControllerInformation::new(
                    self,
                ))
            }
            SMBiosMemoryModuleInformation::STRUCT_TYPE => {
                DefinedStruct::MemoryModuleInformation(SMBiosMemoryModuleInformation::new(self))
            }
            SMBiosCacheInformation::STRUCT_TYPE => {
                DefinedStruct::CacheInformation(SMBiosCacheInformation::new(self))
            }
            SMBiosPortConnectorInformation::STRUCT_TYPE => {
                DefinedStruct::PortConnectorInformation(SMBiosPortConnectorInformation::new(self))
            }
            SMBiosSystemSlot::STRUCT_TYPE => DefinedStruct::SystemSlot(SMBiosSystemSlot::new(self)),
            SMBiosOnBoardDeviceInformation::STRUCT_TYPE => {
                DefinedStruct::OnBoardDeviceInformation(SMBiosOnBoardDeviceInformation::new(self))
            }
            SMBiosOemStrings::STRUCT_TYPE => DefinedStruct::OemStrings(SMBiosOemStrings::new(self)),
            SMBiosSystemConfigurationOptions::STRUCT_TYPE => {
                DefinedStruct::SystemConfigurationOptions(SMBiosSystemConfigurationOptions::new(
                    self,
                ))
            }
            SMBiosBiosLanguageInformation::STRUCT_TYPE => {
                DefinedStruct::LanguageInformation(SMBiosBiosLanguageInformation::new(self))
            }
            SMBiosGroupAssociations::STRUCT_TYPE => {
                DefinedStruct::GroupAssociations(SMBiosGroupAssociations::new(self))
            }
            SMBiosSystemEventLog::STRUCT_TYPE => {
                DefinedStruct::EventLog(SMBiosSystemEventLog::new(self))
            }
            SMBiosPhysicalMemoryArray::STRUCT_TYPE => {
                DefinedStruct::PhysicalMemoryArray(SMBiosPhysicalMemoryArray::new(self))
            }
            SMBiosMemoryDevice::STRUCT_TYPE => {
                DefinedStruct::MemoryDevice(SMBiosMemoryDevice::new(self))
            }
            SMBiosMemoryErrorInformation32::STRUCT_TYPE => {
                DefinedStruct::MemoryErrorInformation32Bit(SMBiosMemoryErrorInformation32::new(
                    self,
                ))
            }
            SMBiosMemoryArrayMappedAddress::STRUCT_TYPE => {
                DefinedStruct::MemoryArrayMappedAddress(SMBiosMemoryArrayMappedAddress::new(self))
            }
            SMBiosMemoryDeviceMappedAddress::STRUCT_TYPE => {
                DefinedStruct::MemoryDeviceMappedAddress(SMBiosMemoryDeviceMappedAddress::new(self))
            }
            SMBiosBuiltInPointingDevice::STRUCT_TYPE => {
                DefinedStruct::BuiltInPointingDevice(SMBiosBuiltInPointingDevice::new(self))
            }
            SMBiosPortableBattery::STRUCT_TYPE => {
                DefinedStruct::PortableBattery(SMBiosPortableBattery::new(self))
            }
            SMBiosSystemReset::STRUCT_TYPE => {
                DefinedStruct::SystemReset(SMBiosSystemReset::new(self))
            }
            SMBiosHardwareSecurity::STRUCT_TYPE => {
                DefinedStruct::HardwareSecurity(SMBiosHardwareSecurity::new(self))
            }
            SMBiosSystemPowerControls::STRUCT_TYPE => {
                DefinedStruct::SystemPowerControls(SMBiosSystemPowerControls::new(self))
            }
            SMBiosVoltageProbe::STRUCT_TYPE => {
                DefinedStruct::VoltageProbe(SMBiosVoltageProbe::new(self))
            }
            SMBiosCoolingDevice::STRUCT_TYPE => {
                DefinedStruct::CoolingDevice(SMBiosCoolingDevice::new(self))
            }
            SMBiosTemperatureProbe::STRUCT_TYPE => {
                DefinedStruct::TemperatureProbe(SMBiosTemperatureProbe::new(self))
            }
            SMBiosElectricalCurrentProbe::STRUCT_TYPE => {
                DefinedStruct::ElectricalCurrentProbe(SMBiosElectricalCurrentProbe::new(self))
            }
            SMBiosOutOfBandRemoteAccess::STRUCT_TYPE => {
                DefinedStruct::OutOfBandRemoteAccess(SMBiosOutOfBandRemoteAccess::new(self))
            }
            SMBiosBisEntryPoint::STRUCT_TYPE => {
                DefinedStruct::BisEntryPoint(SMBiosBisEntryPoint::new(self))
            }
            SMBiosSystemBootInformation::STRUCT_TYPE => {
                DefinedStruct::SystemBootInformation(SMBiosSystemBootInformation::new(self))
            }
            SMBiosMemoryErrorInformation64::STRUCT_TYPE => {
                DefinedStruct::MemoryErrorInformation64Bit(SMBiosMemoryErrorInformation64::new(
                    self,
                ))
            }
            SMBiosManagementDevice::STRUCT_TYPE => {
                DefinedStruct::ManagementDevice(SMBiosManagementDevice::new(self))
            }
            SMBiosManagementDeviceComponent::STRUCT_TYPE => {
                DefinedStruct::ManagementDeviceComponent(SMBiosManagementDeviceComponent::new(self))
            }
            SMBiosManagementDeviceThresholdData::STRUCT_TYPE => {
                DefinedStruct::ManagementDeviceThresholdData(
                    SMBiosManagementDeviceThresholdData::new(self),
                )
            }
            SMBiosMemoryChannel::STRUCT_TYPE => {
                DefinedStruct::MemoryChannel(SMBiosMemoryChannel::new(self))
            }
            SMBiosIpmiDeviceInformation::STRUCT_TYPE => {
                DefinedStruct::IpmiDeviceInformation(SMBiosIpmiDeviceInformation::new(self))
            }
            SMBiosSystemPowerSupply::STRUCT_TYPE => {
                DefinedStruct::SystemPowerSupply(SMBiosSystemPowerSupply::new(self))
            }
            SMBiosAdditionalInformation::STRUCT_TYPE => {
                DefinedStruct::AdditionalInformation(SMBiosAdditionalInformation::new(self))
            }
            SMBiosOnboardDevicesExtendedInformation::STRUCT_TYPE => {
                DefinedStruct::OnboardDevicesExtendedInformation(
                    SMBiosOnboardDevicesExtendedInformation::new(self),
                )
            }
            SMBiosManagementControllerHostInterface::STRUCT_TYPE => {
                DefinedStruct::ManagementControllerHostInterface(
                    SMBiosManagementControllerHostInterface::new(self),
                )
            }
            SMBiosTpmDevice::STRUCT_TYPE => DefinedStruct::TpmDevice(SMBiosTpmDevice::new(self)),
            SMBiosProcessorAdditionalInformation::STRUCT_TYPE => {
                DefinedStruct::ProcessorAdditionalInformation(
                    SMBiosProcessorAdditionalInformation::new(self),
                )
            }
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
            .field("strings", &self.strings)
            .finish()
    }
}

/// # SMBIOS Strings
///
/// The strings part/section of a structure
pub struct Strings<'a> {
    strings: Vec<&'a [u8]>,
    current_string_index: usize,
}

impl<'a> Strings<'a> {
    fn new(string_area: &[u8]) -> Strings {
        Strings {
            strings: {
                if string_area == &[] {
                    vec![]
                } else {
                    string_area.split(|num| *num == 0).into_iter().collect()
                }
            },
            current_string_index: 0,
        }
    }

    fn reset(&mut self) {
        self.current_string_index = 0;
    }

    fn get_string(&self, index: u8) -> Option<String> {
        let index_usize = index as usize;

        if index_usize == 0 || index_usize > self.strings.len() {
            // BIOS strings are 1 based indexing, ignore bad input
            return None;
        }

        // TODO: "*x as char" is not ISO-8859-1.  This should be made ISO-8859-1.
        Some(
            self.strings[index_usize - 1]
                .into_iter()
                .map(|x| *x as char)
                .collect(),
        )
    }
}

impl<'a> Iterator for Strings<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_string_index == self.strings.len() {
            self.reset();
            return None;
        }

        // TODO: "*x as char" is not ISO-8859-1.  This should be made ISO-8859-1.
        let result: String = self.strings[self.current_string_index]
            .into_iter()
            .map(|x| *x as char)
            .collect();
        self.current_string_index = self.current_string_index + 1;

        Some(result)
    }
}

impl<'a> IntoIterator for &'a Strings<'a> {
    type Item = String;
    type IntoIter = Strings<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Strings {
            strings: self.strings.clone(),
            current_string_index: 0,
        }
    }
}

impl<'a> fmt::Debug for Strings<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

/// # SMBIOS Header
///
/// The header part/section of a structure
pub struct Header<'a> {
    data: &'a [u8],
}

impl fmt::Debug for Header<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Header>())
            .field("struct_type", &self.struct_type())
            .field("length", &self.length())
            .field("handle", &self.handle())
            .finish()
    }
}

impl<'a> Header<'a> {
    const SIZE: usize = 4;
    const LENGTH_OFFSET: usize = 1;

    fn new(data: &'a [u8]) -> Self {
        assert!(
            data.len() == Self::SIZE,
            "Header must be 4 bytes in length, 1 for struct_type, 1 for length, and 2 for handle."
        );
        Header { data }
    }

    /// The type of SMBIOS structure
    pub fn struct_type(&self) -> u8 {
        self.data[0] // struct_type is 1 byte at offset 0
    }

    /// The length of the structure not including the strings part/section
    pub fn length(&self) -> u8 {
        self.data[Self::LENGTH_OFFSET] // length is 1 byte at offset 1
    }

    /// The handle of this structure instance
    pub fn handle(&self) -> Handle {
        // handle is 2 bytes at offset 2
        Handle(u16::from_le_bytes(
            self.data[2..4]
                .try_into()
                .expect("array length does not match type width"),
        ))
    }
}

/// # SMBIOS Raw Table Data
///
/// Contains the raw data of BIOS and provides iteration of
/// the structures contained within the raw data.
pub struct SMBiosTableData<'a> {
    data: &'a [u8],
}

impl<'a> SMBiosTableData<'a> {
    /// Creates a wrapper around raw SMBIOS data
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
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

/// # SMBIOS Structure
///
/// A type implementing this trait provies a representation of an SMBIOS type.
pub trait SMBiosStruct<'a> {
    /// The SMBIOS structure type
    ///
    /// Example: System Information (Type 1) this is set to 1.
    const STRUCT_TYPE: u8;

    /// Creates a new instance of the implementing SMBIOS type
    fn new(parts: &'a SMBiosStructParts<'a>) -> Self;

    /// Contains the standard parts/sections of the implementing SMBIOS type.
    fn parts(&self) -> &'a SMBiosStructParts<'a>;
}

/// # Iterator of [SMBiosTableData]
///
/// Allows iteration of [SMBiosTableData] and returns [SMBiosStructParts].
pub struct RawStructIterator<'a> {
    data: &'a [u8],
    current_index: usize,
}

impl<'a> RawStructIterator<'a> {
    /// Creates an instance of this iterator
    pub fn new(data: &'a [u8]) -> Self {
        RawStructIterator {
            data: data,
            current_index: 0,
        }
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
            || self.data[len - 1] != 0)
        {
            // Last byte should be zero and it is not
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
        let mut a: bool;
        let mut b = true;
        loop {
            a = self.data[next_index] != 0;
            next_index = next_index + 1;
            if a || b {
                b = self.data[next_index] != 0;
                next_index = next_index + 1;
            }
            if !(a || b) {
                break;
            }
        }

        let previous_index = self.current_index;
        self.current_index = next_index;

        match self.data.get(previous_index..self.current_index) {
            Some(val) => Some(SMBiosStructParts::new(val)),
            None => None,
        }
    }
}
