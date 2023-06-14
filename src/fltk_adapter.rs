#![allow(unused_imports)]
#![allow(unused_variables)]
use accesskit::{ActionHandler, ActionRequest, Point, Rect, Size, TreeUpdate};
use fltk::{enums::*, prelude::*, *};
use std::rc::Rc;

use crate::platform_adapter;

#[derive(Debug)]
pub struct ActionRequestEvent {
    pub window_id: window::Window,
    pub request: ActionRequest,
}

struct FltkActionHandler {
    window_id: window::Window,
}

impl ActionHandler for FltkActionHandler {
    fn do_action(&self, request: ActionRequest) {
        unsafe {
            app::handle_raw(
                std::mem::transmute(request.action as i32 + 100),
                self.window_id.as_widget_ptr() as _,
            );
        }
    }
}

#[derive(Clone)]
pub struct Adapter {
    adapter: Rc<platform_adapter::Adapter>,
}

impl Adapter {
    pub fn new(
        window: &window::Window,
        source: impl 'static + FnOnce() -> TreeUpdate + Send,
    ) -> Self {
        let action_handler = FltkActionHandler {
            window_id: window.clone(),
        };
        Self::with_action_handler(window, source, Box::new(action_handler))
    }

    pub fn with_action_handler(
        window: &window::Window,
        source: impl 'static + FnOnce() -> TreeUpdate + Send,
        action_handler: Box<dyn ActionHandler + Send + Sync + 'static>,
    ) -> Self {
        let adapter = platform_adapter::Adapter::new(window, source, action_handler);
        window.clone().resize_callback({
            let adapter = adapter.clone();
            move |_, x, y, w, h| {
                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                adapter.set_root_window_bounds(
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

    pub fn update(&self, update: TreeUpdate) {
        self.adapter.update(update)
    }

    pub fn update_if_active(&self, updater: impl FnOnce() -> TreeUpdate) {
        self.adapter.update_if_active(updater)
    }
}
