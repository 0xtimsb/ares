use crate::platform::{Platform, PlatformWindow};

pub trait LinuxClient {
    fn run(&self);
    fn open_window(&self) -> anyhow::Result<Box<dyn PlatformWindow>>;
}

impl<P: LinuxClient + 'static> Platform for P {
    fn run(&self) {
        LinuxClient::run(self);
    }

    fn open_window(&self) -> anyhow::Result<Box<dyn PlatformWindow>> {
        LinuxClient::open_window(self)
    }

    fn quit(&self) {}
}
