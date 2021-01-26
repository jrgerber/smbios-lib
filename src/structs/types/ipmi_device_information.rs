use crate::*;

/// # IPMI Device Information (Type 38)
///
/// The information in this structure defines the attributes of an Intelligent Platform Management Interface
/// (IPMI) Baseboard Management Controller (BMC).
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosIpmiDeviceInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosIpmiDeviceInformation<'a> {
    const STRUCT_TYPE: u8 = 38u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosIpmiDeviceInformation<'a> {
    /// Baseboard Management Controller (BMC) interface type.
    pub fn interface_type(&self) -> Option<IpmiInterfaceTypeData> {
        self.parts
            .get_field_byte(0x04)
            .and_then(|raw| Some(IpmiInterfaceTypeData::from(raw)))
    }

    /// IPMI specification revision, in BCD format, to which the BMC was designed
    pub fn ipmi_specification_revision(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Slave address on the I2C bus of this BMC
    pub fn i2c_target_address(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Bus ID of the NV storage device.
    ///
    /// If no storage device exists for this BMC, the field is set to 0FFh.
    pub fn nvstorage_device_address(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    /// Base address (either memory-mapped or I/O) of the BMC
    ///
    /// If the least-significant bit of the field is a 1, the address is in
    /// I/O space; otherwise, the address is memory-mapped. Refer
    /// to the [IPMI Interface Specification](https://www.intel.com/content/www/us/en/products/docs/servers/ipmi/ipmi-home.html) for usage details.
    pub fn base_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x08)
    }

    /// Base Address Modifier and Interrupt Info
    pub fn base_address_modifier(&self) -> Option<BaseAddressModifier> {
        self.parts
            .get_field_byte(0x10)
            .and_then(|raw| Some(BaseAddressModifier::from(raw)))
    }

    /// Interrupt number for IPMI System Interface
    ///
    /// 00h = unspecified/unsupported
    pub fn interrupt_number(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }
}

impl fmt::Debug for SMBiosIpmiDeviceInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosIpmiDeviceInformation>())
            .field("header", &self.parts.header)
            .field("interface_type", &self.interface_type())
            .field(
                "ipmi_specification_revision",
                &self.ipmi_specification_revision(),
            )
            .field("i2c_target_address", &self.i2c_target_address())
            .field("nvstorage_device_address", &self.nvstorage_device_address())
            .field("base_address", &self.base_address())
            .field("base_address_modifier", &self.base_address_modifier())
            .field("interrupt_number", &self.interrupt_number())
            .finish()
    }
}

/// # Electrical Current Probe Location and Status
#[derive(PartialEq, Eq)]
pub struct BaseAddressModifier {
    /// Raw value
    pub raw: u8,
    /// Register Spacing
    pub register_spacing: RegisterSpacing,
    /// LS-bit for addresses
    pub ls_address_bit: AddressBit,
    /// Interrupt Info
    ///
    /// Identifies the type and polarity of the interrupt
    /// associated with the IPMI system interface, if any
    pub interrupt_info: InterruptInfo,
    /// Interrupt Polarity
    pub interrupt_polarity: InterruptPolarity,
    /// Interrupt Trigger Mode
    pub interrupt_trigger_mode: InterruptTriggerMode,
}

impl fmt::Debug for BaseAddressModifier {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<BaseAddressModifier>())
            .field("raw", &self.raw)
            .field("register_spacing", &self.register_spacing)
            .field("ls_address_bit", &self.ls_address_bit)
            .field("interrupt_info", &self.interrupt_info)
            .field("interrupt_polarity", &self.interrupt_polarity)
            .field("interrupt_trigger_mode", &self.interrupt_trigger_mode)
            .finish()
    }
}

/// # Register Spacing
#[derive(Debug, PartialEq, Eq)]
pub enum RegisterSpacing {
    /// Interface registers are on successive byte boundaries.
    BoundaryByte,
    /// Interface registers are on 32-bit boundaries.
    Boundary32Bit,
    /// Interface registers are on 16-byte boundaries.
    Boundary16Bit,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # LS-Bit for Addresses
#[derive(Debug, PartialEq, Eq)]
pub enum AddressBit {
    /// Address bit 0 = 0b
    Zero,
    /// Address bit 1 = 1b
    One,
}

/// # Interrupt Info
///
/// Identifies the type and polarity of the interrupt
/// associated with the IPMI system interface, if any
#[derive(Debug, PartialEq, Eq)]
pub enum InterruptInfo {
    /// Interrupt information specified
    Specified,
    /// Interrupt information not specified
    NotSpecified,
}

/// # Interrupt Polarity
#[derive(Debug, PartialEq, Eq)]
pub enum InterruptPolarity {
    /// active high
    ActiveHigh,
    /// active low
    ActiveLow,
}

/// # Interrupt Trigger Mode
#[derive(Debug, PartialEq, Eq)]
pub enum InterruptTriggerMode {
    /// level
    Level,
    /// edge
    Edge,
}

impl From<u8> for BaseAddressModifier {
    fn from(raw: u8) -> Self {
        BaseAddressModifier {
            register_spacing: match raw & 0b11_000000 {
                0b00_000000 => RegisterSpacing::BoundaryByte,
                0b01_000000 => RegisterSpacing::Boundary32Bit,
                0b10_000000 => RegisterSpacing::Boundary16Bit,
                _ => RegisterSpacing::None,
            },
            ls_address_bit: match raw & 0b000_1_0000 {
                0b000_0_0000 => AddressBit::Zero,
                0b000_1_0000 => AddressBit::One,
                _ => panic!("Impossible value"),
            },
            interrupt_info: match raw & 0b0000_1_000 {
                0b0000_1_000 => InterruptInfo::Specified,
                0b0000_0_000 => InterruptInfo::NotSpecified,
                _ => panic!("Impossible value"),
            },
            interrupt_polarity: match raw & 0b000000_1_0 {
                0b000000_1_0 => InterruptPolarity::ActiveHigh,
                0b000000_0_0 => InterruptPolarity::ActiveLow,
                _ => panic!("Impossible value"),
            },
            interrupt_trigger_mode: match raw & 0b0000000_1 {
                0b0000000_1 => InterruptTriggerMode::Level,
                0b0000000_0 => InterruptTriggerMode::Edge,
                _ => panic!("Impossible value"),
            },
            raw,
        }
    }
}

/// # Baseboard Management Controller (BMC) interface type
#[derive(Debug, PartialEq, Eq)]
pub enum IpmiInterfaceType {
    /// Unknown
    Unknown,
    /// KCS: Keyboard Controller Style
    KeyboardControllerStyle,
    /// SMIC: Server Management Interface Chip
    ServerManagementInterfaceChip,
    /// BT: Block Transfer
    BlockTransfer,
    /// SSIF: SMBus System Interface
    SMBusSystemInterface,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Baseboard Management Controller (BMC) interface type data
pub struct IpmiInterfaceTypeData {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The contained [IpmiInterfaceType] value
    pub value: IpmiInterfaceType,
}

impl fmt::Debug for IpmiInterfaceTypeData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<IpmiInterfaceType>())
            .field("raw", &self.raw)
            .field("value", &self.value)
            .finish()
    }
}

impl Deref for IpmiInterfaceTypeData {
    type Target = IpmiInterfaceType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<u8> for IpmiInterfaceTypeData {
    fn from(raw: u8) -> Self {
        IpmiInterfaceTypeData {
            value: match raw {
                0x00 => IpmiInterfaceType::Unknown,
                0x01 => IpmiInterfaceType::KeyboardControllerStyle,
                0x02 => IpmiInterfaceType::ServerManagementInterfaceChip,
                0x03 => IpmiInterfaceType::BlockTransfer,
                0x04 => IpmiInterfaceType::SMBusSystemInterface,
                _ => IpmiInterfaceType::None,
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
        let struct_type38 = vec![
            0x26, 0x12, 0x24, 0x00, 0x01, 0x10, 0x05, 0xFF, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03,
            0x02, 0x01, 0b10010010, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type38.as_slice());
        let test_struct = SMBiosIpmiDeviceInformation::new(&parts);

        assert_eq!(
            *test_struct.interface_type().unwrap(),
            IpmiInterfaceType::KeyboardControllerStyle
        );
        assert_eq!(test_struct.ipmi_specification_revision(), Some(0x10));
        assert_eq!(test_struct.i2c_target_address(), Some(5));
        assert_eq!(test_struct.nvstorage_device_address(), Some(0xFF));
        assert_eq!(test_struct.base_address(), Some(0x0102030405060708));
        let base_address_modifier = test_struct.base_address_modifier().unwrap();
        assert_eq!(
            base_address_modifier.register_spacing,
            RegisterSpacing::Boundary16Bit
        );
        assert_eq!(base_address_modifier.ls_address_bit, AddressBit::One);
        assert_eq!(
            base_address_modifier.interrupt_info,
            InterruptInfo::NotSpecified
        );
        assert_eq!(
            base_address_modifier.interrupt_polarity,
            InterruptPolarity::ActiveHigh
        );
        assert_eq!(
            base_address_modifier.interrupt_trigger_mode,
            InterruptTriggerMode::Edge
        );
    }
}
