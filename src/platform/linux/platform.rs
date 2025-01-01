use crate::component::Buffer;
use crate::platform::Platform;

pub trait LinuxClient {
    fn create_window(&self, buffer: &Buffer);
    fn quit(&self);
}

impl<P: LinuxClient + 'static> Platform for P {
    fn create_window(&self, buffer: &Buffer) {
        LinuxClient::create_window(self, buffer);
    }

    fn quit(&self) {
        LinuxClient::quit(self);
    }
}
