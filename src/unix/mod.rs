#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
mod platform;

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
pub use platform::*;

pub use std::convert::TryInto;
pub use std::{fmt, fs, io};
