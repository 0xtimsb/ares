#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub(crate) use linux::*;

use crate::WindowSettings;

pub(crate) trait Platform: 'static {
    fn open_window(&self, settings: WindowSettings);
    fn quit(&self);
}
