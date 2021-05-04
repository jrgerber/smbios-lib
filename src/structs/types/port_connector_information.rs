use crate::{SMBiosStruct, UndefinedStruct};
use std::{fmt, ops::Deref};

/// # Port Connector Information (Type 8)
///
/// The information in this structure defines the attributes of a system port connector
/// (for example, parallel, serial, keyboard, or mouse ports). The port’s type and connector information are
/// provided. One structure is present for each port provided by the system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosPortConnectorInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosPortConnectorInformation<'a> {
    const STRUCT_TYPE: u8 = 8u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosPortConnectorInformation<'a> {
    ///  Internal reference designator, that is,
    /// internal to the system enclosure
    ///
    /// EXAMPLE: "J101"
    pub fn internal_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Internal connector type
    pub fn internal_connector_type(&self) -> Option<PortInformationConnectorTypeData> {
        self.parts
            .get_field_byte(0x05)
            .map(|raw| PortInformationConnectorTypeData::from(raw))
    }

    /// External reference designation,
    /// external to the system enclosure
    ///
    /// EXAMPLE: "COM A"
    pub fn external_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// External connector type
    pub fn external_connector_type(&self) -> Option<PortInformationConnectorTypeData> {
        self.parts
            .get_field_byte(0x07)
            .map(|raw| PortInformationConnectorTypeData::from(raw))
    }

    /// Describes the function of the port
    pub fn port_type(&self) -> Option<PortInformationPortTypeData> {
        self.parts
            .get_field_byte(0x08)
            .map(|raw| PortInformationPortTypeData::from(raw))
    }
}

impl fmt::Debug for SMBiosPortConnectorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPortConnectorInformation<'_>>())
            .field("header", &self.parts.header)
            .field(
                "internal_reference_designator",
                &self.internal_reference_designator(),
            )
            .field("internal_connector_type", &self.internal_connector_type())
            .field(
                "external_reference_designator",
                &self.external_reference_designator(),
            )
            .field("external_connector_type", &self.external_connector_type())
            .field("port_type", &self.port_type())
            .finish()
    }
}

/// # Port Information - Connector Types Data
pub struct PortInformationConnectorTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [PortInformationConnectorType] value
    pub value: PortInformationConnectorType,
}

impl fmt::Debug for PortInformationConnectorTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<PortInformationConnectorTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl fmt::Display for PortInformationConnectorTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            PortInformationConnectorType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for PortInformationConnectorTypeData {
    type Target = PortInformationConnectorType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Port Information - Connector Types
#[derive(Debug, PartialEq, Eq)]
pub enum PortInformationConnectorType {
    /// There is No Connector
    NoConnector,
    /// Centronics
    Centronics,
    /// Mini Centronics
    MiniCentronics,
    /// Proprietary
    Proprietary,
    /// DB-25 pin male
    DB25PinMale,
    /// DB-25 pin female
    DB25PinFemale,
    /// DB-15 pin male
    DB15PinMale,
    /// DB-15 pin female
    DB15PinFemale,
    /// DB-9 pin male
    DB9PinMale,
    /// DB-9 pin female
    DB8PinFemale,
    /// RJ-11
    RJ11,
    /// RJ-45
    RJ45,
    /// 50-pin MiniSCSI
    MiniScsi50Pin,
    /// Mini-DIN
    MiniDin,
    /// Micro-DIN
    MicroDin,
    /// PS/2
    Ps2,
    /// Infrared
    Infrared,
    /// HP-HIL
    HpHil,
    /// Access Bus (USB)
    AccessBusUsb,
    /// SSA SCSI
    SsaScsi,
    /// Circular DIN-8 male
    CircularDin8Male,
    /// Circular DIN-8 female
    CircularDin8Female,
    /// On Board IDE
    OnBoardIde,
    /// On Board Floppy
    OnBoardFloppy,
    /// 9-pin Dual Inline (pin 10 cut)
    DualInline9Pin,
    /// 25-pin Dual Inline (pin 26 cut)
    DualInline25Pin,
    /// 50-pin Dual Inline
    DualInline50Pin,
    /// 68-pin Dual Inline
    DualInline68Pin,
    /// On Board Sound Input from CD-ROM
    OnBoardSoundInputCDRom,
    /// Mini-Centronics Type-14
    MiniCentronicsType14,
    /// Mini-Centronics Type-26
    MiniCentronicsTyp26,
    /// Mini-jack (headphones)
    MiniJackHeadphones,
    /// BNC
    Bnc,
    /// 1394
    Port1394,
    /// SAS/SATA Plug Receptacle
    SasSataPlugReceptacle,
    /// USB Type-C Receptacle
    UsbTypeCReceptacle,
    /// PC-98
    PC98,
    /// PC-98Hireso
    PC98Hireso,
    /// PC-H98
    PCH88,
    /// PC-98Note
    PC98Note,
    /// PC-98Full
    PC98Full,
    /// Other – Use Reference Designator Strings to supply information.
    Other,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for PortInformationConnectorTypeData {
    fn from(raw: u8) -> Self {
        PortInformationConnectorTypeData {
            value: match raw {
                0x00 => PortInformationConnectorType::NoConnector,
                0x01 => PortInformationConnectorType::Centronics,
                0x02 => PortInformationConnectorType::MiniCentronics,
                0x03 => PortInformationConnectorType::Proprietary,
                0x04 => PortInformationConnectorType::DB25PinMale,
                0x05 => PortInformationConnectorType::DB25PinFemale,
                0x06 => PortInformationConnectorType::DB15PinMale,
                0x07 => PortInformationConnectorType::DB15PinFemale,
                0x08 => PortInformationConnectorType::DB9PinMale,
                0x09 => PortInformationConnectorType::DB8PinFemale,
                0x0A => PortInformationConnectorType::RJ11,
                0x0B => PortInformationConnectorType::RJ45,
                0x0C => PortInformationConnectorType::MiniScsi50Pin,
                0x0D => PortInformationConnectorType::MiniDin,
                0x0E => PortInformationConnectorType::MicroDin,
                0x0F => PortInformationConnectorType::Ps2,
                0x10 => PortInformationConnectorType::Infrared,
                0x11 => PortInformationConnectorType::HpHil,
                0x12 => PortInformationConnectorType::AccessBusUsb,
                0x13 => PortInformationConnectorType::SsaScsi,
                0x14 => PortInformationConnectorType::CircularDin8Male,
                0x15 => PortInformationConnectorType::CircularDin8Female,
                0x16 => PortInformationConnectorType::OnBoardIde,
                0x17 => PortInformationConnectorType::OnBoardFloppy,
                0x18 => PortInformationConnectorType::DualInline9Pin,
                0x19 => PortInformationConnectorType::DualInline25Pin,
                0x1A => PortInformationConnectorType::DualInline50Pin,
                0x1B => PortInformationConnectorType::DualInline68Pin,
                0x1C => PortInformationConnectorType::OnBoardSoundInputCDRom,
                0x1D => PortInformationConnectorType::MiniCentronicsType14,
                0x1E => PortInformationConnectorType::MiniCentronicsTyp26,
                0x1F => PortInformationConnectorType::MiniJackHeadphones,
                0x20 => PortInformationConnectorType::Bnc,
                0x21 => PortInformationConnectorType::Port1394,
                0x22 => PortInformationConnectorType::SasSataPlugReceptacle,
                0x23 => PortInformationConnectorType::UsbTypeCReceptacle,
                0xA0 => PortInformationConnectorType::PC98,
                0xA1 => PortInformationConnectorType::PC98Hireso,
                0xA2 => PortInformationConnectorType::PCH88,
                0xA3 => PortInformationConnectorType::PC98Note,
                0xA4 => PortInformationConnectorType::PC98Full,
                0xFF => PortInformationConnectorType::Other,
                _ => PortInformationConnectorType::None,
            },
            raw,
        }
    }
}

/// # Port Types Data
pub struct PortInformationPortTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [PortInformationPortType] value
    pub value: PortInformationPortType,
}

impl fmt::Debug for PortInformationPortTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<PortInformationPortTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl fmt::Display for PortInformationPortTypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            PortInformationPortType::None => write!(f, "{}", &self.raw),
            _ => write!(f, "{:?}", &self.value),
        }
    }
}

impl Deref for PortInformationPortTypeData {
    type Target = PortInformationPortType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # Port Types
#[derive(Debug, PartialEq, Eq)]
pub enum PortInformationPortType {
    /// No Port
    NoPort,
    /// Parallel Port XT/AT Compatible
    ParallelPortXTATCompatible,
    /// Parallel Port PS/2
    ParallelPortPS2,
    /// Parallel Port ECP
    ParallelPortEcp,
    /// Parallel Port EPP
    ParallelPortEpp,
    /// Parallel Port ECP/EPP
    ParallelPortEcpEpp,
    /// Serial Port XT/AT Compatible
    SerialPortXTATCompatible,
    /// Serial Port 16450 Compatible
    SerialPort16450Compatible,
    /// Serial Port 16550 Compatible
    SerialPort16550Compatible,
    /// Serial Port 16550A Compatible
    SerialPort16550ACompatible,
    /// SCSI Port
    ScsiPort,
    /// MIDI Port
    MidiPort,
    /// Joy Stick Port
    JoyStickPort,
    /// Keyboard Port
    KeyboardPort,
    /// Mouse Port
    MousePort,
    /// SSA SCSI
    SsaScsi,
    /// USB
    Usb,
    /// FireWire (IEEE P1394)
    Firewire,
    /// PCMCIA Type I2
    PcmciaTypeI,
    /// PCMCIA Type II
    PcmcialTypeII,
    /// PCMCIA Type III
    PcmciaTypeIii,
    /// Cardbus
    Cardbus,
    /// Access Bus Port
    AccessBusPort,
    /// SCSI II
    ScsiII,
    /// SCSI Wide
    ScsiWide,
    /// PC-98
    PC98,
    /// PC-98-Hireso
    PC98Hireso,
    /// PC-H98
    PCH98,
    /// Video Port
    VideoPort,
    /// Audio Port
    AudioPort,
    /// Modem Port
    ModemPort,
    /// Network Port
    NetworkPort,
    /// SATA
    Sata,
    /// SAS
    Sas,
    /// MFDP (Multi-Function Display Port)
    Mfdp,
    /// Thunderbolt
    Thunderbolt,
    /// 8251 Compatible
    Port8251Compatible,
    /// 8251 FIFO Compatible
    Port8251FifoCompatible,
    /// Other
    Other,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for PortInformationPortTypeData {
    fn from(raw: u8) -> Self {
        PortInformationPortTypeData {
            value: match raw {
                0x00 => PortInformationPortType::NoPort,
                0x01 => PortInformationPortType::ParallelPortXTATCompatible,
                0x02 => PortInformationPortType::ParallelPortPS2,
                0x03 => PortInformationPortType::ParallelPortEcp,
                0x04 => PortInformationPortType::ParallelPortEpp,
                0x05 => PortInformationPortType::ParallelPortEcpEpp,
                0x06 => PortInformationPortType::SerialPortXTATCompatible,
                0x07 => PortInformationPortType::SerialPort16450Compatible,
                0x08 => PortInformationPortType::SerialPort16550Compatible,
                0x09 => PortInformationPortType::SerialPort16550ACompatible,
                0x0A => PortInformationPortType::ScsiPort,
                0x0B => PortInformationPortType::MidiPort,
                0x0C => PortInformationPortType::JoyStickPort,
                0x0D => PortInformationPortType::KeyboardPort,
                0x0E => PortInformationPortType::MousePort,
                0x0F => PortInformationPortType::SsaScsi,
                0x10 => PortInformationPortType::Usb,
                0x11 => PortInformationPortType::Firewire,
                0x12 => PortInformationPortType::PcmciaTypeI,
                0x13 => PortInformationPortType::PcmcialTypeII,
                0x14 => PortInformationPortType::PcmciaTypeIii,
                0x15 => PortInformationPortType::Cardbus,
                0x16 => PortInformationPortType::AccessBusPort,
                0x17 => PortInformationPortType::ScsiII,
                0x18 => PortInformationPortType::ScsiWide,
                0x19 => PortInformationPortType::PC98,
                0x1A => PortInformationPortType::PC98Hireso,
                0x1B => PortInformationPortType::PCH98,
                0x1C => PortInformationPortType::VideoPort,
                0x1D => PortInformationPortType::AudioPort,
                0x1E => PortInformationPortType::ModemPort,
                0x1F => PortInformationPortType::NetworkPort,
                0x20 => PortInformationPortType::Sata,
                0x21 => PortInformationPortType::Sas,
                0x22 => PortInformationPortType::Mfdp,
                0x23 => PortInformationPortType::Thunderbolt,
                0xA0 => PortInformationPortType::Port8251Compatible,
                0xA1 => PortInformationPortType::Port8251FifoCompatible,
                0xFF => PortInformationPortType::Other,
                _ => PortInformationPortType::None,
            },
            raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type8 = vec![
            0x08, 0x09, 0x04, 0x00, 0x01, 0x00, 0x02, 0x0F, 0x0E, 0x4A, 0x31, 0x41, 0x31, 0x00,
            0x50, 0x53, 0x32, 0x4D, 0x6F, 0x75, 0x73, 0x65, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type8);
        let test_struct = SMBiosPortConnectorInformation::new(&parts);

        assert_eq!(
            test_struct.internal_reference_designator(),
            Some("J1A1".to_string())
        );
        assert_eq!(
            *test_struct.internal_connector_type().unwrap(),
            PortInformationConnectorType::NoConnector
        );
        assert_eq!(
            test_struct.external_reference_designator(),
            Some("PS2Mouse".to_string())
        );
        assert_eq!(
            *test_struct.external_connector_type().unwrap(),
            PortInformationConnectorType::Ps2
        );
        assert_eq!(
            *test_struct.port_type().unwrap(),
            PortInformationPortType::MousePort
        );
    }
}
