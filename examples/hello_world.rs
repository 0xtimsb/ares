use ares::{div, App, Color, Paintable, Render, Stylable, Touchable, WindowSettings};
use taffy::prelude::*;

struct Counter {
  count: i32,
}

impl Render for Counter {
  fn render(&self) -> impl Paintable {
    div()
      .w(100.0)
      .h(40.0)
      .display(Display::Flex)
      .bg(Color::RED)
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
