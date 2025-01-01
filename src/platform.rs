#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub(crate) use linux::*;

use crate::element::{Buffer, MouseEvent};

pub(crate) trait Platform: 'static {
    fn create_window(&self, buffer: &Buffer);
    fn quit(&self);
    fn set_mouse_handler(&self, handler: Box<dyn Fn(MouseEvent)>);
}
