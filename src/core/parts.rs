use crate::*;

/// # The three basic parts of an SMBIOS structure
///
/// Every SMBIOS structure contains three parts or sections: A header,
/// structure data, and string data.
pub struct SMBiosStructParts<'a> {
    /// The raw data, including header, fields, and strings
    pub raw: &'a [u8],

    /// The [Header] of the structure
    pub header: Header<'a>,

    /// The raw data for the header and fields
    ///
    /// _fields_ is used by the get_field_*() functions. _fields_ does not
    /// include _strings_; therefore, preventing accidentally retrieving
    /// data from the _strings_ area.  This avoids a need to check
    /// _header.length()_ during field retrieval.
    ///
    /// Note: A better design is for this to only hold the fields, however,
    /// that will shift field offsets given in code by 4 (the header size).
    /// The SMBIOS specification gives offsets relative to the start of the
    /// header, and therefore maintaining this library code is easier to
    /// keep the header.
    ///
    /// An alternative would be to make the get_field_*() functions adjust
    /// for the header offset though this adds a small cost to every field
    /// retrieval in comparison to just keeping an extra 4 bytes for every
    /// structure.
    pub fields: &'a [u8],

    /// The strings of the structure
    pub strings: Strings<'a>,
}

impl<'a> SMBiosStructParts<'a> {
    /// Creates a structure instance of the given byte array slice
    pub fn new(raw: &'a [u8]) -> Self {
        SMBiosStructParts {
            raw: raw,
            header: Header::new(
                raw.get(..Header::SIZE)
                    .expect("A minimum of Header::SIZE bytes are required."),
            ),
            fields: raw
                .get(..SMBiosStructParts::header_length(raw))
                .unwrap_or(&[]),
            strings: {
                Strings::new(
                    raw.get(SMBiosStructParts::header_length(raw)..raw.len() - 2)
                        .unwrap_or(&[]),
                )
            },
        }
    }

    fn header_length(raw: &[u8]) -> usize {
        raw.get(Header::LENGTH_OFFSET..Header::LENGTH_OFFSET + 1)
            .unwrap_or(&[0])[0] as usize
    }

    /// Retrieve a byte at the given offset from the structure's data section
    pub fn get_field_byte(&self, offset: usize) -> Option<u8> {
        match self.fields.get(offset..offset + 1) {
            Some(val) => Some(val[0]),
            None => None,
        }
    }

    /// Retrieve a WORD at the given offset from the structure's data section
    pub fn get_field_word(&self, offset: usize) -> Option<u16> {
        match self.fields.get(offset..offset + 2) {
            Some(val) => Some(u16::from_le_bytes(
                val.try_into()
                    .expect("array length does not match type width"),
            )),
            None => None,
        }
    }

    /// Retrieve a [Handle] at the given offset from the structure's data section
    pub fn get_field_handle(&self, offset: usize) -> Option<Handle> {
        match self.fields.get(offset..offset + 2) {
            Some(val) => Some(Handle(u16::from_le_bytes(
                val.try_into()
                    .expect("array length does not match type width"),
            ))),
            None => None,
        }
    }

    /// Retrieve a DWORD at the given offset from the structure's data section
    pub fn get_field_dword(&self, offset: usize) -> Option<u32> {
        match self.fields.get(offset..offset + 4) {
            Some(val) => Some(u32::from_le_bytes(
                val.try_into()
                    .expect("array length does not match type width"),
            )),
            None => None,
        }
    }

    /// Retrieve a QWORD at the given offset from the structure's data section
    pub fn get_field_qword(&self, offset: usize) -> Option<u64> {
        match self.fields.get(offset..offset + 8) {
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
        return self.raw.get(start_index..end_index);
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
