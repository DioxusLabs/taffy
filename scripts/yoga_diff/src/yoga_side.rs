//! Builds the case as a Yoga tree through the `yoga` crate and runs the reference layout.

use crate::case::{Case, LayoutInputs, Step, StyleMap, TextSpec, TreeNode, EDGES, V};
use crate::config::Config;
use crate::output::{BaselineCall, CallLog, MeasureCall, NodeLayout, Run};
use crate::text::{self, MeasureInput, Mode};
use std::cell::RefCell;
use yoga::{
    Align, BoxSizing, Direction, Display, Edge, FlexDirection, Gutter, Justify, MeasureMode, Node, NodeRef, Overflow,
    PositionType, StyleUnit, Undefined, Wrap,
};

thread_local! {
    static LOG: RefCell<CallLog> = RefCell::new(CallLog::default());
}

struct Ctx {
    path: String,
    text: TextSpec,
}

fn mode_from(m: MeasureMode) -> Mode {
    match m {
        MeasureMode::Undefined => Mode::Undefined,
        MeasureMode::Exactly => Mode::Exactly,
        MeasureMode::AtMost => Mode::AtMost,
    }
}

fn ctx(node: &NodeRef) -> &Ctx {
    yoga::get_node_ref_context(node).expect("measured node has no context").downcast_ref::<Ctx>().unwrap()
}

extern "C" fn measure(node: NodeRef, w: f32, wm: MeasureMode, h: f32, hm: MeasureMode) -> yoga::Size {
    let c = ctx(&node);
    let input = MeasureInput { w, wm: mode_from(wm), h, hm: mode_from(hm) };
    let (width, height) = text::measure(&c.text, input);
    LOG.with(|l| {
        l.borrow_mut().measure.push(MeasureCall {
            path: c.path.clone(),
            w: (input.wm != Mode::Undefined).then_some(w),
            wm: input.wm,
            h: (input.hm != Mode::Undefined).then_some(h),
            hm: input.hm,
            out: [width, height],
        })
    });
    yoga::Size { width, height }
}

extern "C" fn baseline(node: NodeRef, w: f32, h: f32) -> f32 {
    let c = ctx(&node);
    let out = text::baseline(&c.text, w, h);
    LOG.with(|l| l.borrow_mut().baseline.push(BaselineCall { path: c.path.clone(), w, h, out }));
    out
}

fn unit(v: V) -> StyleUnit {
    match v {
        V::Undef => StyleUnit::UndefinedValue,
        V::Px(x) => StyleUnit::Point(x.into()),
        V::Pct(x) => StyleUnit::Percent(x.into()),
        V::Auto => StyleUnit::Auto,
    }
}

fn edge(name: &str) -> Edge {
    match name {
        "left" => Edge::Left,
        "top" => Edge::Top,
        "right" => Edge::Right,
        "bottom" => Edge::Bottom,
        "start" => Edge::Start,
        "end" => Edge::End,
        "horizontal" => Edge::Horizontal,
        "vertical" => Edge::Vertical,
        _ => Edge::All,
    }
}

fn align(s: &str) -> Align {
    match s {
        "auto" => Align::Auto,
        "flex-start" => Align::FlexStart,
        "center" => Align::Center,
        "flex-end" => Align::FlexEnd,
        "stretch" => Align::Stretch,
        "baseline" => Align::Baseline,
        "space-between" => Align::SpaceBetween,
        "space-around" => Align::SpaceAround,
        "space-evenly" => Align::SpaceEvenly,
        "start" => Align::Start,
        "end" => Align::End,
        other => panic!("unknown align `{other}`"),
    }
}

fn justify(s: &str) -> Justify {
    match s {
        "auto" => Justify::Auto,
        "flex-start" => Justify::FlexStart,
        "center" => Justify::Center,
        "flex-end" => Justify::FlexEnd,
        "space-between" => Justify::SpaceBetween,
        "space-around" => Justify::SpaceAround,
        "space-evenly" => Justify::SpaceEvenly,
        "stretch" => Justify::Stretch,
        "start" => Justify::Start,
        "end" => Justify::End,
        other => panic!("unknown justify `{other}`"),
    }
}

pub fn direction(s: &str) -> Direction {
    match s {
        "rtl" => Direction::RTL,
        "ltr" => Direction::LTR,
        _ => Direction::Inherit,
    }
}

/// Applies every key of `style` with the matching `YGNodeStyleSet*` call (same key set as `yogadiff.cpp`).
fn apply_style(node: &mut Node, style: &StyleMap) {
    for (key, value) in style {
        let v = || V::parse(value);
        let s = || value.as_str().unwrap();
        let f = || value.as_f64().unwrap() as f32;
        match key.as_str() {
            "direction" => node.set_direction(direction(s())),
            "flex-direction" => node.set_flex_direction(match s() {
                "row" => FlexDirection::Row,
                "row-reverse" => FlexDirection::RowReverse,
                "column-reverse" => FlexDirection::ColumnReverse,
                _ => FlexDirection::Column,
            }),
            "justify-content" => node.set_justify_content(justify(s())),
            "align-items" => node.set_align_items(align(s())),
            "align-content" => node.set_align_content(align(s())),
            "align-self" => node.set_align_self(align(s())),
            "flex-wrap" => node.set_flex_wrap(match s() {
                "wrap" => Wrap::Wrap,
                "wrap-reverse" => Wrap::WrapReverse,
                _ => Wrap::NoWrap,
            }),
            "overflow" => node.set_overflow(match s() {
                "hidden" => Overflow::Hidden,
                "scroll" => Overflow::Scroll,
                _ => Overflow::Visible,
            }),
            "display" => node.set_display(match s() {
                "none" => Display::None,
                "contents" => Display::Contents,
                _ => Display::Flex,
            }),
            "position-type" => node.set_position_type(match s() {
                "absolute" => PositionType::Absolute,
                "static" => PositionType::Static,
                _ => PositionType::Relative,
            }),
            "box-sizing" => node.set_box_sizing(match s() {
                "content-box" => BoxSizing::ContentBox,
                _ => BoxSizing::BorderBox,
            }),
            "flex-grow" => node.set_flex_grow(f()),
            "flex-shrink" => node.set_flex_shrink(f()),
            "flex" => node.set_flex(f()),
            "aspect-ratio" => node.set_aspect_ratio(f()),
            "flex-basis" => node.set_flex_basis(unit(v())),
            "width" => node.set_width(unit(v())),
            "height" => node.set_height(unit(v())),
            "min-width" => node.set_min_width(unit(v())),
            "min-height" => node.set_min_height(unit(v())),
            "max-width" => node.set_max_width(unit(v())),
            "max-height" => node.set_max_height(unit(v())),
            "gap" => node.set_gap(Gutter::All, unit(v())),
            "column-gap" => node.set_gap(Gutter::Column, unit(v())),
            "row-gap" => node.set_gap(Gutter::Row, unit(v())),
            k => {
                let (prop, e) = k.split_once('-').unwrap_or_else(|| panic!("unknown style key `{k}`"));
                assert!(EDGES.contains(&e), "unknown style key `{k}`");
                match prop {
                    "position" => node.set_position(edge(e), unit(v())),
                    "margin" => node.set_margin(edge(e), unit(v())),
                    "padding" => node.set_padding(edge(e), unit(v())),
                    "border" => node.set_border(
                        edge(e),
                        match v() {
                            V::Px(x) => x,
                            _ => Undefined,
                        },
                    ),
                    _ => panic!("unknown style key `{k}`"),
                }
            }
        }
    }
}

fn apply_node_props(node: &mut Node, props: &StyleMap) {
    for (key, value) in props {
        match key.as_str() {
            "always-forms-containing-block" => node.set_always_forms_containing_block(value.as_bool().unwrap()),
            "node-type" => node.set_node_type(match value.as_str().unwrap() {
                "text" => yoga::NodeType::Text,
                _ => yoga::NodeType::Default,
            }),
            k => panic!("unknown node property `{k}`"),
        }
    }
}

/// Nodes are stored in pre-order; `nodes[i]` is the node at `paths[i]`.
pub struct YogaTree {
    nodes: Vec<Node>,
    paths: Vec<Vec<usize>>,
    config: yoga::Config,
    direction: Direction,
}

impl Drop for YogaTree {
    fn drop(&mut self) {
        // Children before parents, so YGNodeFree never sees a freed owner.
        while self.nodes.pop().is_some() {}
    }
}

pub fn path_string(path: &[usize]) -> String {
    if path.is_empty() {
        "root".into()
    } else {
        path.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(".")
    }
}

impl YogaTree {
    pub fn new(case: &Case, config: Config) -> YogaTree {
        let mut yoga_config = yoga::Config::new();
        config.apply(&mut yoga_config);
        let mut tree = YogaTree {
            nodes: Vec::new(),
            paths: Vec::new(),
            config: yoga_config,
            direction: direction(&case.layout_inputs.owner_direction),
        };
        tree.build(&case.tree, Vec::new());
        tree
    }

    fn build(&mut self, j: &TreeNode, path: Vec<usize>) -> usize {
        let index = self.nodes.len();
        let mut node = Node::new_with_config(&mut self.config);
        if let Some(text) = j.text {
            node.set_context(Some(yoga::Context::new(Ctx { path: path_string(&path), text })));
            node.set_measure_func(Some(measure));
            if text.baseline {
                node.set_baseline_func(Some(baseline));
            }
        }
        apply_node_props(&mut node, &j.node);
        apply_style(&mut node, &j.style);
        self.nodes.push(node);
        self.paths.push(path.clone());
        for (i, child) in j.children.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(i);
            let child_index = self.build(child, child_path);
            let (parents, children) = self.nodes.split_at_mut(child_index);
            parents[index].insert_child(&mut children[0], i);
        }
        index
    }

    fn index_of(&self, path: &[usize]) -> usize {
        self.paths.iter().position(|p| p == path).expect("no node at path")
    }

    pub fn apply_step(&mut self, step: &Step, tree: &TreeNode) {
        match step {
            Step::SetStyle { path, .. } => {
                let i = self.index_of(path);
                apply_style(&mut self.nodes[i], &tree.get(path).style);
            }
            Step::MarkDirty { path } => {
                let i = self.index_of(path);
                self.nodes[i].mark_dirty();
            }
            Step::LayoutInputs(inputs) => self.direction = direction(&inputs.owner_direction),
        }
    }

    pub fn calculate_layout(&mut self, inputs: &LayoutInputs) -> Run {
        LOG.with(|l| *l.borrow_mut() = CallLog::default());
        let w = inputs.available_width.unwrap_or(Undefined);
        let h = inputs.available_height.unwrap_or(Undefined);
        self.nodes[0].calculate_layout(w, h, self.direction);
        // Positions in root coordinates (see TaffySide::calculate_layout)
        let mut origin = vec![[0.0f32; 2]; self.nodes.len()];
        let mut layout = Vec::with_capacity(self.nodes.len());
        for (index, n) in self.nodes.iter().enumerate() {
            let parent = self.paths[index]
                .split_last()
                .and_then(|(_, parent_path)| self.paths.iter().position(|p| p == parent_path));
            let base = parent.map(|p| origin[p]).unwrap_or([0.0, 0.0]);
            origin[index] = [base[0] + n.get_layout_left(), base[1] + n.get_layout_top()];
            layout.push(NodeLayout([origin[index][0], origin[index][1], n.get_layout_width(), n.get_layout_height()]));
        }
        let log = LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
        Run { layout, measure: log.measure, baseline: log.baseline }
    }
}
