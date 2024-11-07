use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::platform::{Platform, X11Client};

pub struct AppContext {
    // we might need self reference in future, for say accessing app context in callback. todo: update use case here.
    pub(crate) this: Weak<RefCell<AppContext>>,
    // dyn is used as platform is trait, and we don't know yet which type. it can be anything that implements Platform trait.
    pub(crate) platform: Rc<dyn Platform>,
}

impl AppContext {
    pub(crate) fn new(platform: Rc<dyn Platform>) -> Rc<RefCell<AppContext>> {
        let app = Rc::new_cyclic(|this| {
            RefCell::new(AppContext {
                this: this.clone(),
                platform: platform.clone(), // it's fine to not clone here too, but we do it just in case in future we need to change platform down the line.
            })
        });
        app
    }
    pub fn quit(&self) {
        self.platform.quit();
    }
}

// rc is used to allow multiple owners
// refcell is used to allow interior mutability
// note: need to be careful with borrow and borrow_mut
pub struct App(Rc<RefCell<AppContext>>);

impl App {
    pub fn new() -> Self {
        // current app only consist of app context, which takes platform as argument.
        Self(AppContext::new(Rc::new(X11Client::new())))
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        // 'static here means closure should not use any non-static variables.
        // FnOnce is used so that we can move data from closure. which doesn't happen in Fn trait.
        F: 'static + FnOnce(&mut AppContext),
    {
        // clone needed because the closure needs its own reference to the AppContext
        let this = self.0.clone();
        // clone platform to avoid keeping a borrow on self.0 across the async boundary
        let platform = self.0.borrow().platform.clone();
        // Box::new needed because Platform::run expects a trait object (Box<dyn FnOnce()>)
        // 'move' is required because the closure must own its captured values to be 'static
        platform.run(Box::new(move || {
            // Safe to borrow_mut here since we have our own clone of the Rc
            // and this is the only place accessing it in this closure
            let cx = &mut *this.borrow_mut();
            on_finish_launching(cx);
        }));
    }
}
