use crate::element::Buffer;
use crate::platform::Platform;
use crate::MouseEvent;

pub trait LinuxClient {
    fn create_window(&self, buffer: &Buffer);
    fn quit(&self);
    fn set_mouse_handler(&self, handler: Box<dyn Fn(MouseEvent)>);
}

impl<P: LinuxClient + 'static> Platform for P {
    fn create_window(&self, buffer: &Buffer) {
        LinuxClient::create_window(self, buffer);
    }

    fn quit(&self) {
        LinuxClient::quit(self);
    }

    fn set_mouse_handler(&self, handler: Box<dyn Fn(MouseEvent)>) {
        LinuxClient::set_mouse_handler(self, handler);
    }
}
