#![windows_subsystem = "windows"]

use fltk::{prelude::*, *};
use fltk_accesskit::{builder, AccessibleApp};

fn main() {
    let a = app::App::default();
    let mut w = window::Window::default()
        .with_size(400, 300)
        .with_label("Hello Window");
    let col = group::Flex::default_fill().column();
    let _b1 = button::Button::default().with_label("Increment");
    let _f = frame::Frame::default().with_label("0");
    let _b2 = button::Button::default().with_label("Decrement");
    col.end();
    w.end();
    w.make_resizable(true);
    w.show();

    let ac = builder(w).attach();

    a.run_with_accessibility(ac).unwrap();
}
