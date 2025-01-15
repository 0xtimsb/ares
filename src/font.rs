use crate::Color;
use fontdue::Font;
use std::sync::Arc;

pub struct FontManager {
  default_font: Arc<Font>,
}

pub struct TextBitmap {
  pub buffer: Vec<u8>,
  pub width: usize,
  pub height: usize,
}

impl FontManager {
  pub fn new() -> Self {
    let font_data = include_bytes!("../assets/Roboto-Regular.ttf");
    let font = Font::from_bytes(font_data as &[u8], fontdue::FontSettings::default())
      .expect("Failed to load font");

    Self {
      default_font: Arc::new(font),
    }
  }

  pub fn render_text(&self, text: &str, size: f32, color: Color) -> TextBitmap {
    let rasterized = text
      .chars()
      .map(|c| self.default_font.rasterize(c, size))
      .collect::<Vec<_>>();

    let width = rasterized
      .iter()
      .map(|(m, _)| m.advance_width.ceil() as usize)
      .sum();
    let height = rasterized
      .iter()
      .map(|(m, _)| m.height + m.ymin.abs() as usize)
      .max()
      .unwrap_or(0);

    let mut buffer = vec![0u8; width * height * 4];
    let stride = width;

    let mut cursor_x = 0;
    for (metrics, bitmap) in rasterized {
      let y_offset = metrics.ymin.abs() as usize;

      for y in 0..metrics.height {
        for x in 0..metrics.width {
          let alpha = bitmap[y * metrics.width + x];
          let pixel_index = ((y + y_offset) * stride + (cursor_x + x)) * 4;

          if pixel_index + 3 >= buffer.len() {
            continue;
          }

          buffer[pixel_index] = color.b;
          buffer[pixel_index + 1] = color.g;
          buffer[pixel_index + 2] = color.r;
          buffer[pixel_index + 3] = alpha;
        }
      }
      cursor_x += metrics.advance_width.ceil() as usize;
    }

    TextBitmap {
      buffer,
      width,
      height,
    }
  }
}
