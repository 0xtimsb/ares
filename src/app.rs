use std::cell::RefCell;
use std::rc::Rc;

use taffy::{AvailableSpace, Size, TaffyTree};

use crate::{
  ables::{
    paintable::{Paintable, Render},
    stylable::Stylable,
    touchable::find_mouse_target,
  },
  div,
  element::div::Div,
  platform::{Platform, WaylandClient},
  WindowSettings,
};

pub struct AppContext {
  platform: Box<dyn Platform>,
  taffy: TaffyTree,
  root_node_id: Option<taffy::NodeId>,
  root_element: Option<Div>,
}

pub struct App(Rc<RefCell<AppContext>>);

impl App {
  pub fn new() -> Self {
    Self(Rc::new(RefCell::new(AppContext {
      platform: Box::new(WaylandClient::new()),
      taffy: TaffyTree::new(),
      root_node_id: None,
      root_element: None,
    })))
  }

  pub fn open_window<T: Render + 'static>(&self, root: T, settings: WindowSettings) {
    let content = root.render();

    let root = div()
      .w(settings.width as f32)
      .h(settings.height as f32)
      .child(content);

    {
      let mut app = self.0.borrow_mut();
      app.root_node_id = Some(root.compute_layout(
        &mut app.taffy,
        Size {
          width: AvailableSpace::Definite(settings.width as f32),
          height: AvailableSpace::Definite(settings.height as f32),
        },
      ));
      app.root_element = Some(root);
    }

    let ctx = self.0.borrow();
    let app = Rc::clone(&self.0);
    ctx.platform.set_mouse_handler(Box::new(move |event| {
      let ctx = app.borrow();
      if let Some(target) = find_mouse_target(
        ctx.root_element.as_ref().unwrap(),
        &ctx.taffy,
        ctx.root_node_id.unwrap(),
        event.x,
        event.y,
      ) {
        if let Some(handler) = &target.click_handler {
          handler(&event);
        }
      }
    }));

    let buffer = ctx
      .root_element
      .as_ref()
      .unwrap()
      .paint(&ctx.taffy, ctx.root_node_id.unwrap());

    ctx.platform.create_window(&buffer);
  }

  pub fn quit(&self) {
    self.0.borrow().platform.quit();
  }
}
