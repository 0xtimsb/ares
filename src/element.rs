use taffy::prelude::*;

use crate::utils::Color;

pub trait Render {
    fn render(&self) -> Element;
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

type ClickCallback = Box<dyn Fn(&MouseEvent)>;

pub struct Element {
    pub style: Style,
    pub children: Vec<Element>,
    pub background_color: Option<Color>,
    pub click_handler: Option<ClickCallback>,
}

pub fn div() -> Element {
    Element::new()
}

impl Element {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
            background_color: None,
            click_handler: None,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn compute_layout(
        &self,
        taffy: &mut TaffyTree,
        available_space: Size<AvailableSpace>,
    ) -> NodeId {
        fn create_node(taffy: &mut TaffyTree, element: &Element) -> NodeId {
            let child_nodes: Vec<_> = element
                .children
                .iter()
                .map(|child| create_node(taffy, child))
                .collect();
            taffy
                .new_with_children(element.style.clone(), &child_nodes)
                .unwrap()
        }
        let root_node = create_node(taffy, self);
        taffy.compute_layout(root_node, available_space).unwrap();
        root_node
    }

    pub fn paint(&self, taffy: &TaffyTree, root_node: NodeId) -> Buffer {
        let root_layout = taffy.layout(root_node).unwrap();
        let buffer_size = (root_layout.size.width * root_layout.size.height * 4.0) as usize;
        let mut buffer_data = vec![0u8; buffer_size];
        let stride = root_layout.size.width.round() as usize;

        fn render_recursive(
            taffy: &TaffyTree,
            element: &Element,
            node: NodeId,
            buffer: &mut [u8],
            stride: usize,
        ) {
            let layout = taffy.layout(node).unwrap();

            if let Some(color) = element.background_color {
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
            for (child, child_node) in element.children.iter().zip(taffy.children(node).unwrap()) {
                render_recursive(taffy, child, child_node, buffer, stride);
            }
        }

        render_recursive(taffy, self, root_node, &mut buffer_data, stride);

        Buffer {
            data: buffer_data,
            width: root_layout.size.width as u32,
            height: root_layout.size.height as u32,
        }
    }

    pub fn w(mut self, value: f32) -> Self {
        self.style.size.width = Dimension::Length(value);
        self
    }

    pub fn h(mut self, value: f32) -> Self {
        self.style.size.height = Dimension::Length(value);
        self
    }

    pub fn bg(self, color: Color) -> Self {
        self.with_background_color(color)
    }

    pub fn display(mut self, display: Display) -> Self {
        self.style.display = display;
        self
    }

    pub fn on_mouse_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&MouseEvent) + 'static,
    {
        self.click_handler = Some(Box::new(callback));
        self
    }
}

pub struct Buffer {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
