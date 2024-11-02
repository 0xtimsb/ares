mod platform;

pub use platform::*;

pub fn create_window() -> Result<(), Box<dyn std::error::Error>> {
    platform::create_window()
}
