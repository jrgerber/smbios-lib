use crate::*;
use std::{
    array::TryFromSliceError,
    convert::{TryFrom, TryInto},
};

/// # System Information (Type 1)
///
/// The information in this structure defines attributes of the overall system and is intended to be associated
/// with the Component ID group of the system’s MIF. An SMBIOS implementation is associated with a single
/// system instance and contains one and only one System Information (Type 1) structure.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemInformation<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemInformation<'a> {
    const STRUCT_TYPE: u8 = 1u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosSystemInformation<'a> {
    /// Manufacturer
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Product name
    pub fn product_name(&self) -> Option<String> {
        self.parts.get_field_string(0x05)
    }

    /// Version
    pub fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// Serial number
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// System UUID
    pub fn uuid(&self) -> Option<SystemUuidData<'_>> {
        self.parts
            .get_field_data(0x08, 0x18)
            .and_then(|raw| Some(SystemUuidData::try_from(raw).expect("A GUID is 0x10 bytes")))
    }

    /// Wake-up type
    ///
    /// Identifies the event that caused the system to power up.
    pub fn wakeup_type(&self) -> Option<SystemWakeUpTypeData> {
        self.parts
            .get_field_byte(0x18)
            .and_then(|raw| Some(SystemWakeUpTypeData::from(raw)))
    }

    /// SKU Number
    ///
    /// This text string identifies a particular computer
    /// configuration for sale. It is sometimes also
    /// called a product ID or purchase order number.
    /// This number is frequently found in existing
    /// fields, but there is no standard format.
    /// Typically for a given system board from a
    /// given OEM, there are tens of unique
    /// processor, memory, hard drive, and optical
    /// drive configurations.
    pub fn sku_number(&self) -> Option<String> {
        self.parts.get_field_string(0x19)
    }

    /// Family
    ///
    /// This text string identifies the family to which a
    /// particular computer belongs. A family refers to
    /// a set of computers that are similar but not
    /// identical from a hardware or software point of
    /// view. Typically, a family is composed of
    /// different computer models, which have
    /// different configurations and pricing points.
    /// Computers in the same family often have
    /// similar branding and cosmetic features.
    pub fn family(&self) -> Option<String> {
        self.parts.get_field_string(0x1A)
    }
}

impl fmt::Debug for SMBiosSystemInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemInformation<'_>>())
            .field("header", &self.parts.header)
            .field("manufacturer", &self.manufacturer())
            .field("product_name", &self.product_name())
            .field("version", &self.version())
            .field("serial_number", &self.serial_number())
            .field("uuid", &self.uuid())
            .field("wakeup_type", &self.wakeup_type())
            .field("sku_number", &self.sku_number())
            .field("family", &self.family())
            .finish()
    }
}

/// # System - UUID Data
#[derive(Debug)]
pub enum SystemUuidData<'a> {
    /// The ID is not currently present in the system, but it can be set
    IdNotPresentButSettable,
    /// The ID is not present in the system
    IdNotPresent,
    /// System UUID
    Uuid(SystemUuid<'a>),
}

impl<'a> SystemUuidData<'a> {
    fn new(array: &'a [u8; 0x10]) -> SystemUuidData<'a> {
        if array.iter().all(|&x| x == 0) {
            SystemUuidData::IdNotPresentButSettable
        } else if array.iter().all(|&x| x == 0xFF) {
            SystemUuidData::IdNotPresent
        } else {
            SystemUuidData::Uuid(SystemUuid::from(array))
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for SystemUuidData<'a> {
    type Error = TryFromSliceError;

    fn try_from(raw: &'a [u8]) -> Result<Self, Self::Error> {
        <&[u8; 0x10]>::try_from(raw).and_then(|array| Ok(SystemUuidData::new(array)))
    }
}

/// # System - UUID
#[derive(PartialEq, Eq)]
pub struct SystemUuid<'a> {
    /// Raw byte array for this UUID
    pub raw: &'a [u8; 0x10],
}

impl<'a> SystemUuid<'a> {
    /// Low field of the timestamp
    pub fn time_low(&self) -> u32 {
        u32::from_le_bytes(self.raw[..0x4].try_into().expect("incorrect size"))
    }

    /// Middle field of the timestamp
    pub fn time_mid(&self) -> u16 {
        u16::from_le_bytes(self.raw[0x4..0x6].try_into().expect("incorrect size"))
    }

    /// High field of the timestamp multiplexed with the version number
    pub fn time_high_and_version(&self) -> u16 {
        u16::from_le_bytes(self.raw[0x6..0x8].try_into().expect("incorrect size"))
    }

    /// High field of the clock sequence multiplexed with the variant
    pub fn clock_seq_high_and_reserved(&self) -> u8 {
        self.raw[0x8]
    }

    /// Low field of the clock sequence
    pub fn clock_seq_low(&self) -> u8 {
        self.raw[0x9]
    }

    /// Spatially unique node identifier
    pub fn node(&self) -> &[u8; 6] {
        self.raw[0xA..0x10].try_into().expect("incorrect size")
    }
}

impl<'a> From<&'a [u8; 0x10]> for SystemUuid<'a> {
    fn from(raw: &'a [u8; 0x10]) -> Self {
        SystemUuid { raw }
    }
}

impl<'a> fmt::Debug for SystemUuid<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Example output:
        // "00360FE7-D4D5-11E5-9C43-BC0000F00000"
        // <TimeLow>-<TimeMid>-<TimeHiAndVersion>-<ClockSeqHiAndReserved><ClockSeqLow>-<Node[6]>
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-",
            self.time_low(),
            self.time_mid(),
            self.time_high_and_version(),
            self.clock_seq_high_and_reserved(),
            self.clock_seq_low()
        )?;

        self.node().iter().fold(Ok(()), |result, node_byte| {
            result.and_then(|_| write!(f, "{:02X}", node_byte))
        })
    }
}

/// # System - Wake-up Type Data
pub struct SystemWakeUpTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [SystemWakeUpType] value
    pub value: SystemWakeUpType,
}

impl fmt::Debug for SystemWakeUpTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SystemWakeUpTypeData>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for SystemWakeUpTypeData {
    type Target = SystemWakeUpType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// # System - Wake-up Type
#[derive(Debug, PartialEq, Eq)]
pub enum SystemWakeUpType {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// APM Timer
    ApmTimer,
    /// Modem Ring
    ModernRing,
    /// LAN Remote
    LanRemote,
    /// Power Switch
    PowerSwitch,
    /// PCI PME#
    PciPme,
    /// AC Power Restored
    ACPowerRestored,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for SystemWakeUpTypeData {
    fn from(raw: u8) -> Self {
        SystemWakeUpTypeData {
            value: match raw {
                0x01 => SystemWakeUpType::Other,
                0x02 => SystemWakeUpType::Unknown,
                0x03 => SystemWakeUpType::ApmTimer,
                0x04 => SystemWakeUpType::ModernRing,
                0x05 => SystemWakeUpType::LanRemote,
                0x06 => SystemWakeUpType::PowerSwitch,
                0x07 => SystemWakeUpType::PciPme,
                0x08 => SystemWakeUpType::ACPowerRestored,
                _ => SystemWakeUpType::None,
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
        let struct_type1 = vec![
            0x01, 0x1B, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04, 0xD2, 0x01, 0x25, 0x3E, 0x48, 0xE6,
            0x11, 0xE8, 0xBA, 0xD3, 0x70, 0x20, 0x84, 0x0F, 0x9D, 0x47, 0x06, 0x05, 0x06, b'L',
            b'E', b'N', b'O', b'V', b'O', 0x00, b'3', b'0', b'B', b'F', b'S', b'0', b'7', b'5',
            b'0', b'0', 0x00, b'T', b'h', b'i', b'n', b'k', b'S', b't', b'a', b't', b'i', b'o',
            b'n', b' ', b'P', b'5', b'2', b'0', 0x00, b'M', b'N', b'0', b'6', b'P', b'Q', b'R',
            b'S', 0x00, b'L', b'E', b'N', b'O', b'V', b'O', b'_', b'M', b'T', b'_', b'3', b'0',
            b'B', b'F', b'_', b'B', b'U', b'_', b'T', b'h', b'i', b'n', b'k', b'_', b'F', b'M',
            b'_', b'T', b'h', b'i', b'n', b'k', b'S', b't', b'a', b't', b'i', b'o', b'n', b' ',
            b'P', b'5', b'2', b'0', 0x00, b'T', b'h', b'i', b'n', b'k', b'S', b't', b'a', b't',
            b'i', b'o', b'n', b' ', b'P', b'5', b'2', b'0', 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type1);
        let test_struct = SMBiosSystemInformation::new(&parts);

        assert_eq!(test_struct.manufacturer(), Some("LENOVO".to_string()));
        assert_eq!(test_struct.product_name(), Some("30BFS07500".to_string()));
        assert_eq!(test_struct.version(), Some("ThinkStation P520".to_string()));
        assert_eq!(test_struct.serial_number(), Some("MN06PQRS".to_string()));
        assert_eq!(
            format!("{:?}", test_struct.uuid()),
            "Some(Uuid(3E2501D2-E648-E811-BAD3-7020840F9D47))".to_string()
        );
        assert_eq!(
            *test_struct.wakeup_type().unwrap(),
            SystemWakeUpType::PowerSwitch
        );
        assert_eq!(
            test_struct.sku_number(),
            Some("LENOVO_MT_30BF_BU_Think_FM_ThinkStation P520".to_string())
        );
        assert_eq!(test_struct.family(), Some("ThinkStation P520".to_string()));
    }
}
