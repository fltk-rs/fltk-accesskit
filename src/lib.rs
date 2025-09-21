#![doc = include_str!("../README.md")]

use accesskit::{NodeId, TreeUpdate};
use fltk::{enums::*, prelude::*, widget, *};
use std::collections::HashSet;
type ExcludePred = Box<dyn Fn(&widget::Widget) -> bool>;

pub mod accessible;
mod fltk_adapter;
mod platform_adapter;

pub use accessible::Accessible;
pub use fltk_adapter::Adapter;

#[derive(Default)]
pub struct Excludes {
    ptrs: HashSet<u64>,
    subtree_ptrs: HashSet<u64>,
    preds: Vec<ExcludePred>,
}

impl Excludes {
    fn matches(&self, w: &widget::Widget) -> bool {
        let p = w.as_widget_ptr() as usize as u64;
        if self.ptrs.contains(&p) {
            return true;
        }
        for f in &self.preds {
            if f(w) {
                return true;
            }
        }
        false
    }
    fn skip_subtree(&self, w: &widget::Widget) -> bool {
        let p = w.as_widget_ptr() as usize as u64;
        self.subtree_ptrs.contains(&p)
    }
}

pub struct AccessibilityBuilder {
    root: window::Window,
    excludes: Excludes,
}

impl AccessibilityBuilder {
    pub fn new(root: window::Window) -> Self {
        Self { root, excludes: Excludes::default() }
    }
    pub fn exclude_widget<W: WidgetExt>(mut self, w: &W) -> Self {
        self.excludes.ptrs.insert(w.as_widget_ptr() as usize as u64);
        self
    }
    pub fn exclude_subtree<W: WidgetExt>(mut self, w: &W) -> Self {
        self.excludes
            .subtree_ptrs
            .insert(w.as_widget_ptr() as usize as u64);
        self
    }
    pub fn exclude_type<T: WidgetBase>(mut self) -> Self {
        self.excludes
            .preds
            .push(Box::new(|w: &widget::Widget| fltk::utils::is_ptr_of::<T>(w.as_widget_ptr())));
        self
    }
    pub fn exclude_if(mut self, pred: impl Fn(&widget::Widget) -> bool + 'static) -> Self {
        self.excludes.preds.push(Box::new(pred));
        self
    }
    pub fn attach(self) -> AccessibilityContext {
        let mut wids = collect_nodes(&self.root, &self.excludes);
        let (win_id, win_node) = self
            .root
            .make_node(&wids.iter().map(|x| x.0).collect::<Vec<_>>());
        wids.push((win_id, win_node));
        let activation_handler = crate::fltk_adapter::FltkActivationHandler { wids, win_id };
        let adapter = Adapter::new(&self.root, activation_handler);
        AccessibilityContext {
            adapter,
            root: self.root,
            excludes: self.excludes,
        }
    }
}

pub fn builder(root: window::Window) -> AccessibilityBuilder {
    AccessibilityBuilder::new(root)
}

pub struct AccessibilityContext {
    adapter: Adapter,
    root: window::Window,
    excludes: Excludes,
}

impl AccessibilityContext {
    fn collect(&self) -> Vec<(NodeId, accesskit::Node)> {
        let mut wids = collect_nodes(&self.root, &self.excludes);
        let (win_id, win_node) = self
            .root
            .make_node(&wids.iter().map(|x| x.0).collect::<Vec<_>>());
        wids.push((win_id, win_node));
        wids
    }
}

pub trait AccessibleApp {
    fn run_with_accessibility(&self, ac: AccessibilityContext) -> Result<(), FltkError>;
}

impl AccessibleApp for app::App {
    fn run_with_accessibility(&self, ac: AccessibilityContext) -> Result<(), FltkError> {
        // Move context into the handler, using a cloned root to register the closure.
        let ctx = ac;
        let mut root = ctx.root.clone();
        let mut adapter = ctx.adapter.clone();
        root.handle({
            move |_, ev| {
                match ev {
                    Event::KeyUp => {
                        let wids = ctx.collect();
                        if let Some(focused) = fltk::app::focus() {
                            let node_id = NodeId(focused.as_widget_ptr() as _);
                            adapter.update_if_active(|| TreeUpdate {
                                nodes: wids,
                                tree: None,
                                focus: node_id,
                            });
                        }
                        false
                    }
                    _ => false,
                }
            }
        });
        self.run()
    }
}

fn collect_nodes(
    root: &window::Window,
    excludes: &Excludes,
) -> Vec<(NodeId, accesskit::Node)> {
    let mut out = Vec::new();
    // Traverse children of root
    let root_w = root.as_base_widget();
    if let Some(grp) = root_w.as_group() {
        walk_group(&grp, excludes, &mut out);
    }
    out
}

fn walk_group(grp: &group::Group, excludes: &Excludes, out: &mut Vec<(NodeId, accesskit::Node)>) {
    for i in 0..grp.children() {
        if let Some(child) = grp.child(i) {
            if excludes.skip_subtree(&child) {
                continue;
            }
            // If the child is excluded and it's a group, skip its entire subtree.
            // If it's excluded and not a group, just skip the node.
            let subgrp = child.as_group();
            if excludes.matches(&child) {
                // For groups this prevents iterating children.
                continue;
            }
            // Add node if supported
            if let Some(n) = crate::accessible::node_for_widget(&child, &[]) {
                out.push(n);
            }
            // Recurse into groups that weren't excluded
            if let Some(subgrp) = subgrp {
                walk_group(&subgrp, excludes, out);
            }
        }
    }
}
