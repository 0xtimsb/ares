use taffy::prelude::*;

use crate::utils::Color;

pub trait Render {
    fn render(&self) -> Element;
}

#[derive(Clone)]
pub struct Element {
    pub style: Style,
    pub children: Vec<Element>,
    pub background_color: Option<Color>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
            background_color: None,
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

    pub fn to_buffer(&mut self, available_space: Size<AvailableSpace>) -> Buffer {
        let mut taffy = TaffyTree::new();

        let mut root_container = Element::new()
            .with_style(Style {
                size: Size {
                    width: match available_space.width {
                        AvailableSpace::Definite(points) => Dimension::Length(points),
                        _ => Dimension::Length(0.0),
                    },
                    height: match available_space.height {
                        AvailableSpace::Definite(points) => Dimension::Length(points),
                        _ => Dimension::Length(0.0),
                    },
                },
                ..Style::default()
            })
            .with_children(vec![self.clone()]);

        let root_node = root_container.prepaint(&mut taffy, available_space);
        root_container.paint(&taffy, root_node)
    }

    fn prepaint(&mut self, taffy: &mut TaffyTree, available_space: Size<AvailableSpace>) -> NodeId {
        fn create_node(taffy: &mut TaffyTree, element: &mut Element) -> NodeId {
            let child_nodes: Vec<_> = element
                .children
                .iter_mut()
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

    fn paint(&mut self, taffy: &TaffyTree, root_node: NodeId) -> Buffer {
        let root_layout = taffy.layout(root_node).unwrap();
        let buffer_size = (root_layout.size.width * root_layout.size.height * 4.0) as usize;
        let mut buffer_data = vec![0u8; buffer_size];
        let stride = root_layout.size.width.round() as usize;

        fn render_recursive(
            taffy: &TaffyTree,
            element: &mut Element,
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
            for (child, child_node) in element
                .children
                .iter_mut()
                .zip(taffy.children(node).unwrap())
            {
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
}

pub struct Buffer {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
