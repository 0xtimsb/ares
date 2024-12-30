use crate::{platform::Platform, WindowSettings};

pub trait LinuxClient {
    fn open_window(&self, settings: WindowSettings);
    fn quit(&self);
}

impl<P: LinuxClient + 'static> Platform for P {
    fn open_window(&self, settings: WindowSettings) {
        LinuxClient::open_window(self, settings);
    }

    fn quit(&self) {
        LinuxClient::quit(self);
    }
}
