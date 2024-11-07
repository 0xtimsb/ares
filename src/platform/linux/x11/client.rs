use std::{cell::RefCell, rc::Rc};

use x11rb::xcb_ffi::XCBConnection;

use crate::platform::LinuxClient;

pub struct X11ClientState {
    pub(crate) xcb_connection: Rc<XCBConnection>,
}

#[derive(Clone)]
pub(crate) struct X11Client(Rc<RefCell<X11ClientState>>);

impl X11Client {
    // returns a new instance of X11Client
    pub(crate) fn new() -> Self {
        let (conn, screen_num) = XCBConnection::connect(None).unwrap();

        let xcb_connection = Rc::new(conn);

        X11Client(Rc::new(RefCell::new(X11ClientState { xcb_connection })))
    }
}

impl LinuxClient for X11Client {
    // x11 specific logic to start the app
    fn run(&self) {}
}
