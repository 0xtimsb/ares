use crate::{
    platform::{Platform, WaylandClient},
    WindowSettings,
};

pub struct App {
    platform: Box<dyn Platform>,
}

impl App {
    pub fn new() -> Self {
        Self {
            platform: Box::new(WaylandClient::new()),
        }
    }

    pub fn open_window(&self, settings: WindowSettings) {
        self.platform.open_window(settings);
    }

    pub fn quit(&self) {
        self.platform.quit();
    }
}
