use super::*;

/// # Processor Information (Type 4)
///
/// The information in this structure defines the attributes of a single processor; a separate
/// structure instance is provided for each system processor socket/slot. For example, a system with an
/// IntelDX2™ processor would have a single structure instance while a system with an IntelSX2™ processor
/// would have a structure to describe the main CPU and a second structure to describe the 80487 co1021 processor.
///
/// NOTE One structure is provided for each processor instance in a system. For example, a system that supports up
/// to two processors includes two Processor Information structures — even if only one processor is currently
/// installed. Software that interprets the SMBIOS information can count the Processor Information structures to
/// determine the maximum possible configuration of the system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosProcessorInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosProcessorInformation<'a> {
    const STRUCT_TYPE: u8 = 4u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosProcessorInformation<'a> {
    /// Socket reference designation
    ///
    /// EXAMPLE: "J202"
    pub fn socket_designation(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Processor type
    pub fn processor_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Processor family
    pub fn processor_family(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Processor manufacturer
    pub fn processor_manufacturer(&self) -> Option<String> {
        self.parts.get_field_string(0x07)
    }

    /// Raw processor identification data
    pub fn processor_id(&self) -> Option<u64> {
        self.parts.get_field_qword(0x08)
    }

    /// Processor version
    pub fn processor_version(&self) -> Option<String> {
        self.parts.get_field_string(0x10)
    }

    /// Voltage
    pub fn voltage(&self) -> Option<u8> {
        self.parts.get_field_byte(0x11)
    }

    /// External clock frequency, in MHz
    ///
    /// If the value is unknown, the field is set to 0.
    pub fn external_clock(&self) -> Option<u16> {
        self.parts.get_field_word(0x12)
    }

    /// Maximum processor speed (in MHz) supported
    /// by the system for this processor socket
    ///
    /// 0E9h is for a 233 MHz processor. If the value is
    /// unknown, the field is set to 0.
    ///
    /// NOTE: This field identifies a capability for the system,
    /// not the processor itself.
    pub fn max_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }

    /// Current speed
    ///
    /// Same format as Max Speed
    ///
    /// NOTE: This field identifies the processor's speed at
    /// system boot; the processor may support
    /// more than one speed.
    pub fn current_speed(&self) -> Option<u16> {
        self.parts.get_field_word(0x16)
    }

    /// Status bit field
    pub fn status(&self) -> Option<u8> {
        self.parts.get_field_byte(0x18)
    }

    /// Processor upgrade
    pub fn processor_upgrade(&self) -> Option<u8> {
        self.parts.get_field_byte(0x19)
    }

    /// Handle of a [SMBiosCacheInformation] structure that
    /// defines the attributes of the primary (Level 1)
    /// cache for this processor
    ///
    /// For version 2.1 and version 2.2
    /// implementations, the value is 0FFFFh if the
    /// processor has no L1 cache. For version 2.3 and
    /// later implementations, the value is 0FFFFh if
    /// the Cache Information structure is not provided.
    pub fn l1cache_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x1A)
    }

    /// Handle of a [SMBiosCacheInformation] structure that
    /// defines the attributes of the primary (Level 2)
    /// cache for this processor
    ///
    /// For version 2.1 and version 2.2
    /// implementations, the value is 0FFFFh if the
    /// processor has no L2 cache. For version 2.3 and
    /// later implementations, the value is 0FFFFh if
    /// the Cache Information structure is not provided.
    pub fn l2cache_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x1C)
    }

    /// Handle of a [SMBiosCacheInformation] structure that
    /// defines the attributes of the primary (Level 3)
    /// cache for this processor
    ///
    /// For version 2.1 and version 2.2
    /// implementations, the value is 0FFFFh if the
    /// processor has no L3 cache. For version 2.3 and
    /// later implementations, the value is 0FFFFh if
    /// the Cache Information structure is not provided.
    pub fn l3cache_handle(&self) -> Option<u16> {
        self.parts.get_field_word(0x1E)
    }

    /// The serial number of this processor
    ///
    /// This value is set by the manufacturer and
    /// normally not changeable.
    pub fn serial_number(&self) -> Option<String> {
        self.parts.get_field_string(0x20)
    }

    /// The asset tag of this processor
    pub fn asset_tag(&self) -> Option<String> {
        self.parts.get_field_string(0x21)
    }

    /// The part number of this processor
    ///
    /// This value is set by the manufacturer and
    /// normally not changeable.
    pub fn part_number(&self) -> Option<String> {
        self.parts.get_field_string(0x22)
    }

    /// Number of cores per processor socket
    ///
    /// If the value is unknown, the field is
    /// set to 0. For core counts of 256 or greater, the
    /// Core Count field is set to FFh and the Core
    /// Count 2 field is set to the number of cores.
    pub fn core_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x23)
    }

    /// Number of enabled cores per processor socket
    ///
    /// If the value is unknown, the field is
    /// set 0. For core counts of 256 or greater, the
    /// Core Enabled field is set to FFh and the Core
    /// Enabled 2 field is set to the number of enabled
    /// cores.
    pub fn core_enabled(&self) -> Option<u8> {
        self.parts.get_field_byte(0x24)
    }

    /// Number of threads per processor socket
    ///
    /// If the value is unknown, the field is
    /// set to 0. For thread counts of 256 or greater,
    /// the Thread Count field is set to FFh and the
    /// Thread Count 2 field is set to the number of
    /// threads.
    pub fn thread_count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x25)
    }

    /// Defines which functions the processor supports
    pub fn processor_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x26)
    }

    /// Processor family 2
    pub fn processor_family_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x28)
    }

    /// Number of Cores per processor socket.
    ///
    /// Supports core counts >255. If this field is
    /// present, it holds the core count for the
    /// processor socket. Core Count will also hold the
    /// core count, except for core counts that are 256
    /// or greater. In that case, Core Count shall be set
    /// to FFh and Core Count 2 will hold the count.
    ///
    /// Legal values:
    /// 0000h = unknown
    /// 0001h-00FFh = core counts 1 to 255.
    /// Matches Core Count value.
    /// 0100h-FFFEh = Core counts 256 to
    /// 65534, respectively.
    /// FFFFh = reserved.
    pub fn core_count_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2A)
    }

    /// Number of enabled cores per processor socket.
    ///
    /// Supports core enabled counts >255. If this field
    /// is present, it holds the core enabled count for
    /// the processor socket. Core Enabled will also
    /// hold the core enabled count, except for core
    /// counts that are 256 or greater. In that case,
    /// Core Enabled shall be set to FFh and Core
    /// Enabled 2 will hold the count.
    ///
    /// Legal values:
    /// 0000h = unknown
    /// 0001h-00FFh = core enabled counts 1 to
    /// 255. Matches Core Enabled value.
    /// 0100h-FFFEh = core enabled counts 256
    /// to 65534, respectively.
    /// FFFFh = reserved.
    pub fn core_enabled_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2C)
    }

    /// Number of threads per processor socket.
    ///
    /// Supports thread counts >255. If this field is
    /// present, it holds the thread count for the
    /// processor socket. Thread Count will also hold
    /// the thread count, except for thread counts that
    /// are 256 or greater. In that case, Thread Count
    /// shall be set to FFh and Thread Count 2 will
    /// hold the count.
    ///
    /// Legal values:0000h = unknown
    /// 0001h-00FFh = thread counts 1 to 255.
    /// Matches Thread Count value.
    /// 0100h-FFFEh = thread counts 256 to
    /// 65534, respectively.
    /// FFFFh = reserved.
    pub fn thread_count_2(&self) -> Option<u16> {
        self.parts.get_field_word(0x2E)
    }
}

impl fmt::Debug for SMBiosProcessorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosProcessorInformation>())
            .field("header", &self.parts.header)
            .field("socket_designation", &self.socket_designation())
            .field("processor_type", &self.processor_type())
            .field("processor_family", &self.processor_family())
            .field("processor_manufacturer", &self.processor_manufacturer())
            .field("processor_id", &self.processor_id())
            .field("processor_version", &self.processor_version())
            .field("voltage", &self.voltage())
            .field("external_clock", &self.external_clock())
            .field("max_speed", &self.max_speed())
            .field("current_speed", &self.current_speed())
            .field("status", &self.status())
            .field("processor_upgrade", &self.processor_upgrade())
            .field("l1cache_handle", &self.l1cache_handle())
            .field("l2cache_handle", &self.l2cache_handle())
            .field("l3cache_handle", &self.l3cache_handle())
            .field("serial_number", &self.serial_number())
            .field("asset_tag", &self.asset_tag())
            .field("part_number", &self.part_number())
            .field("core_count", &self.core_count())
            .field("core_enabled", &self.core_enabled())
            .field("thread_count", &self.thread_count())
            .field(
                "processor_characteristics",
                &self.processor_characteristics(),
            )
            .field("processor_family_2", &self.processor_family_2())
            .field("core_count_2", &self.core_count_2())
            .field("core_enabled_2", &self.core_enabled_2())
            .field("thread_count_2", &self.thread_count_2())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type4 = vec![
            0x04, 0x30, 0x56, 0x00, 0x01, 0x03, 0xB3, 0x02, 0x54, 0x06, 0x05, 0x00, 0xFF, 0xFB,
            0xEB, 0xBF, 0x03, 0x90, 0x64, 0x00, 0x3C, 0x0F, 0x10, 0x0E, 0x41, 0x01, 0x53, 0x00,
            0x54, 0x00, 0x55, 0x00, 0x00, 0x04, 0x00, 0x06, 0x06, 0x0C, 0xFC, 0x00, 0xB3, 0x00,
            0x06, 0x00, 0x06, 0x00, 0x0C, 0x00, 0x43, 0x50, 0x55, 0x30, 0x00, 0x49, 0x6E, 0x74,
            0x65, 0x6C, 0x28, 0x52, 0x29, 0x20, 0x43, 0x6F, 0x72, 0x70, 0x6F, 0x72, 0x61, 0x74,
            0x69, 0x6F, 0x6E, 0x00, 0x49, 0x6E, 0x74, 0x65, 0x6C, 0x28, 0x52, 0x29, 0x20, 0x58,
            0x65, 0x6F, 0x6E, 0x28, 0x52, 0x29, 0x20, 0x57, 0x2D, 0x32, 0x31, 0x33, 0x33, 0x20,
            0x43, 0x50, 0x55, 0x20, 0x40, 0x20, 0x33, 0x2E, 0x36, 0x30, 0x47, 0x48, 0x7A, 0x00,
            0x55, 0x4E, 0x4B, 0x4E, 0x4F, 0x57, 0x4E, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type4.as_slice());
        let test_struct = SMBiosProcessorInformation::new(&parts);

        assert_eq!(test_struct.socket_designation(), Some("CPU0".to_string()));
        assert_eq!(test_struct.processor_type(), Some(3));
        assert_eq!(test_struct.processor_family(), Some(179));
        assert_eq!(
            test_struct.processor_manufacturer(),
            Some("Intel(R.to_string()) Corporation".to_string())
        );
        assert_eq!(test_struct.processor_id(), Some(13829424153406801492));
        assert_eq!(
            test_struct.processor_version(),
            Some("Intel(R.to_string()) Xeon(R.to_string()) W-2133 CPU @ 3.60GHz".to_string())
        );
        assert_eq!(test_struct.voltage(), Some(144));
        assert_eq!(test_struct.external_clock(), Some(100));
        assert_eq!(test_struct.max_speed(), Some(3900));
        assert_eq!(test_struct.current_speed(), Some(3600));
        assert_eq!(test_struct.status(), Some(65));
        assert_eq!(test_struct.processor_upgrade(), Some(1));
        assert_eq!(test_struct.l1cache_handle(), Some(83));
        assert_eq!(test_struct.l2cache_handle(), Some(84));
        assert_eq!(test_struct.l3cache_handle(), Some(85));
        assert_eq!(test_struct.serial_number(), None);
        assert_eq!(test_struct.asset_tag(), Some("UNKNOWN".to_string()));
        assert_eq!(test_struct.part_number(), None);
        assert_eq!(test_struct.core_count(), Some(6));
        assert_eq!(test_struct.core_enabled(), Some(6));
        assert_eq!(test_struct.thread_count(), Some(12));
        assert_eq!(test_struct.processor_characteristics(), Some(252));
        assert_eq!(test_struct.processor_family_2(), Some(179));
        assert_eq!(test_struct.core_count_2(), Some(6));
        assert_eq!(test_struct.core_enabled_2(), Some(6));
        assert_eq!(test_struct.thread_count_2(), Some(12));
    }
}
