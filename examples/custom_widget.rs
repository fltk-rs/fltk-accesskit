#![windows_subsystem = "windows"]

use accesskit::{Action, Node, NodeId, Rect, Role};
use fltk::{enums::*, prelude::*, *};
use fltk_accesskit::{builder, Accessible, AccessibleApp};

#[derive(Clone)]
struct MyButton {
    f: button::Button,
}

impl MyButton {
    pub fn new(label: &str) -> Self {
        let mut f = button::Button::default_fill().with_label(label);
        f.set_frame(FrameType::FlatBox);
        Self { f }
    }
}

impl Accessible for MyButton {
    fn make_node(&self, _children: &[NodeId]) -> (NodeId, Node) {
        let node_id = NodeId(self.as_widget_ptr() as usize as u64);
        let node = {
            let mut builder = Node::new(Role::Button);
            builder.set_bounds(Rect {
                x0: self.x() as f64,
                y0: self.y() as f64,
                x1: (self.w() + self.x()) as f64,
                y1: (self.h() + self.y()) as f64,
            });
            builder.set_label(&*self.label());
            builder.add_action(Action::Focus);
            builder.add_action(Action::Click);
            builder
        };
        (node_id, node)
    }
}

fltk::widget_extends!(MyButton, button::Button, f);

fn main() {
    let a = app::App::default();
    let mut w = window::Window::default()
        .with_size(400, 300)
        .with_label("Hello Window");
    let col = group::Flex::default_fill().column();
    let mut b1 = MyButton::new("Click 1");
    let mut b2 = MyButton::new("Click 2");
    b2.set_callback(|_| println!("clicked 2"));
    col.end();
    w.end();
    w.show();

    let ac = builder(w).attach();

    b1.set_callback(|_| println!("clicked 1"));

    a.run_with_accessibility(ac).unwrap();
}
