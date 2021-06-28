use crate::{SMBiosStruct, UndefinedStruct};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use core::{fmt, any};

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
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemReset<'a> {
    const STRUCT_TYPE: u8 = 23u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosSystemReset<'a> {
    /// Capabilities bit-field
    ///
    /// Identifies the system-reset capabilities for the system
    pub fn capabilities(&self) -> Option<SystemResetCapabilities> {
        self.parts
            .get_field_byte(0x04)
            .map(|raw| SystemResetCapabilities::from(raw))
    }

    /// Reset count
    ///
    /// Number of automatic system resets since the last intentional
    /// reset
    pub fn reset_count(&self) -> Option<ResetCount> {
        self.parts
            .get_field_word(0x05)
            .map(|raw| ResetCount::from(raw))
    }

    /// Reset limit
    ///
    /// Number of consecutive times the system reset is attempted
    pub fn reset_limit(&self) -> Option<ResetLimit> {
        self.parts
            .get_field_word(0x07)
            .map(|raw| ResetLimit::from(raw))
    }

    /// Timer interval
    ///
    /// Number of minutes to use for the watchdog timer
    ///
    /// If the timer is not reset within this interval, the system reset
    /// timeout begins.
    pub fn timer_interval(&self) -> Option<TimerInterval> {
        self.parts
            .get_field_word(0x09)
            .map(|raw| TimerInterval::from(raw))
    }

    /// Timeout
    ///
    /// Number of minutes before the reboot is initiated
    ///
    /// It is used after a system power cycle, system reset (local or
    /// remote), and automatic system reset.
    pub fn timeout(&self) -> Option<Timeout> {
        self.parts
            .get_field_word(0x0B)
            .map(|raw| Timeout::from(raw))
    }
}

impl fmt::Debug for SMBiosSystemReset<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SMBiosSystemReset<'_>>())
            .field("header", &self.parts.header)
            .field("capabilities", &self.capabilities())
            .field("reset_count", &self.reset_count())
            .field("reset_limit", &self.reset_limit())
            .field("timer_interval", &self.timer_interval())
            .field("timeout", &self.timeout())
            .finish()
    }
}

impl Serialize for SMBiosSystemReset<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMBiosSystemReset", 6)?;
        state.serialize_field("header", &self.parts.header)?;
        state.serialize_field("capabilities", &self.capabilities())?;
        state.serialize_field("reset_count", &self.reset_count())?;
        state.serialize_field("reset_limit", &self.reset_limit())?;
        state.serialize_field("timer_interval", &self.timer_interval())?;
        state.serialize_field("timeout", &self.timeout())?;
        state.end()
    }
}

/// # System Reset Capabilities
#[derive(PartialEq, Eq)]
pub struct SystemResetCapabilities {
    /// Raw byte of the system reset capabilities
    pub raw: u8,
}

impl From<u8> for SystemResetCapabilities {
    fn from(raw: u8) -> Self {
        SystemResetCapabilities { raw }
    }
}

impl SystemResetCapabilities {
    /// System contains a watchdog timer; either
    /// True (1) or False (0).
    pub fn has_watchdog_timer(&self) -> bool {
        self.raw & 0b0010_0000 == 0b0010_0000
    }

    /// Boot Option on Limit
    ///
    /// Identifies one of the system actions
    /// to be taken when the Reset Limit is reached.
    pub fn boot_option_on_limit(&self) -> BootOptionOnLimit {
        BootOptionOnLimit::from(self.raw)
    }

    /// Boot Option
    ///
    /// Indicates one of the following actions
    /// to be taken after a watchdog reset:
    pub fn boot_option(&self) -> BootOption {
        BootOption::from(self.raw)
    }

    /// Status
    ///
    /// Identifies whether (1) or not (0)
    /// the system reset is enabled by the user.
    pub fn reset_enabled(&self) -> bool {
        self.raw & 0b0000_0001 == 0b0000_0001
    }
}

impl fmt::Debug for SystemResetCapabilities {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(any::type_name::<SystemResetCapabilities>())
            .field("raw", &self.raw)
            .field("has_watchdog_timer", &self.has_watchdog_timer())
            .field("boot_option_on_limit", &self.boot_option_on_limit())
            .field("boot_option", &self.boot_option())
            .field("reset_enabled", &self.reset_enabled())
            .finish()
    }
}

impl Serialize for SystemResetCapabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SystemResetCapabilities", 5)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("has_watchdog_timer", &self.has_watchdog_timer())?;
        state.serialize_field("boot_option_on_limit", &self.boot_option_on_limit())?;
        state.serialize_field("boot_option", &self.boot_option())?;
        state.serialize_field("reset_enabled", &self.reset_enabled())?;
        state.end()
    }
}

/// # Boot Option on Limit
///
/// Identifies one of the following system actions to
/// be taken when the Reset Limit is reached
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BootOptionOnLimit {
    /// Reserved, do not use.
    Reserved,
    /// Operating System
    OperatingSystem,
    /// System utilities
    SystemUtilities,
    /// Do not reboot
    DoNotReboot,
}

impl From<u8> for BootOptionOnLimit {
    fn from(raw: u8) -> Self {
        match raw & 0b0001_1000 {
            0b0000_0000 => BootOptionOnLimit::Reserved,
            0b0000_1000 => BootOptionOnLimit::OperatingSystem,
            0b0001_0000 => BootOptionOnLimit::SystemUtilities,
            0b0001_1000 => BootOptionOnLimit::DoNotReboot,
            _ => panic!("impossible value"),
        }
    }
}

/// # Boot Option
///
/// Indicates one of the following actions to be taken
//  after a watchdog reset
#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BootOption {
    /// Reserved, do not use.
    Reserved,
    /// Operating System
    OperatingSystem,
    /// System utilities
    SystemUtilities,
    /// Do not reboot
    DoNotReboot,
}

impl From<u8> for BootOption {
    fn from(raw: u8) -> Self {
        match raw & 0b0000_0110 {
            0b0000_0000 => BootOption::Reserved,
            0b0000_0010 => BootOption::OperatingSystem,
            0b0000_0100 => BootOption::SystemUtilities,
            0b0000_0110 => BootOption::DoNotReboot,
            _ => panic!("impossible value"),
        }
    }
}

/// # Reset Count
#[derive(Serialize, Debug)]
pub enum ResetCount {
    /// Number of automatic system resets since the last intentional reset
    Count(u16),
    /// Reset count is unknown.
    Unknown,
}

impl From<u16> for ResetCount {
    fn from(raw: u16) -> Self {
        match raw {
            0xFFFF => ResetCount::Unknown,
            _ => ResetCount::Count(raw),
        }
    }
}

/// # Reset Limit
#[derive(Serialize, Debug)]
pub enum ResetLimit {
    /// Number of consecutive times the system reset is attempted
    Count(u16),
    /// Reset limit is unknown.
    Unknown,
}

impl From<u16> for ResetLimit {
    fn from(raw: u16) -> Self {
        match raw {
            0xFFFF => ResetLimit::Unknown,
            _ => ResetLimit::Count(raw),
        }
    }
}

/// # Timer Interval
#[derive(Serialize, Debug)]
pub enum TimerInterval {
    /// Number of minutes to use for the watchdog timer
    ///
    /// If the timer is not reset within this interval,
    /// the system reset timeout begins.
    Minutes(u16),
    /// Timer interval is unknown.
    Unknown,
}

impl From<u16> for TimerInterval {
    fn from(raw: u16) -> Self {
        match raw {
            0xFFFF => TimerInterval::Unknown,
            _ => TimerInterval::Minutes(raw),
        }
    }
}

/// # Timeout
#[derive(Serialize, Debug)]
pub enum Timeout {
    /// Number of minutes before the reboot is initiated
    ///
    /// It is used after a system power cycle, system reset
    // (local or remote), and automatic system reset.
    Minutes(u16),
    /// Timeout is unknown.
    Unknown,
}

impl From<u16> for Timeout {
    fn from(raw: u16) -> Self {
        match raw {
            0xFFFF => Timeout::Unknown,
            _ => Timeout::Minutes(raw),
        }
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

        let parts = UndefinedStruct::new(&struct_type23);
        let test_struct = SMBiosSystemReset::new(&parts);

        assert_eq!(
            test_struct.capabilities(),
            Some(SystemResetCapabilities::from(0))
        );
        match test_struct.reset_count().unwrap() {
            ResetCount::Count(_) => panic!("expected unknown"),
            ResetCount::Unknown => (),
        }
        match test_struct.reset_limit().unwrap() {
            ResetLimit::Count(_) => panic!("expected unknown"),
            ResetLimit::Unknown => (),
        }
        match test_struct.timer_interval().unwrap() {
            TimerInterval::Minutes(_) => panic!("expected unknown"),
            TimerInterval::Unknown => (),
        }
        match test_struct.timeout().unwrap() {
            Timeout::Minutes(_) => panic!("expected unknown"),
            Timeout::Unknown => (),
        }
    }
}
