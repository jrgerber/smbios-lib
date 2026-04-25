#[cfg(any(target_os = "macos", target_os = "ios"))]
mod platform;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use platform::*;

