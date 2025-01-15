use super::paintable::Paintable;

pub trait Childable: Paintable {
  fn children(&self) -> &Vec<Box<dyn Paintable>>;
}
