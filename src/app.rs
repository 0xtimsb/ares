use std::cell::RefCell;

use taffy::{AvailableSpace, Size};

use crate::{
    component::Render,
    platform::{Platform, WaylandClient},
    WindowSettings,
};

pub struct AppContext {
    platform: Box<dyn Platform>,
}

pub struct App(RefCell<AppContext>);

impl App {
    pub fn new() -> Self {
        Self(RefCell::new(AppContext {
            platform: Box::new(WaylandClient::new()),
        }))
    }

    pub fn open_window<T: Render + 'static>(&self, root: T, settings: WindowSettings) {
        let ctx = self.0.borrow();
        let buffer = root.render().to_buffer(Size {
            width: AvailableSpace::Definite(settings.width as f32),
            height: AvailableSpace::Definite(settings.height as f32),
        });
        ctx.platform.create_window(&buffer);
    }

    pub fn quit(&self) {
        self.0.borrow().platform.quit();
    }
}
