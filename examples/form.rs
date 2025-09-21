#![windows_subsystem = "windows"]

use fltk::{prelude::*, *};
use fltk_accesskit::{builder, AccessibleApp};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Oxy);
    let mut w = window::Window::default()
        .with_size(400, 300)
        .with_label("Hello fltk-accesskit");
    let col = group::Flex::default()
        .with_size(200, 100)
        .center_of_parent()
        .column();
    let _inp = input::Input::default()
        .with_id("inp")
        .with_label("Enter name:");
    let mut btn = button::Button::default().with_label("Greet");
    let _out = output::Output::default().with_id("out");
    col.end();
    w.end();
    w.make_resizable(true);
    w.show();

    btn.set_callback(btn_callback);

    let ac = builder(w).attach();

    a.run_with_accessibility(ac).unwrap();
}

fn btn_callback(_btn: &mut button::Button) {
    let inp: input::Input = app::widget_from_id("inp").unwrap();
    let mut out: output::Output = app::widget_from_id("out").unwrap();
    let name = inp.value();
    if name.is_empty() {
        return;
    }
    out.set_value(&format!("Hello {}", name));
}
