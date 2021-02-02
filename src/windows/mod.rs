#[cfg(target_family = "windows")]
mod win_struct;

#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "windows")]
pub use win_struct::*;

#[cfg(target_family = "windows")]
pub use windows::*;

pub use std::convert::TryInto;
pub use std::{fmt, fs, io};
