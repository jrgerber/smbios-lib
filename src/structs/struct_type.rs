use crate::*;
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
