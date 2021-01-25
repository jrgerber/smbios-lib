use super::*;

/// # Management Controller Host Interface (Type 42)
///
/// The information in this structure defines the attributes of a Management Controller Host Interface that is
/// not discoverable by "Plug and Play" mechanisms. The Type 42 structure can
/// be used to describe a physical management controller host interface and one or more protocols that
/// share that interface.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
///
/// In SMBIOS 3.2, a Change Request is applied to this structure to add the missing information that is
/// needed to parse the structure completely. The addition of the Interface Type Specific Data Length field
/// may cause parser (prior to SMBIOS 3.2) compatibility issue when Interface Type = OEM. Prior to
/// SMBIOS 3.2, when Interface Type = OEM, the first four bytes following the Interface Type field is the
/// IANA-assigned vendor ID.
pub struct SMBiosManagementControllerHostInterface<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementControllerHostInterface<'a> {
    const STRUCT_TYPE: u8 = 42u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosManagementControllerHostInterface<'a> {
    /// Management Controller Interface Type
    pub fn interface_type(&self) -> Option<HostInterfaceTypeData> {
        self.parts
            .get_field_byte(0x04)
            .and_then(|raw| Some(HostInterfaceTypeData::from(raw)))
    }

    /// Interface Type Specific Data Length
    pub fn interface_type_specific_data_length(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    // fn interface_type_specific_data(&self) -> Option<FixMe> {
    //     self.parts.get_field_undefined(0x06)
    // }
}

impl fmt::Debug for SMBiosManagementControllerHostInterface<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosManagementControllerHostInterface>())
            .field("header", &self.parts.header)
            .field("interface_type", &self.interface_type())
            .field(
                "interface_type_specific_data_length",
                &self.interface_type_specific_data_length(),
            )
            // .field("interface_type_specific_data", &self.interface_type_specific_data())
            .finish()
    }
}

/// # Management Controller Host Interface Types
///
/// 00h-3Fh: MCTP Host Interfaces - Refer to [DSP0239](https://www.dmtf.org/sites/default/files/standards/documents/DSP0239_1.1.0.pdf) for the definition and assignment of MCTP host interface type values
/// 40h: Network Host Interface - Refer to [DSP0270](https://www.dmtf.org/sites/default/files/DSP0270_1.0.1.pdf) for the definition and details of the Network Host Interface type
/// F0h: OEM-defined
/// All others: Reserved
#[derive(Debug, PartialEq, Eq)]
pub enum HostInterfaceType {
    /// KCS: Keyboard Controller Style
    ///
    /// Refer to _Intelligent Platform
    /// Management Interface Specification_ Section 9 Keyboard Controller
    /// Style (KCS) Interface
    KeyboardControllerStyle,
    /// 8250 UART Register Compatible
    Uart8250,
    /// 16450 UART Register Compatible
    Uart16450,
    /// 16550/16550A UART Register Compatible
    Uart16550,
    /// 16650/16650A UART Register Compatible
    Uart16650,
    /// 16750/16750A UART Register Compatible
    Uart16750,
    /// 16850/16850A UART Register Compatible
    Uart16850,
    /// Redfish Network Host Interface
    ///
    /// See [DSP0270](https://www.dmtf.org/sites/default/files/DSP0270_1.0.1.pdf) Redfish Host Interface Specification
    NetworkHostInterface,
    /// OEM Defined
    OemDefined,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Management Controller Host Interface Type Data
pub struct HostInterfaceTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [HostInterfaceType] value
    pub value: HostInterfaceType,
}

impl fmt::Debug for HostInterfaceTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<HostInterfaceType>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for HostInterfaceTypeData {
    type Target = HostInterfaceType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for HostInterfaceType {
    fn from(raw: u8) -> Self {
        match raw {
            0x02 => HostInterfaceType::KeyboardControllerStyle,
            0x03 => HostInterfaceType::Uart8250,
            0x04 => HostInterfaceType::Uart16450,
            0x05 => HostInterfaceType::Uart16550,
            0x06 => HostInterfaceType::Uart16650,
            0x07 => HostInterfaceType::Uart16750,
            0x08 => HostInterfaceType::Uart16850,
            0x40 => HostInterfaceType::NetworkHostInterface,
            0xF0 => HostInterfaceType::OemDefined,
            _ => HostInterfaceType::None,
        }
    }
}

impl From<u8> for HostInterfaceTypeData {
    fn from(raw: u8) -> Self {
        Self {
            raw,
            value: HostInterfaceType::from(raw),
        }
    }
}
