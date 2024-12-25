mod platform;
#[cfg(feature = "wayland")]
mod wayland;

pub(crate) use platform::*;
#[cfg(feature = "wayland")]
pub(crate) use wayland::*;
