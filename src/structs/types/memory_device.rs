use crate::*;

/// # Memory Device (Type 17)
///
/// This structure describes a single memory device that is part of a larger [SMBiosPhysicalMemoryArray] (Type 16) structure.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosMemoryDevice<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosMemoryDevice<'a> {
    const STRUCT_TYPE: u8 = 17u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosMemoryDevice<'a> {
    /// Handle, or instance number, associated with the
    /// [SMBiosPhysicalMemoryArray] to which this device belongs
    pub fn physical_memory_array_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x04)
    }

    /// Handle, or instance number, associated with any
    /// error that was previously detected for the device
    /// If the system does not provide the error information
    /// structure, the field contains FFFEh; otherwise, the
    /// field contains either FFFFh (if no error was
    /// detected) or the handle of the error-information
    /// structure ([SMBiosMemoryErrorInformation32] or
    /// [SMBiosMemoryErrorInformation64]).
    pub fn memory_error_information_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    /// Total width, in bits, of this memory device, including
    /// any check or error-correction bits
    /// If there are no error-correction bits, this value
    /// should be equal to Data Width. If the width is
    /// unknown, the field is set to FFFFh.
    pub fn total_width(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Data width, in bits, of this memory device
    /// A Data Width of 0 and a Total Width of 8 indicates
    /// that the device is being used solely to provide 8
    /// error-correction bits. If the width is unknown, the
    /// field is set to FFFFh.
    pub fn data_width(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Size of the memory device
    /// If the value is 0, no memory device is installed in the
    /// socket; if the size is unknown, the field value is
    /// FFFFh. If the size is 32 GB-1 MB or greater, the
    /// field value is 7FFFh and the actual size is stored in
    /// the Extended Size field.
    /// The granularity in which the value is specified
    /// depends on the setting of the most-significant bit (bit
    /// 15). If the bit is 0, the value is specified in megabyte
    /// units; if the bit is 1, the value is specified in kilobyte
    /// units. For example, the value 8100h identifies a
    /// 256 KB memory device and 0100h identifies a
    /// 256 MB memory device.
    pub fn size(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Implementation form factor for this memory device
    pub fn form_factor(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0E)
    }

    /// Identifies when the Memory Device is one of a set
    /// of Memory Devices that must be populated with all
    /// devices of the same type and size, and the set to
    /// which this device belongs
    /// A value of 0 indicates that the device is not part of a
    /// set; a value of FFh indicates that the attribute is
    /// unknown.
    /// NOTE: A Device Set number must be unique within the
    /// context of the Memory Array containing this Memory
    /// Device.
    pub fn device_set(&self) -> Option<u8> {
        self.parts.get_field_byte(0x0F)
    }

    /// Identifies the physically-labeled socket or board position where
    /// the memory device is located
    /// EXAMPLE: “SIMM 3”
    pub fn device_locator(&self) -> Option<String> {
        self.parts.get_field_string(0x10)
    }

    /// Identifies the physically labeled bank where the memory device is located
    /// EXAMPLE: “Bank 0” or “A”
    pub fn bank_locator(&self) -> Option<String> {
        self.parts.get_field_string(0x11)
    }

    /// Type of memory used in this device
    pub fn memory_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    /// Additional detail on the memory device type
    pub fn type_detail(&self) -> Option<u16> {
        self.parts.get_field_word(0x13)
    }

    /// Identifies the maximum capable speed of the
    /// device, in megatransfers per second (MT/s).
    /// 0000h = the speed is unknown
    /// FFFFh = the speed is 65,535 MT/s or greater,
    /// and the actual speed is stored in the Extended
    /// Speed field
    pub fn speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x15)
    }

    /// The manufacturer of this memory device
    pub fn manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x17)
    }

    /// The serial number of this memory device.
    /// This value is set by the manufacturer and normally
    /// is not changeable.
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x18)
    }

    /// The asset tag of this memory device
    pub fn asset_tag(&self) -> Option<String> {
        self.parts.get_field_string(0x19)
    }

    /// The part number of this memory device.
    /// This value is set by the manufacturer and normally
    /// is not changeable.
    pub fn part_number(&self) -> Option<String> {
        self.parts.get_field_string(0x1A)
    }

    /// Bits 7-4: reserved
    /// Bits 3-0: rank
    /// Value=0 for unknown rank information
    pub fn attributes(&self) -> Option<u8> {
        self.parts.get_field_byte(0x1B)
    }

    /// Extended size of the memory device (complements
    /// the Size field at offset 0Ch)
    pub fn extended_size(&self) -> Option<u32> {
        self.parts.get_field_dword(0x1C)
    }

    /// Identifies the configured speed of the memory
    /// device, in megatransfers per second (MT/s). See
    /// 7.18.4 for details.
    /// 0000h = the speed is unknown
    /// FFFFh = the speed is 65,535 MT/s or greater,
    /// and the actual speed is stored in the Extended
    /// Configured Memory Speed field
    pub fn configured_memory_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x20)
    }

    /// Minimum operating voltage for this device, in
    /// millivolts
    /// If the value is 0, the voltage is unknown.
    pub fn minimum_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x22)
    }

    /// Maximum operating voltage for this device, in
    /// millivolts
    /// If the value is 0, the voltage is unknown.
    pub fn maximum_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x24)
    }

    /// Configured voltage for this device, in millivolts
    /// If the value is 0, the voltage is unknown.
    pub fn configured_voltage(&self) -> Option<u16> {
        self.parts.get_field_word(0x26)
    }

    /// Memory technology type for this memory device.
    pub fn memory_technology(&self) -> Option<u8> {
        self.parts.get_field_byte(0x28)
    }

    /// The operating modes supported by this memory device.
    pub fn memory_operating_mode_capability(&self) -> Option<u16> {
        self.parts.get_field_word(0x29)
    }

    /// The firmware version of this memory device.
    pub fn firmware_version(&self) -> Option<String> {
        self.parts.get_field_string(0x2B)
    }

    /// The two-byte module manufacturer ID found in the SPD of this memory device; LSB first.
    pub fn module_manufacturer_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x2C)
    }

    /// The two-byte module product ID found in the SPD of this memory device; LSB first.
    pub fn module_product_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x2E)
    }

    /// The two-byte memory subsystem controller manufacturer ID found in the SPD of this memory device; LSB first.
    pub fn memory_subsystem_controller_manufacturer_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x30)
    }

    /// The two-byte memory subsystem controller product ID found in the SPD of this memory device; LSB first.
    pub fn memory_subsystem_controller_product_id(&self) -> Option<u16> {
        self.parts.get_field_word(0x32)
    }

    /// Size of the Non-volatile portion of the memory
    /// device in Bytes, if any. If the value is 0, there is no
    /// non-volatile portion. If the Non-volatile Size is
    /// unknown, the field is set to FFFFFFFFFFFFFFFFh.
    pub fn non_volatile_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x34)
    }

    /// Size of the Volatile portion of the memory device in
    /// Bytes, if any. If the value is 0, there is no Volatile
    /// portion. If the Volatile Size is unknown, the field is
    /// set to FFFFFFFFFFFFFFFFh.
    pub fn volatile_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x3C)
    }

    /// Size of the Cache portion of the memory device in
    /// Bytes, if any. If the value is 0, there is no Cache
    /// portion. If the Cache Size is unknown, the field is
    /// set to FFFFFFFFFFFFFFFFh.
    pub fn cache_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x44)
    }

    /// Size of the Logical memory device in Bytes. If the
    /// size is unknown, the field is set to
    /// FFFFFFFFFFFFFFFFh.
    pub fn logical_size(&self) -> Option<u64> {
        self.parts.get_field_qword(0x4C)
    }

    /// Extended speed of the memory device
    /// (complements the Speed field at offset 15h).
    /// Identifies the maximum capable speed of the
    /// device, in megatransfers per second (MT/s).
    pub fn extended_speed(&self) -> Option<u32> {
        self.parts.get_field_dword(0x54)
    }

    /// Extended configured memory speed of the memory
    /// device (complements the Configured Memory
    /// Speed field at offset 20h). Identifies the configured
    /// speed of the memory device, in megatransfers per
    /// second (MT/s).    
    pub fn extended_configured_memory_speed(&self) -> Option<u32> {
        self.parts.get_field_dword(0x58)
    }
}

impl fmt::Debug for SMBiosMemoryDevice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosMemoryDevice>())
            .field("header", &self.parts.header)
            .field(
                "physical_memory_array_handle",
                &self.physical_memory_array_handle(),
            )
            .field(
                "memory_error_information_handle",
                &self.memory_error_information_handle(),
            )
            .field("total_width", &self.total_width())
            .field("data_width", &self.data_width())
            .field("size", &self.size())
            .field("form_factor", &self.form_factor())
            .field("device_set", &self.device_set())
            .field("device_locator", &self.device_locator())
            .field("bank_locator", &self.bank_locator())
            .field("memory_type", &self.memory_type())
            .field("type_detail", &self.type_detail())
            .field("speed", &self.speed())
            .field("manufacturer", &self.manufacturer())
            .field("serial_number", &self.serial_number())
            .field("asset_tag", &self.asset_tag())
            .field("part_number", &self.part_number())
            .field("attributes", &self.attributes())
            .field("extended_size", &self.extended_size())
            .field("configured_memory_speed", &self.configured_memory_speed())
            .field("minimum_voltage", &self.minimum_voltage())
            .field("maximum_voltage", &self.maximum_voltage())
            .field("configured_voltage", &self.configured_voltage())
            .field("memory_technology", &self.memory_technology())
            .field(
                "memory_operating_mode_capability",
                &self.memory_operating_mode_capability(),
            )
            .field("firmware_version", &self.firmware_version())
            .field("module_manufacturer_id", &self.module_manufacturer_id())
            .field("module_product_id", &self.module_product_id())
            .field(
                "memory_subsystem_controller_manufacturer_id",
                &self.memory_subsystem_controller_manufacturer_id(),
            )
            .field(
                "memory_subsystem_controller_product_id",
                &self.memory_subsystem_controller_product_id(),
            )
            .field("non_volatile_size", &self.non_volatile_size())
            .field("volatile_size", &self.volatile_size())
            .field("cache_size", &self.cache_size())
            .field("logical_size", &self.logical_size())
            .field("extended_speed", &self.extended_speed())
            .field(
                "extended_configured_memory_speed",
                &self.extended_configured_memory_speed(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Memory Device structure is sensitive to BIOS specification versions
        // and prone to bugs.  Therefore, it is important to test different
        // structure versions.
        //
        // Length of the structure:
        // 15h for version 2.1,
        // 1Bh for version 2.3,
        // 1Ch for version 2.6,
        // 22h for version 2.7,
        // 28h for version 2.8,
        // 54h for version 3.2,
        // 5Ch for version 3.3 and later

        // Memory Device structure version 2.8 (does not contain _memory_technology()_ field and beyond)
        let struct_type17 = vec![
            0x11, 0x28, 0x40, 0x00, 0x3E, 0x00, 0xFE, 0xFF, 0x48, 0x00, 0x40, 0x00, 0x00, 0x20,
            0x09, 0x00, 0x01, 0x02, 0x1A, 0x80, 0x00, 0x6A, 0x0A, 0x03, 0x04, 0x05, 0x06, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x6A, 0x0A, 0xB0, 0x04, 0xB0, 0x04, 0xB0, 0x04, 0x43, 0x50,
            0x55, 0x31, 0x5F, 0x44, 0x49, 0x4D, 0x4D, 0x5F, 0x31, 0x00, 0x4E, 0x4F, 0x44, 0x45,
            0x20, 0x31, 0x00, 0x48, 0x79, 0x6E, 0x69, 0x78, 0x00, 0x37, 0x32, 0x30, 0x39, 0x31,
            0x30, 0x30, 0x33, 0x00, 0x20, 0x00, 0x48, 0x4D, 0x41, 0x38, 0x31, 0x47, 0x52, 0x37,
            0x41, 0x46, 0x52, 0x38, 0x4E, 0x2D, 0x56, 0x4B, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type17.as_slice());
        let test_struct = SMBiosMemoryDevice::new(&parts);
        assert_eq!(test_struct.physical_memory_array_handle(), Some(62));
        assert_eq!(test_struct.memory_error_information_handle(), Some(65534));
        assert_eq!(test_struct.total_width(), Some(72));
        assert_eq!(test_struct.data_width(), Some(64));
        assert_eq!(test_struct.size(), Some(8192));
        assert_eq!(test_struct.form_factor(), Some(9));
        assert_eq!(test_struct.device_set(), Some(0));
        assert_eq!(
            test_struct.device_locator(),
            Some("CPU1_DIMM_1".to_string())
        );
        assert_eq!(test_struct.bank_locator(), Some("NODE 1".to_string()));
        assert_eq!(test_struct.memory_type(), Some(26));
        assert_eq!(test_struct.type_detail(), Some(128));
        assert_eq!(test_struct.speed(), Some(2666));
        assert_eq!(test_struct.manufacturer(), Some("Hynix".to_string()));
        assert_eq!(test_struct.serial_number(), Some("72091003".to_string()));
        assert_eq!(test_struct.asset_tag(), Some(" ".to_string()));
        assert_eq!(
            test_struct.part_number(),
            Some("HMA81GR7AFR8N-VK    ".to_string())
        );
        assert_eq!(test_struct.attributes(), Some(1));
        assert_eq!(test_struct.extended_size(), Some(0));
        assert_eq!(test_struct.configured_memory_speed(), Some(2666));
        assert_eq!(test_struct.minimum_voltage(), Some(1200));
        assert_eq!(test_struct.maximum_voltage(), Some(1200));
        assert_eq!(test_struct.configured_voltage(), Some(1200));

        // version 2.8 does not contain _memory_technology()_ field and fields beyond
        assert_eq!(test_struct.memory_technology(), None);

        // TODO: add a 3.2 and 3.3 structure and test these fields:
        // assert_eq!(test_struct.memory_operating_mode_capability(), Some(21840));
        // assert_eq!(test_struct.firmware_version(), None);
        // assert_eq!(test_struct.module_manufacturer_id(), Some(17503));
        // assert_eq!(test_struct.module_product_id(), Some(19785));
        // assert_eq!(
        //     test_struct.memory_subsystem_controller_manufacturer_id(),
        //     Some(24397)
        // );
        // assert_eq!(
        //     test_struct.memory_subsystem_controller_product_id(),
        //     Some(49)
        // );
        // assert_eq!(test_struct.non_volatile_size(), Some(5188200785401630542));
        // assert_eq!(test_struct.volatile_size(), Some(3472898737815776889));
        // assert_eq!(test_struct.cache_size(), Some(9007419106537785));
        // assert_eq!(test_struct.logical_size(), Some(3986326896899083592));
        // assert_eq!(test_struct.extended_speed(), Some(944916033));
        // assert_eq!(
        //     test_struct.extended_configured_memory_speed(),
        //     Some(1263938894)
        // );
    }
}
