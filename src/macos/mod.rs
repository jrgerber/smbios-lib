#[cfg(any(target_os = "macos", target_os = "ios"))]
mod platform;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use platform::*;

pub use std::convert::TryInto;
pub use std::{fmt, fs, io};
