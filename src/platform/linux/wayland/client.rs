use std::cell::RefCell;
use std::io::Write;
use std::os::fd::BorrowedFd;
use std::os::unix::io::AsRawFd;

use calloop::EventLoop;
use calloop_wayland_source::WaylandSource;
use memfd::MemfdOptions;
use wayland_client::protocol::wl_shm_pool;
use wayland_client::WEnum;
use wayland_client::{
  delegate_noop,
  globals::{registry_queue_init, GlobalListContents},
  protocol::{wl_buffer, wl_compositor, wl_pointer, wl_registry, wl_seat, wl_shm, wl_surface},
  Connection, Dispatch, QueueHandle,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

use crate::ables::paintable::Buffer;
use crate::ables::touchable::{MouseButton, MouseEvent};
use crate::platform::LinuxClient;

struct WaylandClientState {
  event_loop: Option<EventLoop<'static, WaylandClientState>>,
  queue_handle: QueueHandle<Self>,
  wl_compositor: wl_compositor::WlCompositor,
  wl_shm: wl_shm::WlShm,
  xdg_wm_base: xdg_wm_base::XdgWmBase,
  wl_surface: wl_surface::WlSurface,
  xdg_surface: xdg_surface::XdgSurface,
  xdg_toplevel: xdg_toplevel::XdgToplevel,
  wl_seat: wl_seat::WlSeat,
  wl_pointer: wl_pointer::WlPointer,
  current_x: f32,
  current_y: f32,
  mouse_handler: Option<Box<dyn Fn(MouseEvent)>>,
}

pub struct WaylandClient(RefCell<WaylandClientState>);

impl WaylandClient {
  pub fn new() -> Self {
    let connection = Connection::connect_to_env().unwrap();
    let (globals, event_queue) = registry_queue_init::<WaylandClientState>(&connection).unwrap();
    let qh = event_queue.handle();
    let event_loop = EventLoop::<WaylandClientState>::try_new().unwrap();

    WaylandSource::new(connection, event_queue)
      .insert(event_loop.handle())
      .unwrap();

    let wl_compositor: wl_compositor::WlCompositor = globals.bind(&qh, 1..=5, ()).unwrap();
    let wl_shm: wl_shm::WlShm = globals.bind(&qh, 1..=1, ()).unwrap();
    let xdg_wm_base: xdg_wm_base::XdgWmBase = globals.bind(&qh, 1..=5, ()).unwrap();

    let wl_surface = wl_compositor.create_surface(&qh, ());
    let xdg_surface = xdg_wm_base.get_xdg_surface(&wl_surface, &qh, ());
    let xdg_toplevel = xdg_surface.get_toplevel(&qh, ());

    let wl_seat: wl_seat::WlSeat = globals.bind(&qh, 1..=7, ()).unwrap();
    let wl_pointer = wl_seat.get_pointer(&qh, ());

    WaylandClient(RefCell::new(WaylandClientState {
      event_loop: Some(event_loop),
      queue_handle: qh.clone(),
      wl_compositor,
      wl_shm,
      xdg_wm_base,
      wl_surface,
      xdg_surface,
      xdg_toplevel,
      wl_seat,
      wl_pointer,
      current_x: 0.0,
      current_y: 0.0,
      mouse_handler: None,
    }))
  }
}

impl LinuxClient for WaylandClient {
  fn create_window(&self, buffer: &Buffer) {
    let mut state = self.0.borrow_mut();

    let buffer_size = buffer.data.len();
    let memfd = MemfdOptions::new()
      .allow_sealing(false)
      .create("buffer")
      .expect("Failed to create memfd");
    memfd
      .as_file()
      .set_len(buffer_size as u64)
      .expect("Failed to set memfd size");
    memfd
      .as_file()
      .write_all(&buffer.data)
      .expect("Failed to write pixel data");

    let borrowed_fd = unsafe { BorrowedFd::borrow_raw(memfd.as_raw_fd()) };
    let pool = state
      .wl_shm
      .create_pool(borrowed_fd, buffer_size as i32, &state.queue_handle, ());

    let wl_buffer = pool.create_buffer(
      0,
      buffer.width as i32,
      buffer.height as i32,
      (buffer.width * 4) as i32,
      wl_shm::Format::Xrgb8888,
      &state.queue_handle,
      (),
    );

    state.wl_surface.attach(Some(&wl_buffer), 0, 0);
    state.wl_surface.commit();

    let mut event_loop = state.event_loop.take().unwrap();
    event_loop
      .run(None, &mut *state, |state| {
        //
      })
      .unwrap();
  }

  fn quit(&self) {
    todo!()
  }

  fn set_mouse_handler(&self, handler: Box<dyn Fn(MouseEvent)>) {
    self.0.borrow_mut().mouse_handler = Some(handler);
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

impl Dispatch<wl_pointer::WlPointer, ()> for WaylandClientState {
  fn event(
    state: &mut Self,
    _: &wl_pointer::WlPointer,
    event: wl_pointer::Event,
    _: &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    match event {
      wl_pointer::Event::Button {
        button,
        state: button_state,
        ..
      } => {
        if button_state == WEnum::Value(wl_pointer::ButtonState::Released) {
          let button = match button {
            0x110 => MouseButton::Left,
            0x111 => MouseButton::Right,
            _ => return,
          };
          if let Some(handler) = &state.mouse_handler {
            handler(MouseEvent {
              x: state.current_x,
              y: state.current_y,
              button,
            });
          }
        }
      }
      wl_pointer::Event::Motion {
        surface_x,
        surface_y,
        ..
      } => {
        state.current_x = surface_x as f32;
        state.current_y = surface_y as f32;
      }
      _ => {}
    }
  }
}

delegate_noop!(WaylandClientState: ignore wl_buffer::WlBuffer);
delegate_noop!(WaylandClientState: ignore wl_shm_pool::WlShmPool);
delegate_noop!(WaylandClientState: ignore xdg_toplevel::XdgToplevel);
delegate_noop!(WaylandClientState: ignore wl_compositor::WlCompositor);
delegate_noop!(WaylandClientState: ignore wl_surface::WlSurface);
delegate_noop!(WaylandClientState: ignore wl_shm::WlShm);
delegate_noop!(WaylandClientState: ignore wl_seat::WlSeat);
