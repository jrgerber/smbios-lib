use super::*;

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
