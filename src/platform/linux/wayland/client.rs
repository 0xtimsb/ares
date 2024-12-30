use std::cell::RefCell;
use std::io::Write;
use std::os::fd::BorrowedFd;
use std::os::unix::io::AsRawFd;

use calloop::EventLoop;
use calloop_wayland_source::WaylandSource;
use memfd::MemfdOptions;
use wayland_client::protocol::wl_shm_pool;
use wayland_client::{
    delegate_noop,
    globals::{registry_queue_init, GlobalListContents},
    protocol::{wl_buffer, wl_compositor, wl_registry, wl_shm, wl_surface},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

use crate::platform::LinuxClient;
use crate::WindowSettings;

struct WaylandClientState {
    event_loop: Option<EventLoop<'static, WaylandClientState>>,
    queue_handle: QueueHandle<Self>,
    wl_compositor: wl_compositor::WlCompositor,
    wl_shm: wl_shm::WlShm,
    xdg_wm_base: xdg_wm_base::XdgWmBase,
    wl_surface: Option<wl_surface::WlSurface>,
    xdg_surface: Option<xdg_surface::XdgSurface>,
    xdg_toplevel: Option<xdg_toplevel::XdgToplevel>,
    buffer: Option<wl_buffer::WlBuffer>,
}

impl WaylandClientState {
    fn cleanup(&mut self) {
        if let Some(toplevel) = self.xdg_toplevel.take() {
            toplevel.destroy();
        }
        if let Some(xdg_surface) = self.xdg_surface.take() {
            xdg_surface.destroy();
        }
        if let Some(surface) = self.wl_surface.take() {
            surface.destroy();
        }
        if let Some(buffer) = self.buffer.take() {
            buffer.destroy();
        }
    }
}

pub struct WaylandClient(RefCell<WaylandClientState>);

impl WaylandClient {
    pub fn new() -> Self {
        let connection = Connection::connect_to_env().unwrap();
        let (globals, event_queue) =
            registry_queue_init::<WaylandClientState>(&connection).unwrap();
        let qh = event_queue.handle();
        let event_loop = EventLoop::<WaylandClientState>::try_new().unwrap();

        WaylandSource::new(connection, event_queue)
            .insert(event_loop.handle())
            .unwrap();

        WaylandClient(RefCell::new(WaylandClientState {
            event_loop: Some(event_loop),
            queue_handle: qh.clone(),
            wl_compositor: globals.bind(&qh, 1..=5, ()).unwrap(),
            wl_shm: globals.bind(&qh, 1..=1, ()).unwrap(),
            xdg_wm_base: globals.bind(&qh, 1..=5, ()).unwrap(),
            wl_surface: None,
            xdg_surface: None,
            xdg_toplevel: None,
            buffer: None,
        }))
    }

    fn create_buffer(
        &self,
        state: &WaylandClientState,
        settings: &WindowSettings,
    ) -> wl_buffer::WlBuffer {
        let row = settings.width * 4;
        let rect = (row * settings.height) as usize;

        let memfd = MemfdOptions::new()
            .allow_sealing(false)
            .create("buffer")
            .expect("Failed to create memfd");
        memfd
            .as_file()
            .set_len(rect as u64)
            .expect("Failed to set file size");

        let mut data = vec![0u8; rect];
        for chunk in data.chunks_exact_mut(4) {
            chunk.copy_from_slice(&[0xff, 0x00, 0x00, 0xff]); // B, G, R, A
        }

        memfd
            .as_file()
            .write_all(&data)
            .expect("Failed to write pixel data");

        let borrowed_fd = unsafe { BorrowedFd::borrow_raw(memfd.as_raw_fd()) };
        let pool = state
            .wl_shm
            .create_pool(borrowed_fd, rect as i32, &state.queue_handle, ());

        pool.create_buffer(
            0,
            settings.width as i32,
            settings.height as i32,
            row as i32,
            wl_shm::Format::Xrgb8888,
            &state.queue_handle,
            (),
        )
    }
}

impl Drop for WaylandClient {
    fn drop(&mut self) {
        let mut state = self.0.borrow_mut();
        state.cleanup();
    }
}

impl LinuxClient for WaylandClient {
    fn open_window(&self, settings: WindowSettings) {
        let mut state = self.0.borrow_mut();

        let surface = state.wl_compositor.create_surface(&state.queue_handle, ());
        let xdg_surface = state
            .xdg_wm_base
            .get_xdg_surface(&surface, &state.queue_handle, ());
        let xdg_toplevel = xdg_surface.get_toplevel(&state.queue_handle, ());

        let buffer = self.create_buffer(&state, &settings);
        surface.attach(Some(&buffer), 0, 0);
        surface.commit();

        state.buffer = Some(buffer);
        state.wl_surface = Some(surface);
        state.xdg_surface = Some(xdg_surface);
        state.xdg_toplevel = Some(xdg_toplevel);

        let mut event_loop = state.event_loop.take().unwrap();
        event_loop.run(None, &mut *state, |_| {}).unwrap();
    }

    fn quit(&self) {
        let mut state = self.0.borrow_mut();
        state.cleanup();
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for WaylandClientState {
    fn event(
        _: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = event {
            xdg_surface.ack_configure(serial);
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for WaylandClientState {
    fn event(
        _: &mut Self,
        wm_base: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for WaylandClientState {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

delegate_noop!(WaylandClientState: ignore wl_buffer::WlBuffer);
delegate_noop!(WaylandClientState: ignore wl_shm_pool::WlShmPool);
delegate_noop!(WaylandClientState: ignore xdg_toplevel::XdgToplevel);
delegate_noop!(WaylandClientState: ignore wl_compositor::WlCompositor);
delegate_noop!(WaylandClientState: ignore wl_surface::WlSurface);
delegate_noop!(WaylandClientState: ignore wl_shm::WlShm);
