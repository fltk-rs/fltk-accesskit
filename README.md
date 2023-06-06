# fltk-accesskit

fltk-accesskit is an fltk accesskit adapter made to work with the fltk gui crate.

Example code:
```rust
use fltk::{prelude::*, *};
use fltk_accesskit::{AccessibilityContext, AccessibleApp};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Oxy);
    let mut w = window::Window::default()
        .with_size(400, 300)
        .with_label("Hello fltk-accesskit");
    let col = group::Flex::default()
        .with_size(200, 100)
        .center_of_parent()
        .column();
    let inp = input::Input::default().with_id("inp").with_label("Enter name:");
    let mut btn = button::Button::default().with_label("Greet");
    let out = output::Output::default().with_id("out");
    col.end();
    w.end();
    w.make_resizable(true);
    w.show();

    btn.set_callback(btn_callback);

    let ac = AccessibilityContext::new(
        w,
        vec![Box::new(inp), Box::new(btn), Box::new(out)],
    );

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
```

To use fltk-accesskit, add fltk-accesskit to your Cargo.toml:
```toml
[dependencies]
fltk = "1.4"
fltk-accesskit = "0.1.0"
```

![image](https://github.com/cross-rs/cross/assets/37966791/c9f16b3b-9ec4-4b23-83ae-fc8759353aa9)