use std::cell::RefCell;
use std::rc::Rc;

use taffy::{AvailableSpace, Dimension, Size, Style, TaffyTree};

use crate::{
    element::Render,
    platform::{Platform, WaylandClient},
    Element, WindowSettings,
};

pub struct AppContext {
    platform: Box<dyn Platform>,
    taffy: TaffyTree,
    root_node_id: Option<taffy::NodeId>,
    root_element: Option<Element>,
}

pub struct App(Rc<RefCell<AppContext>>);

impl App {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(AppContext {
            platform: Box::new(WaylandClient::new()),
            taffy: TaffyTree::new(),
            root_node_id: None,
            root_element: None,
        })))
    }

    pub fn open_window<T: Render + 'static>(&self, root: T, settings: WindowSettings) {
        let element = root.render();

        let root_container = Element::new()
            .with_style(Style {
                size: Size {
                    width: Dimension::Length(settings.width as f32),
                    height: Dimension::Length(settings.height as f32),
                },
                ..Style::default()
            })
            .with_children(vec![element]);

        {
            let mut app = self.0.borrow_mut();
            app.root_node_id = Some(root_container.compute_layout(
                &mut app.taffy,
                Size {
                    width: AvailableSpace::Definite(settings.width as f32),
                    height: AvailableSpace::Definite(settings.height as f32),
                },
            ));
            app.root_element = Some(root_container);
        }

        let ctx = self.0.borrow();
        let app = Rc::clone(&self.0);
        ctx.platform.set_mouse_handler(Box::new(move |event| {
            let ctx = app.borrow();
            if let Some(target) = find_mouse_target(
                ctx.root_element.as_ref().unwrap(),
                &ctx.taffy,
                ctx.root_node_id.unwrap(),
                event.x,
                event.y,
            ) {
                if let Some(handler) = &target.click_handler {
                    handler(&event);
                }
            }
        }));

        let buffer = ctx
            .root_element
            .as_ref()
            .unwrap()
            .paint(&ctx.taffy, ctx.root_node_id.unwrap());

        ctx.platform.create_window(&buffer);
    }

    pub fn quit(&self) {
        self.0.borrow().platform.quit();
    }
}

fn find_mouse_target<'a>(
    element: &'a Element,
    taffy: &'a TaffyTree,
    node_id: taffy::NodeId,
    x: f32,
    y: f32,
) -> Option<&'a Element> {
    let layout = taffy.layout(node_id).unwrap();

    if x < layout.location.x
        || y < layout.location.y
        || x > layout.location.x + layout.size.width
        || y > layout.location.y + layout.size.height
    {
        return None;
    }

    for (child_idx, child) in element.children.iter().enumerate().rev() {
        let child_id = taffy.children(node_id).unwrap()[child_idx];
        if let Some(target) = find_mouse_target(child, taffy, child_id, x, y) {
            return Some(target);
        }
    }

    if element.click_handler.is_some() {
        Some(element)
    } else {
        None
    }
}
