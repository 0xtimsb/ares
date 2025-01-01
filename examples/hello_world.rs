use ares::{App, Color, Element, Render, WindowSettings};
use taffy::prelude::*;

struct Counter {
    count: i32,
}

impl Render for Counter {
    fn render(&self) -> Element {
        Element::new()
            .with_style(Style {
                display: Display::Flex,
                size: Size {
                    width: Dimension::Length(100.0),
                    height: Dimension::Length(40.0),
                },
                ..Default::default()
            })
            .with_background_color(Color::RED)
    }
}

fn main() {
    let app = App::new();
    let counter = Counter { count: 0 };
    app.open_window(counter, WindowSettings::new(512, 512));
}
