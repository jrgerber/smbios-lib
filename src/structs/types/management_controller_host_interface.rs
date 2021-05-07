use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeSeq, ser::SerializeStruct, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

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
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosManagementControllerHostInterface<'a> {
    const STRUCT_TYPE: u8 = 42u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosManagementControllerHostInterface<'a> {
    const INTERFACE_TYPE_OFFSET: usize = 4usize;
    const SPECIFIC_DATA_LENGTH_OFFSET: usize = 5usize;
    const SPECIFIC_DATA_OFFSET: usize = 6usize;
    const PROTOCOL_RECORDS_RELATIVE_OFFSET: usize = 7usize;

    /// Management Controller Interface Type
    pub fn interface_type(&self) -> Option<HostInterfaceTypeData> {
        self.parts
            .get_field_byte(Self::INTERFACE_TYPE_OFFSET)
            .map(|raw| HostInterfaceTypeData::from(raw))
    }

    /// Interface Type Specific Data Length
    pub fn interface_type_specific_data_length(&self) -> Option<u8> {
        self.parts.get_field_byte(Self::SPECIFIC_DATA_LENGTH_OFFSET)
    }

    /// Management Controller Host Interface Data as specified by the Interface Type
    ///
    /// This field has a minimum of four bytes.
    ///
    /// If interface type = OEM, the first four bytes are the vendor ID (MSB first), as assigned by the Internet Assigned Numbers Authority (IANA).
    ///
    /// This format uses the "Enterprise Number" that is assigned and maintained by IANA (www.iana.org) as the means of identifying a particular vendor, company, or organization.
    fn interface_type_specific_data(&self) -> Option<&[u8]> {
        self.interface_type_specific_data_length()
            .and_then(|length| {
                self.parts().get_field_data(
                    Self::SPECIFIC_DATA_OFFSET,
                    Self::SPECIFIC_DATA_OFFSET + length as usize,
                )
            })
    }

    /// X number of Protocol Records for this Host Interface Type
    pub fn number_of_protocol_records(&self) -> Option<u8> {
        self.interface_type_specific_data_length()
            .and_then(|length| {
                self.parts()
                    .get_field_byte(Self::SPECIFIC_DATA_OFFSET + length as usize)
            })
    }

    /// private function for calculating the protocol records offset
    fn protocol_records_offset(&self) -> Option<usize> {
        self.interface_type_specific_data_length()
            .and_then(|length| Some(Self::PROTOCOL_RECORDS_RELATIVE_OFFSET + length as usize))
    }

    /// Protocol Records
    pub fn protocol_record_iterator(&self) -> ProtocolRecordIterator<'_> {
        ProtocolRecordIterator::new(self)
    }
}

impl fmt::Debug for SMBiosManagementControllerHostInterface<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<
            SMBiosManagementControllerHostInterface<'_>,
        >())
        .field("header", &self.parts.header)
        .field("interface_type", &self.interface_type())
        .field(
            "interface_type_specific_data_length",
            &self.interface_type_specific_data_length(),
        )
        .field(
            "interface_type_specific_data",
            &self.interface_type_specific_data(),
        )
        .field(
            "number_of_protocol_records",
            &self.number_of_protocol_records(),
        )
        .field("protocol_record_iterator", &self.protocol_record_iterator())
        .finish()
    }
}

impl Serialize for SMBiosManagementControllerHostInterface<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state =
            serializer.serialize_struct("SMBiosManagementControllerHostInterface", 6)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("interface_type", &self.interface_type())?;
        state.serialize_field(
            "interface_type_specific_data_length",
            &self.interface_type_specific_data_length(),
        )?;
        state.serialize_field(
            "interface_type_specific_data",
            &self.interface_type_specific_data(),
        )?;
        state.serialize_field(
            "number_of_protocol_records",
            &self.number_of_protocol_records(),
        )?;
        state.serialize_field("protocol_record_iterator", &self.protocol_record_iterator())?;
        state.end()
    }
}

/// # Management Controller Host Interface Types
///
/// 00h-3Fh: MCTP Host Interfaces - Refer to [DSP0239](https://www.dmtf.org/sites/default/files/standards/documents/DSP0239_1.1.0.pdf) for the definition and assignment of MCTP host interface type values
/// 40h: Network Host Interface - Refer to [DSP0270](https://www.dmtf.org/sites/default/files/DSP0270_1.0.1.pdf) for the definition and details of the Network Host Interface type
/// F0h: OEM-defined
/// All others: Reserved
#[derive(Serialize, Debug, PartialEq, Eq)]
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<HostInterfaceType>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for HostInterfaceTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HostInterfaceTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for HostInterfaceTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            HostInterfaceType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
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

/// # Management Controller Host Interface - Protocol Types
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum HostProtocolType {
    /// IPMI: Intelligent Platform Management Interface
    ///
    /// Refer to IPMI Appendix C1
    Ipmi,
    /// MCTP: Management Component Transport Protocol
    ///
    /// Refer to DSP0236 for the definition and details of the MCTP protocol type
    Mctp,
    /// Redfish over IP
    ///
    /// Refer to [DSP0270](https://www.dmtf.org/sites/default/files/DSP0270_1.0.1.pdf) for the definition and details of the Redfish over IP protocol type
    RedfishOverIP,
    /// OEM Defined
    OemDefined,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for HostProtocolType {
    fn from(raw: u8) -> Self {
        match raw {
            0x02 => HostProtocolType::Ipmi,
            0x03 => HostProtocolType::Mctp,
            0x04 => HostProtocolType::RedfishOverIP,
            0xF0 => HostProtocolType::OemDefined,
            _ => HostProtocolType::None,
        }
    }
}

/// # Management Controller Host Interface - Protocol Types Data
pub struct HostProtocolTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [HostProtocolType] value
    pub value: HostProtocolType,
}

impl From<u8> for HostProtocolTypeData {
    fn from(raw: u8) -> Self {
        Self {
            raw,
            value: HostProtocolType::from(raw),
        }
    }
}

impl fmt::Debug for HostProtocolTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<HostProtocolTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Serialize for HostProtocolTypeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HostProtocolTypeData", 2)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl fmt::Display for HostProtocolTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            HostProtocolType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for HostProtocolTypeData {
    type Target = HostProtocolType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Protocol Record Data contained within [SMBiosManagementControllerHostInterface]
pub struct ProtocolRecord<'a> {
    host_interface: &'a SMBiosManagementControllerHostInterface<'a>,
    entry_offset: usize,
}

impl<'a> ProtocolRecord<'a> {
    const PROTOCOL_TYPE_OFFSET: usize = 0usize;
    const SPECIFIC_DATA_LENGTH_OFFSET: usize = 1usize;
    const SPECIFIC_DATA_OFFSET: usize = 2usize;
    fn new(
        host_interface: &'a SMBiosManagementControllerHostInterface<'a>,
        entry_offset: usize,
    ) -> Self {
        Self {
            host_interface,
            entry_offset,
        }
    }

    /// Protocol Type
    pub fn protocol_type(&self) -> Option<HostProtocolTypeData> {
        self.host_interface
            .parts()
            .get_field_byte(self.entry_offset + Self::PROTOCOL_TYPE_OFFSET)
            .map(|raw| HostProtocolTypeData::from(raw))
    }

    /// Protocol Type Specific Data Length
    pub fn protocol_type_specific_data_length(&self) -> Option<u8> {
        self.host_interface
            .parts()
            .get_field_byte(self.entry_offset + Self::SPECIFIC_DATA_LENGTH_OFFSET)
    }

    /// Protocol Type Specific Data
    pub fn protocol_type_specific_data(&self) -> Option<&[u8]> {
        let start_index = self.entry_offset + Self::SPECIFIC_DATA_OFFSET;
        self.protocol_type_specific_data_length()
            .and_then(|length| {
                self.host_interface
                    .parts()
                    .get_field_data(start_index, start_index + length as usize)
            })
    }
}

impl fmt::Debug for ProtocolRecord<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<ProtocolRecord<'_>>())
            .field("protocol_type", &self.protocol_type())
            .field(
                "protocol_type_specific_data_length",
                &self.protocol_type_specific_data_length(),
            )
            .field(
                "protocol_type_specific_data",
                &self.protocol_type_specific_data(),
            )
            .finish()
    }
}

impl Serialize for ProtocolRecord<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProtocolRecord", 3)?;
        state.serialize_field("protocol_type", &self.protocol_type())?;
        state.serialize_field(
            "protocol_type_specific_data_length",
            &self.protocol_type_specific_data_length(),
        )?;
        state.serialize_field(
            "protocol_type_specific_data",
            &self.protocol_type_specific_data(),
        )?;
        state.end()
    }
}

/// Iterates over the [ProtocolRecord] entries contained within [SMBiosManagementControllerHostInterface]
pub struct ProtocolRecordIterator<'a> {
    data: &'a SMBiosManagementControllerHostInterface<'a>,
    start_index: usize,
    current_index: usize,
    current_entry: u8,
    number_of_entries: u8,
}

impl<'a> ProtocolRecordIterator<'a> {
    fn new(data: &'a SMBiosManagementControllerHostInterface<'a>) -> Self {
        let start_index = data.protocol_records_offset().unwrap_or(0);
        ProtocolRecordIterator {
            data,
            start_index,
            current_index: start_index,
            current_entry: 0,
            number_of_entries: data.number_of_protocol_records().unwrap_or(0),
        }
    }

    fn reset(&mut self) {
        self.current_index = self.start_index;
        self.current_entry = 0;
    }
}

impl<'a> IntoIterator for &'a ProtocolRecordIterator<'a> {
    type Item = ProtocolRecord<'a>;
    type IntoIter = ProtocolRecordIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ProtocolRecordIterator {
            data: self.data,
            start_index: self.start_index,
            current_index: self.start_index,
            current_entry: 0,
            number_of_entries: self.data.number_of_protocol_records().unwrap_or(0),
        }
    }
}

impl<'a> Iterator for ProtocolRecordIterator<'a> {
    type Item = ProtocolRecord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == self.number_of_entries {
            self.reset();
            return None;
        }

        match self
            .data
            .parts()
            .get_field_byte(self.current_index + ProtocolRecord::SPECIFIC_DATA_LENGTH_OFFSET)
        {
            Some(specific_data_length) => {
                // Length of 0 would result in an endless loop because we would never advance to the next entry
                if specific_data_length == 0 {
                    self.reset();
                    return None;
                }

                let next_index = self.current_index
                    + specific_data_length as usize
                    + ProtocolRecord::SPECIFIC_DATA_OFFSET;
                match self
                    .data
                    .parts()
                    .get_field_data(self.current_index, next_index)
                {
                    Some(_entry_block) => {
                        let result = ProtocolRecord::new(self.data, self.current_index);
                        self.current_index = next_index;
                        self.current_entry += 1;
                        Some(result)
                    }
                    None => {
                        self.reset();
                        None
                    }
                }
            }
            None => {
                self.reset();
                None
            }
        }
    }
}

impl<'a> fmt::Debug for ProtocolRecordIterator<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.into_iter()).finish()
    }
}

impl<'a> Serialize for ProtocolRecordIterator<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let records: Vec<ProtocolRecord<'_>> = self.into_iter().collect();
        let mut seq = serializer.serialize_seq(Some(records.len()))?;
        for e in records {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type42 = vec![
            42u8, 0x13, 0x24, 0x00, 0x02, 0x04, 0x01, 0x02, 0x03, 0x04, 0x02, 0x03, 0x01, 0xDD,
            0x02, 0x01, 0xEE, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type42);
        let test_struct = SMBiosManagementControllerHostInterface::new(&parts);

        assert_eq!(
            *test_struct.interface_type().unwrap(),
            HostInterfaceType::KeyboardControllerStyle
        );

        assert_eq!(test_struct.interface_type_specific_data_length(), Some(4));
        assert_eq!(
            test_struct.interface_type_specific_data(),
            [1u8, 2, 3, 4].get(0..4)
        );
        assert_eq!(test_struct.number_of_protocol_records(), Some(2));

        let mut iterator = test_struct.protocol_record_iterator().into_iter();
        let first = iterator.next().unwrap();
        assert_eq!(*first.protocol_type().unwrap(), HostProtocolType::Mctp);
        let second = iterator.next().unwrap();
        assert_eq!(*second.protocol_type().unwrap(), HostProtocolType::Ipmi);
        assert!(iterator.next().is_none());
    }
}
