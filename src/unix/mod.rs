#[cfg(target_family = "unix")]
mod platform;

#[cfg(target_family = "unix")]
pub use platform::*;