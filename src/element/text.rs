use taffy::{prelude::*, NodeId, Style, TaffyTree};

use crate::{
  ables::{
    childable::Childable,
    paintable::{Buffer, Paintable},
  },
  font::FontManager,
  Color,
};

pub struct Text {
  content: String,
  size: f32,
  color: Color,
  style: Style,
  font_manager: FontManager,
}

impl Text {
  fn new(content: impl Into<String>) -> Self {
    Self {
      content: content.into(),
      size: 16.0,
      color: Color::rgb(0, 0, 0),
      style: Style::default(),
      font_manager: FontManager::new(),
    }
  }

  pub fn size(mut self, size: f32) -> Self {
    self.size = size;
    self
  }

  pub fn color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }
}

impl Paintable for Text {
  fn as_childable(&self) -> Option<&dyn Childable> {
    None
  }

  fn compute_layout(
    &self,
    taffy: &mut TaffyTree,
    _available_space: Size<AvailableSpace>,
  ) -> NodeId {
    println!("Computing layout for text: {}", self.content);
    let bitmap = self
      .font_manager
      .render_text(&self.content, self.size, self.color);

    let mut style = Style::default();
    style.size.width = Dimension::Length(bitmap.width as f32);
    style.size.height = Dimension::Length(bitmap.height as f32);

    taffy.new_leaf(style).unwrap()
  }

  fn paint(&self, taffy: &TaffyTree, node_id: NodeId) -> Buffer {
    let layout = taffy.layout(node_id).unwrap();
    let buffer_size = (layout.size.width * layout.size.height * 4.0) as usize;
    let mut buffer_data = vec![0u8; buffer_size];

    let text_bitmap = self
      .font_manager
      .render_text(&self.content, self.size, self.color);
    let stride = layout.size.width.round() as usize;
    let dest_x = layout.location.x.round() as usize;
    let dest_y = layout.location.y.round() as usize;

    for y in 0..text_bitmap.height {
      for x in 0..text_bitmap.width {
        let src_idx = (y * text_bitmap.width + x) * 4;
        let dest_idx = ((dest_y + y) * stride + dest_x + x) * 4;

        if dest_idx + 3 < buffer_data.len() {
          let alpha = text_bitmap.buffer[src_idx + 3] as f32 / 255.0;

          for c in 0..3 {
            let src_color = text_bitmap.buffer[src_idx + c] as f32;
            let dest_color = buffer_data[dest_idx + c] as f32;
            buffer_data[dest_idx + c] =
              ((src_color * alpha + dest_color * (1.0 - alpha)) as u8).min(255);
          }

          let new_alpha = (alpha * 255.0) as u8;
          buffer_data[dest_idx + 3] = new_alpha.max(buffer_data[dest_idx + 3]);
        }
      }
    }

    Buffer {
      data: buffer_data,
      width: layout.size.width as u32,
      height: layout.size.height as u32,
    }
  }
}

pub fn text(content: impl Into<String>) -> Text {
  Text::new(content)
}
