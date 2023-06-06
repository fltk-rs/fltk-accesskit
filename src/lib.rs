use accesskit::{NodeClassSet, NodeId, Tree, TreeUpdate};
use fltk::{enums::*, prelude::*, *};
use std::num::NonZeroU128;

pub mod accessible;
mod fltk_adapter;
mod platform_adapter;

pub use accessible::Accessible;
pub use fltk_adapter::{ActionRequestEvent, Adapter};

pub struct AccessibilityContext {
    adapter: Adapter,
    root: window::Window,
    widgets: Vec<Box<dyn Accessible>>,
    nc: NodeClassSet,
}

impl AccessibilityContext {
    pub fn new(root: window::Window, mut widgets: Vec<Box<dyn Accessible>>) -> Self {
        let mut nc = NodeClassSet::new();
        let mut wids = vec![];
        for w in &widgets {
            let n = w.make_node(&mut nc, &[]);
            wids.push(n);
        }
        let (win_id, win_node) =
            root.make_node(&mut nc, &wids.iter().map(|x| x.0).collect::<Vec<_>>());
        wids.push((win_id, win_node));
        let adapter = {
            Adapter::new(&root, move || TreeUpdate {
                nodes: wids,
                tree: Some(Tree::new(win_id)),
                focus: if let Some(focused) = app::focus() {
                    let focused = focused.as_widget_ptr() as usize as u128;
                    Some(NodeId(unsafe { NonZeroU128::new_unchecked(focused) }))
                } else {
                    None
                },
            })
        };
        widgets.push(Box::new(root.clone()));
        Self {
            adapter,
            root,
            widgets,
            nc,
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
                let adapter = adapter.clone();
                match ev {
                    Event::KeyUp => {
                        // if app::event_key() == Key::Tab {
                        let mut wids = vec![];
                        for w in &ac.widgets {
                            let n = w.make_node(&mut ac.nc, &[]);
                            wids.push(n);
                        }
                        let (win_id, win_node) =
                            w.make_node(&mut ac.nc, &wids.iter().map(|x| x.0).collect::<Vec<_>>());
                        wids.push((win_id, win_node));
                        if let Some(focused) = app::focus() {
                            let focused = focused.as_widget_ptr() as usize as u128;
                            let node_id = NodeId(unsafe { NonZeroU128::new_unchecked(focused) });
                            adapter.update_if_active(|| TreeUpdate {
                                nodes: wids,
                                tree: None,
                                focus: Some(node_id),
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
