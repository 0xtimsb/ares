use crate::platform::Platform;

pub trait LinuxClient {
    fn run(&self);
}

impl<P: LinuxClient + 'static> Platform for P {
    fn run(&self) {
        LinuxClient::run(self);
    }

    fn quit(&self) {}
}
