use super::*;

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
    pub fn interface_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// IPMI specification revision, in BCD format, to which the BMC was designed
    pub fn ipmi_specification_revision(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Slave address on the I2C bus of this BMC
    pub fn i2c_target_address(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Bus ID of the NV storage device. If no storage device exists for this BMC, the field is set to 0FFh.
    pub fn nvstorage_device_address(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    /// Base address (either memory-mapped or I/O) of the BMC
    /// If the least-significant bit of the field is a 1, the address is in
    /// I/O space; otherwise, the address is memory-mapped. Refer
    /// to the [IPMI Interface Specification](https://www.intel.com/content/www/us/en/products/docs/servers/ipmi/ipmi-home.html) for usage details.
    pub fn base_address(&self) -> Option<u64> {
        self.parts.get_field_qword(0x08)
    }

    /// Bit fields Base Address Modifier and Interrupt Info
    pub fn base_address_modifier(&self) -> Option<u8> {
        self.parts.get_field_byte(0x10)
    }

    /// Interrupt number for IPMI System Interface
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
