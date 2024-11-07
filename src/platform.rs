#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub(crate) use linux::*;

pub(crate) trait Platform: 'static {
    // Box is used here cause FnOnce is a trait object and needs to be sized.
    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>);
    fn quit(&self);
}
