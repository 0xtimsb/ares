use crate::platform::Platform;

// x11, wayland, will implement this trait
// run method will contain platform specific logic to start the app
pub trait LinuxClient {
    fn run(&self);
}

// here LinuxClient trait implements Platform trait
// so, when platform run method is called, it will call LinuxClient's run method
impl<P: LinuxClient + 'static> Platform for P {
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        // this is wrapper for user's actual callback
        // platform it self run some code and call actual callback with whatever params they might want to give to the user
        on_finish_launching();
        // this is where platform specific code starts
        LinuxClient::run(self);
    }

    fn quit(&self) {}
}
