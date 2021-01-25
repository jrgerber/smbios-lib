use crate::*;

/// # System Power Controls (Type 25)
///
/// This structure describes the attributes for controlling the main power supply to the system.
///
/// Software that interprets this structure uses the month, day, hour, minute, and second values to determine
/// the number of seconds until the next power-on of the system. The presence of this structure implies that a
/// timed power-on facility is available for the system.
///
/// NOTE This structure type was added in version 2.2 of the specification.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemPowerControls<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemPowerControls<'a> {
    const STRUCT_TYPE: u8 = 25u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemPowerControls<'a> {
    /// Next scheduled power-on month
    ///
    /// BCD value of the month on which the next scheduled
    /// power-on is to occur, in the range 01h to 12h.
    pub fn next_scheduled_power_on_month(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Next scheduled power-on day-of month
    ///
    /// BCD value of the day-of-month on which the next
    /// scheduled power-on is to occur, in the range 01h to 31h.
    pub fn next_scheduled_power_on_day_of_month(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// Next scheduled power-on hour
    ///
    /// BCD value of the hour on which the next scheduled power-on
    /// is to occur, in the range 00h to 23h.
    pub fn next_scheduled_power_on_hour(&self) -> Option<u8> {
        self.parts.get_field_byte(0x06)
    }

    /// Next scheduled power-on minute
    ///
    /// BCD value of the minute on which the next scheduled
    /// power-on is to occur, in the range 00h to 59h.
    pub fn next_scheduled_power_on_minute(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    /// Next scheduled power-on second
    ///
    /// BCD value of the second on which the next scheduled
    /// power-on is to occur, in the range 00h to 59h.
    pub fn next_scheduled_power_on_second(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }
}

impl fmt::Debug for SMBiosSystemPowerControls<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemPowerControls>())
            .field("header", &self.parts.header)
            .field(
                "next_scheduled_power_on_month",
                &self.next_scheduled_power_on_month(),
            )
            .field(
                "next_scheduled_power_on_day_of_month",
                &self.next_scheduled_power_on_day_of_month(),
            )
            .field(
                "next_scheduled_power_on_hour",
                &self.next_scheduled_power_on_hour(),
            )
            .field(
                "next_scheduled_power_on_minute",
                &self.next_scheduled_power_on_minute(),
            )
            .field(
                "next_scheduled_power_on_second",
                &self.next_scheduled_power_on_second(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type25 = vec![
            0x19, 0x09, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let parts = SMBiosStructParts::new(struct_type25.as_slice());
        let test_struct = SMBiosSystemPowerControls::new(&parts);

        assert_eq!(test_struct.next_scheduled_power_on_month(), Some(0));
        assert_eq!(test_struct.next_scheduled_power_on_day_of_month(), Some(0));
        assert_eq!(test_struct.next_scheduled_power_on_hour(), Some(0));
        assert_eq!(test_struct.next_scheduled_power_on_minute(), Some(0));
        assert_eq!(test_struct.next_scheduled_power_on_second(), Some(0));
    }
}
