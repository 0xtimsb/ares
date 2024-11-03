use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::platform::{Platform, X11Client};

type AppCell = RefCell<AppContext>;

pub struct AppContext {
    pub(crate) this: Weak<AppCell>,
    pub(crate) platform: Rc<dyn Platform>,
}

pub struct App(Rc<AppCell>);

impl App {
    pub fn new() -> Self {
        Self(AppContext::new(Rc::new(X11Client::new())))
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut AppContext),
    {
        let this = self.0.clone();
        let platform = self.0.borrow().platform.clone();
        platform.run(Box::new(move || {
            let cx = &mut *this.borrow_mut();
            on_finish_launching(cx);
        }));
    }
}

impl AppContext {
    pub(crate) fn new(platform: Rc<dyn Platform>) -> Rc<AppCell> {
        let app = Rc::new_cyclic(|this| {
            RefCell::new(AppContext {
                this: this.clone(),
                platform: platform.clone(),
            })
        });
        app
    }

    pub fn quit(&self) {
        self.platform.quit();
    }
}
