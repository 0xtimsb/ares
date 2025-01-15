pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
}

pub type ClickCallback = Box<dyn Fn(&MouseEvent)>;
