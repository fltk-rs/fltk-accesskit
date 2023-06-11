use accesskit::{
    Action, Affine, CheckedState, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId, Rect,
    Role, TextPosition, TextSelection,
};
use fltk::{enums::*, prelude::*, *};
use std::num::NonZeroU128;

pub trait Accessible {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node);
}

fn node_widget_common(builder: &mut NodeBuilder, wid: &impl WidgetExt, children: &[NodeId]) -> NodeId {
    let node_id =
        NodeId(unsafe { NonZeroU128::new_unchecked(wid.as_widget_ptr() as usize as u128) });
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
    builder.set_name(&*wid.label());
    if wid.trigger().contains(CallbackTrigger::Release) {
        builder.set_default_action_verb(DefaultActionVerb::Click);
    }
    if wid.takes_events() && wid.has_visible_focus() {
        builder.add_action(Action::Focus);
    }
    for c in children {
        builder.push_child(*c);
    }
    node_id
}

impl Accessible for button::Button {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Button);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for button::RadioButton {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::RadioButton);
        builder.set_checked_state(if self.value() {
            CheckedState::True
        } else {
            CheckedState::False
        });
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for button::RadioRoundButton {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::RadioButton);
        builder.set_checked_state(if self.value() {
            CheckedState::True
        } else {
            CheckedState::False
        });
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for button::CheckButton {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::CheckBox);
        builder.set_checked_state(if self.value() {
            CheckedState::True
        } else {
            CheckedState::False
        });
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for button::ToggleButton {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::ToggleButton);
        builder.set_checked_state(if self.value() {
            CheckedState::True
        } else {
            CheckedState::False
        });
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for window::Window {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Window);
        builder.set_transform(Affine::scale(app::screen_scale(0) as f64));
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for frame::Frame {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::default();
        let id = node_widget_common(&mut builder, self, children);
        if self.image().is_some() {
            builder.set_role(Role::Image);
        } else {
            builder.set_role(Role::StaticText);
        }
        (id, builder.build(nc))
    }
}

impl Accessible for output::Output {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::StaticText);
        builder.set_value(&*self.value());
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for input::Input {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::TextField);
        builder.set_value(&*self.value());
        builder.set_default_action_verb(DefaultActionVerb::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for input::IntInput {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::TextField);
        builder.set_value(&*self.value());
        builder.set_default_action_verb(DefaultActionVerb::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for input::FloatInput {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::TextField);
        builder.set_value(&*self.value());
        builder.set_default_action_verb(DefaultActionVerb::Focus);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for input::MultilineInput {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::TextField);
        builder.set_value(&*self.value());
        builder.set_default_action_verb(DefaultActionVerb::Focus);
        builder.set_multiline();
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for output::MultilineOutput {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::StaticText);
        builder.set_value(&*self.value());
        builder.set_multiline();
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for text::TextDisplay {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::StaticText);
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
        (id, builder.build(nc))
    }
}

impl Accessible for text::TextEditor {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::TextField);
        builder.set_default_action_verb(DefaultActionVerb::Focus);
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
        (id, builder.build(nc))
    }
}

impl Accessible for text::SimpleTerminal {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Term);
        builder.set_default_action_verb(DefaultActionVerb::Focus);
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
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::Slider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::NiceSlider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::ValueSlider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::FillSlider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::HorSlider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::HorFillSlider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::HorNiceSlider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::HorValueSlider {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::Dial {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::FillDial {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::LineDial {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::Counter {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::Roller {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::ValueInput {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::ValueOutput {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Slider);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        builder.add_action(Action::SetValue);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for valuator::Scrollbar {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::ScrollBar);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        builder.set_numeric_value_step(self.step());
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for menu::MenuBar {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::MenuBar);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for menu::Choice {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::PopupButton);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for table::Table {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Table);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for tree::Tree {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Tree);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for group::Scroll {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::ScrollView);
        builder.add_action(Action::ScrollDown);
        builder.add_action(Action::ScrollUp);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for group::Flex {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Group);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for group::Group {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::Group);
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}

impl Accessible for misc::Progress {
    fn make_node(&self, nc: &mut NodeClassSet, children: &[NodeId]) -> (NodeId, Node) {
        let mut builder = NodeBuilder::new(Role::ProgressIndicator);
        builder.set_numeric_value(self.value());
        builder.set_min_numeric_value(self.minimum());
        builder.set_max_numeric_value(self.maximum());
        let id = node_widget_common(&mut builder, self, children);
        (id, builder.build(nc))
    }
}
