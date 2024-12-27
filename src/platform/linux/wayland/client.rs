use std::cell::RefCell;

use calloop::EventLoop;
use calloop_wayland_source::WaylandSource;
use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::wl_registry::WlRegistry,
    Connection, Dispatch,
};

use crate::platform::{LinuxClient, PlatformWindow};

struct WaylandClientState {
    event_loop: Option<EventLoop<'static, WaylandClientState>>,
}

pub struct WaylandClient(RefCell<WaylandClientState>);

impl WaylandClient {
    pub fn new() -> Self {
        let connection = Connection::connect_to_env().unwrap();

        let (_globals, event_queue) =
            registry_queue_init::<WaylandClientState>(&connection).unwrap();

        let event_loop = EventLoop::<WaylandClientState>::try_new().unwrap();
        let handle = event_loop.handle();

        WaylandSource::new(connection, event_queue)
            .insert(handle)
            .unwrap();

        WaylandClient(RefCell::new(WaylandClientState {
            event_loop: Some(event_loop),
        }))
    }
}

impl LinuxClient for WaylandClient {
    fn open_window(&self) -> anyhow::Result<Box<dyn PlatformWindow>> {
        todo!("Implement window creation")
    }

    fn run(&self) {
        let mut event_loop = self
            .0
            .borrow_mut()
            .event_loop
            .take()
            .expect("You can only run app once");

        let mut data = self.0.borrow_mut();

        event_loop
            .run(None, &mut data, |_| {
                println!("event loop");
            })
            .unwrap();
    }
}

impl Dispatch<WlRegistry, GlobalListContents> for WaylandClientState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        event: <WlRegistry as wayland_client::Proxy>::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        println!("event: {:?}", event);
    }
}
