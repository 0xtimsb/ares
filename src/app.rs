use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::platform::{Platform, WaylandClient};

pub struct AppContext {
    pub(crate) platform: Rc<dyn Platform>,
}

impl AppContext {
    pub(crate) fn new(platform: Rc<dyn Platform>) -> Rc<RefCell<AppContext>> {
        let app = Rc::new(RefCell::new(AppContext {
            platform: platform.clone(),
        }));
        app
    }

    pub fn quit(&self) {
        self.platform.quit();
    }
}

pub struct App(Rc<RefCell<AppContext>>);

impl App {
    pub fn new() -> Self {
        Self(AppContext::new(Rc::new(WaylandClient::new())))
    }

    pub fn run(self) {
        let platform = self.0.borrow().platform.clone();
        platform.run();
    }
}
