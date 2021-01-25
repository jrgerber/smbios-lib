use crate::*;

/// # System Reset (Type 23)
///
/// This structure describes whether Automatic System Reset functions are enabled (Status).
///
/// If the system has a watchdog timer and the timer is not reset (Timer Reset) before the Interval elapses,
/// an automatic system reset occurs. The system re-boots according to the Boot Option. This function may
/// repeat until the Limit is reached, at which time the system re-boots according to the Boot Option at Limit.
///
/// NOTE This structure type was added for version 2.2 of this specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemReset<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemReset<'a> {
    const STRUCT_TYPE: u8 = 23u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemReset<'a> {
    /// Capabilities bit-field
    ///
    /// Identifies the system-reset capabilities for the system
    pub fn capabilities(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Reset count
    ///
    /// Number of automatic system resets since the last intentional
    /// reset
    ///
    /// A value of 0FFFFh indicates unknown.
    pub fn reset_count(&self) -> Option<u16> {
        self.parts.get_field_word(0x05)
    }

    /// Reset limit
    ///
    /// Number of consecutive times the system reset is attempted
    ///
    /// A value of 0FFFFh indicates unknown.
    pub fn reset_limit(&self) -> Option<u16> {
        self.parts.get_field_word(0x07)
    }

    /// Timer interval
    ///
    /// Number of minutes to use for the watchdog timer
    ///
    /// If the timer is not reset within this interval, the system reset
    /// timeout begins. A value of 0FFFFh indicates unknown.
    pub fn timer_interval(&self) -> Option<u16> {
        self.parts.get_field_word(0x09)
    }

    /// Timeout
    ///
    /// Number of minutes before the reboot is initiated
    ///
    /// It is used after a system power cycle, system reset (local or
    /// remote), and automatic system reset. A value of 0FFFFh
    /// indicates unknown.
    pub fn timeout(&self) -> Option<u16> {
        self.parts.get_field_word(0x0B)
    }
}

impl fmt::Debug for SMBiosSystemReset<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemReset>())
            .field("header", &self.parts.header)
            .field("capabilities", &self.capabilities())
            .field("reset_count", &self.reset_count())
            .field("reset_limit", &self.reset_limit())
            .field("timer_interval", &self.timer_interval())
            .field("timeout", &self.timeout())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type23 = vec![
            0x17, 0x0D, 0x4F, 0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
            0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type23.as_slice());
        let test_struct = SMBiosSystemReset::new(&parts);

        assert_eq!(test_struct.capabilities(), Some(0));
        assert_eq!(test_struct.reset_count(), Some(65535));
        assert_eq!(test_struct.reset_limit(), Some(65535));
        assert_eq!(test_struct.timer_interval(), Some(65535));
        assert_eq!(test_struct.timeout(), Some(65535));
    }
}
