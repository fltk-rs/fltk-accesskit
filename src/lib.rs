#![doc = include_str!("../README.md")]

use accesskit::{NodeId, TreeUpdate};
use fltk::{enums::*, prelude::*, *};

pub mod accessible;
mod fltk_adapter;
mod platform_adapter;

pub use accessible::Accessible;
pub use fltk_adapter::{ActionRequestEvent, Adapter};

pub struct AccessibilityContext {
    adapter: Adapter,
    root: window::Window,
    widgets: Vec<Box<dyn Accessible>>,
}

impl AccessibilityContext {
    pub fn new(root: window::Window, mut widgets: Vec<Box<dyn Accessible>>) -> Self {
        let mut wids = vec![];
        for w in &widgets {
            let n = w.make_node(&[]);
            wids.push(n);
        }
        let (win_id, win_node) =
            root.make_node(&wids.iter().map(|x| x.0).collect::<Vec<_>>());
        wids.push((win_id, win_node));
        let activation_handler = crate::fltk_adapter::FltkActivationHandler {
            wids, win_id
        };
        let adapter = {
            Adapter::new(&root, activation_handler)
        };
        widgets.push(Box::new(root.clone()));
        Self {
            adapter,
            root,
            widgets,
        }
    }
}

pub trait AccessibleApp {
    fn run_with_accessibility(&self, ac: AccessibilityContext) -> Result<(), FltkError>;
}

impl AccessibleApp for app::App {
    fn run_with_accessibility(&self, mut ac: AccessibilityContext) -> Result<(), FltkError> {
        ac.root.handle({
            let adapter = ac.adapter.clone();
            move |w, ev| {
                let mut adapter = adapter.clone();
                match ev {
                    Event::KeyUp => {
                        // if app::event_key() == Key::Tab {
                        let mut wids = vec![];
                        for w in &ac.widgets {
                            let n = w.make_node(&[]);
                            wids.push(n);
                        }
                        let (win_id, win_node) =
                            w.make_node(&wids.iter().map(|x| x.0).collect::<Vec<_>>());
                        wids.push((win_id, win_node));
                        if let Some(focused) = app::focus() {
                            let focused = focused.as_widget_ptr() as usize as u64;
                            let node_id = NodeId(focused);
                            adapter.update_if_active(|| TreeUpdate {
                                nodes: wids,
                                tree: None,
                                focus: node_id,
                            });
                        }
                        // }
                        false
                    }
                    _ => false,
                }
            }
        });
        self.run()
    }
}
