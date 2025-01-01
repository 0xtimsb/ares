#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub(crate) use linux::*;

use crate::component::Buffer;

pub(crate) trait Platform: 'static {
    fn create_window(&self, buffer: &Buffer);
    fn quit(&self);
}
