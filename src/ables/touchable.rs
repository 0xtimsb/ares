use taffy::TaffyTree;

use super::paintable::Paintable;

pub trait Touchable {
  fn click_handler(&self) -> Option<&ClickCallback>;
  fn set_click_handler(&mut self, handler: Option<ClickCallback>);

  fn on_click<F>(mut self, callback: F) -> Self
  where
    F: Fn(&MouseEvent) + 'static,
    Self: Sized,
  {
    self.set_click_handler(Some(Box::new(callback)));
    self
  }
}

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

pub fn find_mouse_target<'a, T>(
  element: &'a T,
  taffy: &'a TaffyTree,
  node_id: taffy::NodeId,
  x: f32,
  y: f32,
) -> Option<&'a T>
where
  T: Touchable + Paintable,
{
  let layout = taffy.layout(node_id).unwrap();

  if x < layout.location.x
    || y < layout.location.y
    || x > layout.location.x + layout.size.width
    || y > layout.location.y + layout.size.height
  {
    return None;
  }

  if let Some(childable) = element.as_childable() {
    for (child_idx, child) in childable.children().iter().enumerate().rev() {
      let child_id = taffy.children(node_id).unwrap()[child_idx];
      if let Some(touchable) = child.as_any().downcast_ref::<T>() {
        if let Some(target) = find_mouse_target(touchable, taffy, child_id, x, y) {
          return Some(target);
        }
      }
    }
  }

  if element.click_handler().is_some() {
    Some(element)
  } else {
    None
  }
}
