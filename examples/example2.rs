#![windows_subsystem = "windows"]

use fltk_accesskit::{AccessibilityContext, AccessibleApp};
use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default();
    let mut w = window::Window::default()
        .with_size(400, 300)
        .with_label("Hello Window");
    let col = group::Flex::default_fill().column();
    let b1 = button::Button::default().with_label("Increment");
    let f = frame::Frame::default().with_label("0");
    let b2 = button::Button::default().with_label("Decrement");
    col.end();
    w.end();
    w.make_resizable(true);
    w.show();

    let ac = AccessibilityContext::new(w, vec![Box::new(b1), Box::new(f), Box::new(b2)]);

    a.run_with_accessibility(ac).unwrap();
}
