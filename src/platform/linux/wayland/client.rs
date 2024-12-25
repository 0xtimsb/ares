use std::{cell::RefCell, rc::Rc};
use wayland_client::Connection;

use crate::platform::LinuxClient;

pub struct WaylandClientState {
    pub(crate) connection: Connection,
}

pub(crate) struct WaylandClient(Rc<RefCell<WaylandClientState>>);

impl WaylandClient {
    pub(crate) fn new() -> Self {
        let connection = Connection::connect_to_env().unwrap();

        WaylandClient(Rc::new(RefCell::new(WaylandClientState { connection })))
    }
}

impl LinuxClient for WaylandClient {
    fn run(&self) {}
}
