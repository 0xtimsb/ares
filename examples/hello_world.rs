use ares::{div, text, App, Color, Paintable, Render, Stylable, Touchable, WindowSettings};
use taffy::prelude::*;

struct Counter {
  count: i32,
}

impl Render for Counter {
  fn render(&self) -> impl Paintable {
    div()
      .w(200.0)
      .h(200.0)
      .display(Display::Flex)
      .bg(Color::rgb(255, 255, 255))
      .child(text("Hello").size(24.0).color(Color::rgb(0, 0, 0)))
      .on_click(|click| {
        println!("Clicked: {:?}", click.button);
      })
  }
}

fn main() {
  let app = App::new();
  let counter = Counter { count: 0 };
  app.open_window(counter, WindowSettings::new(512, 512));
}
