pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
}

impl WindowSettings {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}
