use downcast_rs::{impl_downcast, Downcast};
use taffy::prelude::*;

use super::childable::Childable;

pub struct Buffer {
  pub data: Vec<u8>,
  pub width: u32,
  pub height: u32,
}

pub trait Paintable: Downcast + 'static {
  fn compute_layout(&self, taffy: &mut TaffyTree, available_space: Size<AvailableSpace>) -> NodeId;
  fn paint(&self, taffy: &TaffyTree, root_node: NodeId) -> Buffer;
  fn as_childable(&self) -> Option<&dyn Childable>;
}

impl_downcast!(Paintable);

pub trait Render {
  fn render(&self) -> impl Paintable;
}
