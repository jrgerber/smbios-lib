// common display logic

use std::fmt;

pub const NULL_DISPLAY: &'static str = "(null)";
pub const DEFAULT_NESTING_LEVEL: usize = 0;
pub const DEFAULT_INDENTATION: usize = 4;

pub trait SelfDescribe {
    fn get_self_description(nesting_level: usize, indentation: usize) -> fmt::Result;
}
