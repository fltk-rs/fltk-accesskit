#![allow(unused_imports)]
use accesskit::{ActionHandler, Rect, TreeUpdate};
use std::rc::Rc;

#[cfg(target_os = "windows")]
use accesskit_windows::{SubclassingAdapter, HWND};

#[cfg(target_os = "macos")]
use accesskit_macos::SubclassingAdapter;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use accesskit_unix::Adapter as UnixAdapter;

use fltk::prelude::WindowExt;
use fltk::window;

pub struct Adapter {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    adapter: Option<UnixAdapter>,
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    adapter: SubclassingAdapter,
}

impl Adapter {
    pub fn new(
        win: &window::Window,
        source: impl 'static + FnOnce() -> TreeUpdate,
        action_handler: Box<dyn ActionHandler + Send + Sync + 'static>,
    ) -> Rc<Self> {
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let adapter = UnixAdapter::new(
            String::new(),
            String::new(),
            String::new(),
            source,
            action_handler,
        );
        #[cfg(target_os = "windows")]
        let adapter =
            SubclassingAdapter::new(HWND(win.raw_handle() as isize), source, action_handler);
        #[cfg(target_os = "macos")]
        let adapter = {
            use std::os::raw;
            extern "C" {
                pub fn cfltk_getContentView(xid: *mut raw::c_void) -> *mut raw::c_void;
            }
            let cv = unsafe { cfltk_getContentView(win.raw_handle()) };
            unsafe { SubclassingAdapter::new(cv, source, action_handler) }
        };
        Rc::new(Self { adapter })
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    pub fn set_root_window_bounds(&self, outer: Rect, inner: Rect) {
        if let Some(adapter) = &self.adapter {
            adapter.set_root_window_bounds(outer, inner);
        }
    }

    pub fn update(&self, update: TreeUpdate) {
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            if let Some(adapter) = &self.adapter {
                adapter.update(update);
            }
        }
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            self.adapter.update(update).raise();
        }
    }

    pub fn update_if_active(&self, updater: impl FnOnce() -> TreeUpdate) {
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            if let Some(adapter) = &self.adapter {
                adapter.update(updater());
            }
        }
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            if let Some(events) = self.adapter.update_if_active(updater) {
                events.raise();
            }
        }
    }
}
