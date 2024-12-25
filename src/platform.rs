#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub(crate) use linux::*;

pub(crate) trait Platform: 'static {
    fn run(&self);
    fn quit(&self);
}
