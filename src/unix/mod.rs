#[cfg(any(target_os = "linux", target_os = "android"))]
mod platform;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use platform::*;

pub use std::convert::TryInto;
pub use std::{fmt, fs, io};
