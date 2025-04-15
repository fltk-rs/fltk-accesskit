#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::missing_transmute_annotations)]
use accesskit::{
    ActionHandler, ActionRequest, ActivationHandler, DeactivationHandler, Node, NodeId, Point,
    Rect, Size, Tree, TreeUpdate,
};
use fltk::{enums::*, prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

use crate::platform_adapter;

#[derive(Debug)]
pub struct ActionRequestEvent {
    pub window_id: window::Window,
    pub request: ActionRequest,
}

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
    window_id: window::Window,
}

impl ActionHandler for FltkActionHandler {
    fn do_action(&mut self, request: ActionRequest) {
        unsafe {
            app::handle_raw(
                std::mem::transmute(request.action as i32 + 100),
                self.window_id.as_widget_ptr() as _,
            );
        }
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
        let action_handler = FltkActionHandler {
            window_id: window.clone(),
        };
        Self::with_action_handler(window, source, action_handler)
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
            move |_, x, y, w, h| {
                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                adapter.borrow_mut().set_root_window_bounds(
                    Rect::from_origin_size(
                        Point {
                            x: x as _,
                            y: y as _,
                        },
                        Size {
                            width: w as _,
                            height: h as _,
                        },
                    ),
                    Rect::from_origin_size(
                        Point {
                            x: x as _,
                            y: y as _,
                        },
                        Size {
                            width: w as _,
                            height: h as _,
                        },
                    ),
                );
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
