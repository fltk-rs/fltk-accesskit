#![allow(unused_imports)]
use accesskit::{ActionHandler, ActivationHandler, DeactivationHandler, Rect, TreeUpdate};
use std::cell::RefCell;
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
    adapter: Option<SubclassingAdapter>,
}

impl Adapter {
    pub fn new(
        _win: &window::Window,
        activation_handler: impl 'static + ActivationHandler + Send,
        action_handler: impl 'static + ActionHandler + Send,
        _deactivation_handler: impl 'static + DeactivationHandler + Send,
    ) -> Rc<RefCell<Self>> {
        let adapter;
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            adapter = UnixAdapter::new(activation_handler, action_handler, _deactivation_handler);
        }

        #[cfg(target_os = "windows")]
        {
            unsafe extern "system" {
                fn ShowWindow(w: *mut std::os::raw::c_void, v: i32) -> i32;
            }
            unsafe {
                ShowWindow(_win.raw_handle(), 0);
            }
            adapter = SubclassingAdapter::new(
                HWND(_win.raw_handle()),
                activation_handler,
                action_handler,
            );
            unsafe {
                ShowWindow(_win.raw_handle(), 1);
            }
        }

        #[cfg(target_os = "macos")]
        {
            adapter = {
                use std::os::raw;
                extern "C" {
                    pub fn cfltk_getContentView(xid: *mut raw::c_void) -> *mut raw::c_void;
                }
                let cv = unsafe { cfltk_getContentView(_win.raw_handle()) };
                unsafe { SubclassingAdapter::new(cv, activation_handler, action_handler) }
            };
        }
        Rc::new(RefCell::new(Self {
            adapter: Some(adapter),
        }))
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    pub fn set_root_window_bounds(&mut self, outer: Rect, inner: Rect) {
        if let Some(adapter) = &mut self.adapter {
            adapter.set_root_window_bounds(outer, inner);
        }
    }

    pub fn update_if_active(&mut self, updater: impl FnOnce() -> TreeUpdate) {
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            if let Some(adapter) = &mut self.adapter {
                adapter.update_if_active(updater);
            }
        }
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            if let Some(adapter) = &mut self.adapter {
                if let Some(events) = adapter.update_if_active(updater) {
                    events.raise();
                }
            }
        }
    }
}
