#![allow(unused_imports)]
#![allow(unused_variables)]
use accesskit::{
    Action, ActionData, ActionHandler, ActionRequest, ActivationHandler, DeactivationHandler,
    Node, NodeId, Point, Rect, Size, Tree, TreeUpdate,
};
use fltk::{
    button, enums::*, input, misc, prelude::*, text, utils, valuator, widget, *,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::platform_adapter;

pub(crate) struct FltkActivationHandler {
    pub wids: Vec<(NodeId, Node)>,
    pub win_id: NodeId,
}

impl ActivationHandler for FltkActivationHandler {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        Some(TreeUpdate {
            nodes: self.wids.clone(),
            tree: Some(Tree::new(self.win_id)),
            focus: if let Some(focused) = app::focus() {
                let focused = focused.as_widget_ptr() as usize as u64;
                NodeId(focused)
            } else {
                self.win_id
            },
        })
    }
}

pub(crate) struct FltkActionHandler {
    tx: app::Sender<ActionRequest>,
}

impl ActionHandler for FltkActionHandler {
    fn do_action(&mut self, request: ActionRequest) {
        self.tx.send(request);
        app::awake();
    }
}

pub(crate) struct FltkDeactivationHandler {}

impl DeactivationHandler for FltkDeactivationHandler {
    fn deactivate_accessibility(&mut self) {}
}

#[derive(Clone)]
pub struct Adapter {
    adapter: Rc<RefCell<platform_adapter::Adapter>>,
}

impl Adapter {
    pub fn new(window: &window::Window, source: impl 'static + ActivationHandler + Send) -> Self {
        let (tx, rx) = app::channel::<ActionRequest>();
        let action_handler = FltkActionHandler { tx };
        let this = Self::with_action_handler(window, source, action_handler);
        // Drain action requests on the UI thread when awakened.
        let rx = Rc::new(RefCell::new(rx));
        app::awake_callback({
            let rx = rx.clone();
            move || {
                while let Some(req) = rx.borrow_mut().recv() {
                    unsafe {
                        let mut w = widget::Widget::from_widget_ptr(req.target.0 as _);
                        match req.action {
                            Action::Click => {
                                w.do_callback();
                            }
                            Action::Focus => {
                                let _ = w.take_focus();
                            }
                            Action::ReplaceSelectedText => {
                                if let Some(ActionData::Value(s)) = req.data.clone() {
                                    // TextEditor: operate on buffer
                                    if utils::is_ptr_of::<text::TextEditor>(w.as_widget_ptr()) {
                                        let mut e =
                                            text::TextEditor::from_widget_ptr(w.as_widget_ptr() as _);
                                        if let Some(mut buf) = e.buffer() {
                                            if let Some((start, end)) = buf.selection_position() {
                                                if start != end {
                                                    buf.replace(start, end, &s);
                                                    e.set_insert_position(start + s.len() as i32);
                                                } else {
                                                    let pos = e.insert_position();
                                                    buf.insert(pos, &s);
                                                    e.set_insert_position(pos + s.len() as i32);
                                                }
                                            } else {
                                                let pos = e.insert_position();
                                                buf.insert(pos, &s);
                                                e.set_insert_position(pos + s.len() as i32);
                                            }
                                        }
                                    // Input family
                                    } else if utils::is_ptr_of::<input::Input>(w.as_widget_ptr())
                                        || utils::is_ptr_of::<input::IntInput>(w.as_widget_ptr())
                                        || utils::is_ptr_of::<input::FloatInput>(w.as_widget_ptr())
                                        || utils::is_ptr_of::<input::MultilineInput>(w.as_widget_ptr())
                                    {
                                        let mut i = input::Input::from_widget_ptr(w.as_widget_ptr() as _);
                                        let start = i.position();
                                        let end = i.mark();
                                        if start != end {
                                            let _ = i.replace(start, end, &s);
                                            let _ = i.set_position(start + s.len() as i32);
                                            let _ = i.set_mark(start + s.len() as i32);
                                        } else {
                                            let _ = i.insert(&s);
                                            let new_pos = start + s.len() as i32;
                                            let _ = i.set_position(new_pos);
                                            let _ = i.set_mark(new_pos);
                                        }
                                    }
                                }
                            }
                            Action::ScrollIntoView => {
                                // Best effort: for TextEditor, ensure caret is visible
                                if utils::is_ptr_of::<text::TextEditor>(w.as_widget_ptr()) {
                                    let mut e = text::TextEditor::from_widget_ptr(w.as_widget_ptr() as _);
                                    e.show_insert_position();
                                }
                            }
                            Action::ScrollToPoint => {
                                // No robust XY->position mapping for editors; best effort noop.
                                // Could be extended for specific widgets/containers.
                            }
                            Action::SetTextSelection => {
                                if let Some(ActionData::SetTextSelection(sel)) = req.data.clone() {
                                    // Only apply when selection nodes target this widget
                                    if sel.anchor.node == req.target && sel.focus.node == req.target {
                                        // TextEditor path
                                        if utils::is_ptr_of::<text::TextEditor>(w.as_widget_ptr()) {
                                            let mut e =
                                                text::TextEditor::from_widget_ptr(w.as_widget_ptr() as _);
                                            let mut buf = if let Some(b) = e.buffer() {
                                                b
                                            } else {
                                                let b = text::TextBuffer::default();
                                                e.set_buffer(Some(b));
                                                e.buffer().unwrap()
                                            };
                                            let len = buf.length();
                                            let mut a = sel.anchor.character_index as i32;
                                            let mut f = sel.focus.character_index as i32;
                                            a = a.clamp(0, len);
                                            f = f.clamp(0, len);
                                            if a == f {
                                                // Caret move
                                                buf.unselect();
                                                e.set_insert_position(a);
                                            } else {
                                                let (start, end) = if a <= f { (a, f) } else { (f, a) };
                                                buf.select(start, end);
                                                e.set_insert_position(end);
                                            }
                                        // Input family path
                                        } else if utils::is_ptr_of::<input::Input>(w.as_widget_ptr())
                                            || utils::is_ptr_of::<input::IntInput>(w.as_widget_ptr())
                                            || utils::is_ptr_of::<input::FloatInput>(w.as_widget_ptr())
                                            || utils::is_ptr_of::<input::MultilineInput>(w.as_widget_ptr())
                                        {
                                            let mut i = input::Input::from_widget_ptr(w.as_widget_ptr() as _);
                                            let len = i.value().len() as i32;
                                            let mut a = sel.anchor.character_index as i32;
                                            let mut f = sel.focus.character_index as i32;
                                            a = a.clamp(0, len);
                                            f = f.clamp(0, len);
                                            let (start, end) = if a <= f { (a, f) } else { (f, a) };
                                            // Set selection; on collapse, mark==position
                                            let _ = i.set_position(start);
                                            let _ = i.set_mark(end);
                                        }
                                    }
                                }
                            }
                            Action::SetValue => {
                                if let Some(data) = req.data {
                                    match data {
                                        ActionData::Value(s) => {
                                            // Text-capable inputs
                                            if utils::is_ptr_of::<input::IntInput>(w.as_widget_ptr()) {
                                                let mut i = input::IntInput::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&s);
                                            } else if utils::is_ptr_of::<input::FloatInput>(w.as_widget_ptr()) {
                                                let mut i = input::FloatInput::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&s);
                                            } else if utils::is_ptr_of::<input::MultilineInput>(w.as_widget_ptr()) {
                                                let mut i =
                                                    input::MultilineInput::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&s);
                                            } else if utils::is_ptr_of::<input::Input>(w.as_widget_ptr()) {
                                                let mut i = input::Input::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&s);
                                            } else if utils::is_ptr_of::<text::TextEditor>(w.as_widget_ptr()) {
                                                let mut e = text::TextEditor::from_widget_ptr(w.as_widget_ptr() as _);
                                                if let Some(mut buf) = e.buffer() {
                                                    buf.set_text(&s);
                                                } else {
                                                    let mut buf = text::TextBuffer::default();
                                                    buf.set_text(&s);
                                                    e.set_buffer(Some(buf));
                                                }
                                            // Toggle/Check buttons (boolean from string)
                                            } else if utils::is_ptr_of::<button::CheckButton>(w.as_widget_ptr()) {
                                                let mut b = button::CheckButton::from_widget_ptr(
                                                    w.as_widget_ptr() as _,
                                                );
                                                let on = matches!(
                                                    s.to_ascii_lowercase().as_str(),
                                                    "1" | "true" | "on" | "yes"
                                                );
                                                b.set_value(on);
                                            } else if utils::is_ptr_of::<button::ToggleButton>(w.as_widget_ptr()) {
                                                let mut b = button::ToggleButton::from_widget_ptr(
                                                    w.as_widget_ptr() as _,
                                                );
                                                let on = matches!(
                                                    s.to_ascii_lowercase().as_str(),
                                                    "1" | "true" | "on" | "yes"
                                                );
                                                b.set_value(on);
                                            // Valuators (parse string -> f64)
                                            } else if let Ok(n) = s.parse::<f64>() {
                                                macro_rules! set_val {
                                                    ($t:ty) => {{
                                                        if utils::is_ptr_of::<$t>(w.as_widget_ptr()) {
                                                            let mut v = <$t>::from_widget_ptr(w.as_widget_ptr() as _);
                                                            v.set_value(n);
                                                            true
                                                        } else {
                                                            false
                                                        }
                                                    }};
                                                }
                                                let _handled =
                                                    set_val!(valuator::Slider)
                                                        || set_val!(valuator::NiceSlider)
                                                        || set_val!(valuator::Dial)
                                                        || set_val!(valuator::LineDial)
                                                        || set_val!(valuator::Counter)
                                                        || set_val!(valuator::Scrollbar)
                                                        || set_val!(valuator::ValueInput)
                                                        || set_val!(valuator::ValueOutput)
                                                        || set_val!(valuator::ValueSlider)
                                                        || set_val!(valuator::HorValueSlider)
                                                        || set_val!(valuator::HorSlider)
                                                        || set_val!(valuator::HorNiceSlider)
                                                        || set_val!(valuator::FillSlider)
                                                        || set_val!(valuator::HorFillSlider)
                                                        || set_val!(misc::Spinner)
                                                        || set_val!(misc::Progress);
                                                // else: fallback noop
                                            }
                                        }
                                        ActionData::NumericValue(n) => {
                                            // Inputs (apply rounding for IntInput)
                                            if utils::is_ptr_of::<input::IntInput>(w.as_widget_ptr()) {
                                                let mut i = input::IntInput::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&format!("{}", n.round() as i64));
                                            } else if utils::is_ptr_of::<input::FloatInput>(w.as_widget_ptr()) {
                                                let mut i = input::FloatInput::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&format!("{}", n));
                                            } else if utils::is_ptr_of::<input::MultilineInput>(w.as_widget_ptr()) {
                                                let mut i =
                                                    input::MultilineInput::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&format!("{}", n));
                                            } else if utils::is_ptr_of::<input::Input>(w.as_widget_ptr()) {
                                                let mut i = input::Input::from_widget_ptr(w.as_widget_ptr() as _);
                                                i.set_value(&format!("{}", n));
                                            } else if utils::is_ptr_of::<text::TextEditor>(w.as_widget_ptr()) {
                                                let mut e = text::TextEditor::from_widget_ptr(w.as_widget_ptr() as _);
                                                let s = format!("{}", n);
                                                if let Some(mut buf) = e.buffer() {
                                                    buf.set_text(&s);
                                                } else {
                                                    let mut buf = text::TextBuffer::default();
                                                    buf.set_text(&s);
                                                    e.set_buffer(Some(buf));
                                                }
                                            // Toggle/Check buttons (numeric → bool)
                                            } else if utils::is_ptr_of::<button::CheckButton>(w.as_widget_ptr()) {
                                                let mut b = button::CheckButton::from_widget_ptr(
                                                    w.as_widget_ptr() as _,
                                                );
                                                b.set_value(n != 0.0);
                                            } else if utils::is_ptr_of::<button::ToggleButton>(w.as_widget_ptr()) {
                                                let mut b = button::ToggleButton::from_widget_ptr(
                                                    w.as_widget_ptr() as _,
                                                );
                                                b.set_value(n != 0.0);
                                            // Valuators
                                            } else {
                                                macro_rules! set_val {
                                                    ($t:ty) => {{
                                                        if utils::is_ptr_of::<$t>(w.as_widget_ptr()) {
                                                            let mut v = <$t>::from_widget_ptr(w.as_widget_ptr() as _);
                                                            v.set_value(n);
                                                            true
                                                        } else {
                                                            false
                                                        }
                                                    }};
                                                }
                                                let _handled =
                                                    set_val!(valuator::Slider)
                                                        || set_val!(valuator::NiceSlider)
                                                        || set_val!(valuator::Dial)
                                                        || set_val!(valuator::LineDial)
                                                        || set_val!(valuator::Counter)
                                                        || set_val!(valuator::Scrollbar)
                                                        || set_val!(valuator::ValueInput)
                                                        || set_val!(valuator::ValueOutput)
                                                        || set_val!(valuator::ValueSlider)
                                                        || set_val!(valuator::HorValueSlider)
                                                        || set_val!(valuator::HorSlider)
                                                        || set_val!(valuator::HorNiceSlider)
                                                        || set_val!(valuator::FillSlider)
                                                        || set_val!(valuator::HorFillSlider)
                                                        || set_val!(misc::Spinner)
                                                        || set_val!(misc::Progress);
                                                // else: fallback noop
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
        this
    }

    pub fn with_action_handler(
        window: &window::Window,
        source: impl 'static + ActivationHandler + Send,
        action_handler: impl 'static + ActionHandler + Send,
    ) -> Self {
        let deactivation_handler = FltkDeactivationHandler {};
        let adapter =
            platform_adapter::Adapter::new(window, source, action_handler, deactivation_handler);
        window.clone().resize_callback({
            let adapter = adapter.clone();
            move |win, _x, _y, w, h| {
                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                {
                    let outer_origin = Point {
                        x: win.x_root() as _,
                        y: win.y_root() as _,
                    };
                    // Client-area origin (top-left) in root/screen coordinates
                    let inner_origin = Point {
                        x: win.x() as _,
                        y: win.y() as _,
                    };
                    let outer_size = Size {
                        width: win.decorated_w() as _,
                        height: win.decorated_h() as _,
                    };
                    let inner_size = Size {
                        width: w as _,
                        height: h as _,
                    };
                    adapter.borrow_mut().set_root_window_bounds(
                        Rect::from_origin_size(outer_origin, outer_size),
                        Rect::from_origin_size(inner_origin, inner_size),
                    );
                }
            }
        });
        Self { adapter }
    }

    #[cfg(all(
        not(target_os = "linux"),
        not(target_os = "dragonfly"),
        not(target_os = "freebsd"),
        not(target_os = "netbsd"),
        not(target_os = "openbsd")
    ))]
    #[must_use]
    pub fn on_event(&self, window: &window::Window, event: &Event) -> bool {
        unsafe { app::handle_raw(*event, window.as_widget_ptr() as _) }
    }
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    #[must_use]
    pub fn on_event(&self, window: &mut window::Window, event: &Event) -> bool {
        unsafe { app::handle_raw(*event, window.as_widget_ptr() as _) }
    }

    // pub fn update_window_focus_state(&mut self, is_focused: bool) {
    //     self.adapter.borrow_mut().update_window_focus_state(is_focused)
    // }

    pub fn update_if_active(&mut self, updater: impl FnOnce() -> TreeUpdate) {
        self.adapter.borrow_mut().update_if_active(updater)
    }
}
