use ares::{App, WindowSettings};

fn main() {
    let app = App::new();
    app.open_window(WindowSettings::new(512, 512));
}
