use taffy::{prelude::TaffyMaxContent, AvailableSpace, NodeId, Size, Style, TaffyTree};

use crate::{
  ables::{
    childable::Childable,
    paintable::{Buffer, Paintable},
    stylable::Stylable,
    touchable::{ClickCallback, Touchable},
  },
  Color,
};

pub struct Div {
  style: Style,
  children: Vec<Box<dyn Paintable>>,
  pub click_handler: Option<ClickCallback>,
  background_color: Option<Color>,
}

impl Div {
  fn new() -> Self {
    Self {
      style: Style::default(),
      background_color: None,
      children: Vec::new(),
      click_handler: None,
    }
  }

  pub fn bg(mut self, color: Color) -> Self {
    self.background_color = Some(color);
    self
  }

  pub fn child<P: Paintable + 'static>(mut self, child: P) -> Self {
    self.children = vec![Box::new(child)];
    self
  }

  pub fn children<P: Paintable + 'static>(mut self, children: Vec<P>) -> Self {
    self.children = children
      .into_iter()
      .map(|c| Box::new(c) as Box<dyn Paintable>)
      .collect();
    self
  }
}

impl Stylable for Div {
  fn style(&mut self) -> &mut Style {
    &mut self.style
  }
}

impl Touchable for Div {
  fn click_handler(&self) -> Option<&ClickCallback> {
    self.click_handler.as_ref()
  }

  fn set_click_handler(&mut self, handler: Option<ClickCallback>) {
    self.click_handler = handler;
  }
}

impl Childable for Div {
  fn children(&self) -> &Vec<Box<dyn Paintable>> {
    &self.children
  }
}

impl Paintable for Div {
  fn as_childable(&self) -> Option<&dyn Childable> {
    Some(self)
  }

  fn compute_layout(&self, taffy: &mut TaffyTree, available_space: Size<AvailableSpace>) -> NodeId {
    fn create_node(
      taffy: &mut TaffyTree,
      style: &Style,
      children: &[Box<dyn Paintable>],
    ) -> NodeId {
      let child_nodes: Vec<_> = children
        .iter()
        .map(|child| child.compute_layout(taffy, Size::MAX_CONTENT))
        .collect();
      taffy
        .new_with_children(style.clone(), &child_nodes)
        .unwrap()
    }

    let root_node = create_node(taffy, &self.style, &self.children);
    taffy.compute_layout(root_node, available_space).unwrap();
    root_node
  }

  fn paint(&self, taffy: &TaffyTree, root_node: NodeId) -> Buffer {
    let root_layout = taffy.layout(root_node).unwrap();
    let buffer_size = (root_layout.size.width * root_layout.size.height * 4.0) as usize;
    let mut buffer_data = vec![0u8; buffer_size];
    let stride = root_layout.size.width.round() as usize;

    fn render_recursive(
      taffy: &TaffyTree,
      element: &dyn Paintable,
      node: NodeId,
      buffer: &mut [u8],
      stride: usize,
    ) {
      let layout = taffy.layout(node).unwrap();

      if let Some(div) = element.downcast_ref::<Div>() {
        if let Some(color) = div.background_color {
          let x = layout.location.x.round() as usize;
          let y = layout.location.y.round() as usize;
          let width = layout.size.width.round() as usize;
          let height = layout.size.height.round() as usize;

          for row in y..y + height {
            for col in x..x + width {
              let pixel_index = (row * stride + col) * 4;
              if pixel_index + 3 < buffer.len() {
                buffer[pixel_index] = color.b;
                buffer[pixel_index + 1] = color.g;
                buffer[pixel_index + 2] = color.r;
                buffer[pixel_index + 3] = color.a;
              }
            }
          }
        }
      }

      if let Some(childable) = element.as_childable() {
        for (child, child_node) in childable
          .children()
          .iter()
          .zip(taffy.children(node).unwrap())
        {
          render_recursive(taffy, &**child, child_node, buffer, stride);
        }
      }
    }

    render_recursive(taffy, self, root_node, &mut buffer_data, stride);

    Buffer {
      data: buffer_data,
      width: root_layout.size.width as u32,
      height: root_layout.size.height as u32,
    }
  }
}

pub fn div() -> Div {
  Div::new()
}
