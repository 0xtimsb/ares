use crate::platform::Platform;

pub trait LinuxClient {
    fn run(&self);
}

impl<P: LinuxClient + 'static> Platform for P {
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching();
        LinuxClient::run(self);
    }

    fn quit(&self) {}
}
