use taffy::{Dimension, Display, Style};

pub trait Stylable {
  fn style(&mut self) -> &mut Style;

  fn w(mut self, width: f32) -> Self
  where
    Self: Sized,
  {
    self.style().size.width = Dimension::Length(width);
    self
  }

  fn h(mut self, height: f32) -> Self
  where
    Self: Sized,
  {
    self.style().size.height = Dimension::Length(height);
    self
  }

  fn display(mut self, display: Display) -> Self
  where
    Self: Sized,
  {
    self.style().display = display;
    self
  }
}
