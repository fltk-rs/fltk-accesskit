use accesskit::{Action, Affine, Node, NodeId, Rect, Role, TextPosition, TextSelection, Toggled};
use fltk::{
    button, enums::*, frame, input, output, prelude::*, text, utils, widget, window, *,
};

pub trait Accessible {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node);
}

fn node_widget_common(builder: &mut Node, wid: &impl WidgetExt, children: &[NodeId]) -> NodeId {
    let node_id = NodeId(wid.as_widget_ptr() as usize as u64);
    if wid.parent().is_some() && wid.as_window().is_none() {
        builder.set_bounds(Rect {
            x0: wid.x() as f64,
            y0: wid.y() as f64,
            x1: (wid.w() + wid.x()) as f64,
            y1: (wid.h() + wid.y()) as f64,
        });
    } else {
        builder.set_bounds(Rect {
            x0: 0.0,
            y0: 0.0,
            x1: wid.w() as f64,
            y1: wid.h() as f64,
        });
    }
    builder.set_label(&*wid.label());
    if wid.trigger().contains(CallbackTrigger::Release) {
        builder.add_action(Action::Click);
    }
    for c in children {
        builder.push_child(*c);
    }
    node_id
}

/// Try to build an accessibility node for a given widget pointer.
/// Returns None for unsupported widget types. Window nodes are handled separately.
pub fn node_for_widget(w: &widget::Widget, children: &[NodeId]) -> Option<(NodeId, Node)> {
    let ptr = w.as_widget_ptr();
    macro_rules! try_type {
        ($t:ty) => {
            if utils::is_ptr_of::<$t>(ptr) {
                let typed = unsafe { <$t>::from_widget_ptr(ptr as _) };
                let (id, node) = typed.make_node(children);
                return Some((id, node));
            }
        };
    }

    // Buttons (more specific first)
    try_type!(button::RadioButton);
    try_type!(button::RadioRoundButton);
    try_type!(button::CheckButton);
    try_type!(button::ToggleButton);
    try_type!(button::Button);

    // Inputs/Outputs/Text
    try_type!(input::IntInput);
    try_type!(input::FloatInput);
    try_type!(input::MultilineInput);
    try_type!(input::Input);
    try_type!(text::TextEditor);
    try_type!(output::MultilineOutput);
    try_type!(output::Output);
    try_type!(text::TextDisplay);

    // Frames (image/label)
    try_type!(frame::Frame);

    // Windows (non-root windows will be discovered)
    try_type!(window::Window);

    None
}

impl Accessible for button::Button {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Button);
        builder.add_action(Action::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for button::RadioButton {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::RadioButton);
        builder.set_toggled(if self.value() {
            Toggled::True
        } else {
            Toggled::False
        });
        builder.add_action(Action::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for button::RadioRoundButton {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::RadioButton);
        builder.set_toggled(if self.value() {
            Toggled::True
        } else {
            Toggled::False
        });
        builder.add_action(Action::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for button::CheckButton {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::CheckBox);
        builder.set_toggled(if self.value() {
            Toggled::True
        } else {
            Toggled::False
        });
        builder.add_action(Action::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for button::ToggleButton {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Button);
        builder.set_toggled(if self.value() {
            Toggled::True
        } else {
            Toggled::False
        });
        builder.add_action(Action::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for window::Window {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Window);
        let sn = app::screen_num(self.x(), self.y());
        builder.set_transform(Affine::scale(app::screen_scale(sn) as f64));
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for frame::Frame {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::default();
        let id = node_widget_common(&mut builder, self, children);
        if self.image().is_some() {
            builder.set_role(Role::Image);
        } else {
            builder.set_role(Role::Label);
        }
        (id, builder)
    }
}

impl Accessible for output::Output {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Label);
        builder.set_value(&*self.value());
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for input::Input {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::TextInput);
        builder.set_value(&*self.value());
        builder.add_action(Action::Focus);
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for input::IntInput {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::TextInput);
        builder.set_value(&*self.value());
        builder.add_action(Action::Focus);
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for input::FloatInput {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::TextInput);
        builder.set_value(&*self.value());
        builder.add_action(Action::Focus);
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for input::MultilineInput {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::MultilineTextInput);
        builder.set_value(&*self.value());
        builder.add_action(Action::Focus);
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for output::MultilineOutput {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Paragraph);
        builder.set_value(&*self.value());
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for text::TextDisplay {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Paragraph);
        let id = node_widget_common(&mut builder, self, children);
        if let Some(buf) = self.buffer() {
            builder.set_value(&*buf.text());
            if let Some((s, e)) = buf.selection_position() {
                builder.set_text_selection(TextSelection {
                    anchor: TextPosition {
                        node: id,
                        character_index: s as _,
                    },
                    focus: TextPosition {
                        node: id,
                        character_index: e as _,
                    },
                });
            }
        }
        (id, builder)
    }
}

impl Accessible for text::TextEditor {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::MultilineTextInput);

        // Value and selection
        if let Some(buf) = self.buffer() {
            builder.set_value(&*buf.text());
        }
        builder.add_action(Action::Focus);
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        if let Some(buf) = self.buffer() {
            builder.set_value(&*buf.text());
            if let Some((s, e)) = buf.selection_position() {
                builder.set_text_selection(TextSelection {
                    anchor: TextPosition {
                        node: id,
                        character_index: s as _,
                    },
                    focus: TextPosition {
                        node: id,
                        character_index: e as _,
                    },
                });
            }
        }
        (id, builder)
    }
}

#[allow(deprecated)]
impl Accessible for text::SimpleTerminal {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Terminal);
        builder.add_action(Action::Focus);
        let id = node_widget_common(&mut builder, self, children);
        if let Some(buf) = self.buffer() {
            builder.set_value(&*buf.text());
            if let Some((s, e)) = buf.selection_position() {
                builder.set_text_selection(TextSelection {
                    anchor: TextPosition {
                        node: id,
                        character_index: s as _,
                    },
                    focus: TextPosition {
                        node: id,
                        character_index: e as _,
                    },
                });
            }
        }
        (id, builder)
    }
}

impl Accessible for valuator::Slider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::NiceSlider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::ValueSlider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::FillSlider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::HorSlider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::HorFillSlider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::HorNiceSlider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::HorValueSlider {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::Dial {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::FillDial {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::LineDial {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::Counter {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::Roller {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::ValueInput {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::ValueOutput {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for valuator::Scrollbar {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::ScrollBar);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for menu::MenuBar {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::MenuBar);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for menu::Choice {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::MenuListPopup);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for table::Table {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Table);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for tree::Tree {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Tree);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for group::Scroll {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::ScrollView);
        builder.add_action(Action::ScrollDown);
        builder.add_action(Action::ScrollUp);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for group::Flex {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Group);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for group::Group {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::Group);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}

impl Accessible for misc::Progress {
    fn make_node(&self, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = Node::new(Role::ProgressIndicator);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        let id = node_widget_common(&mut builder, self, children);
        (id, builder)
    }
}
