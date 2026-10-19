//! Lays out PanGUI benchmark templates (see [`crate::pangui_template`]) with [Torin](https://github.com/marc2332/freya/tree/main/crates/torin),
//! the layout library of Freya.
//!
//! The templates are translated into Torin's own layout model rather than from a Taffy style: `Expand` is `Flex`
//! (in a parent with `Content::Flex`) along the main axis of the parent and `Fill` along its cross axis, and
//! `Fit` is `Inner`. Torin has no aspect ratio, so templates which use `Ratio` are not supported.
use torin::prelude::*;

use crate::pangui_template::{BenchmarkSize, FlowDirection, LayoutNode, SizeMode, WrapMode};

/// The nodes of a tree in the form that Torin reads them, indexed by node id. The root node has id 0.
#[derive(Default)]
pub struct Tree {
    nodes: Vec<Node>,
    children: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>,
    heights: Vec<u16>,
}

impl TreeAdapter<usize> for Tree {
    fn root_id(&self) -> usize {
        0
    }
    fn read_node<R>(&self, node_id: &usize, reader: impl FnOnce(&Node, &[usize]) -> R) -> Option<R> {
        Some(reader(self.nodes.get(*node_id)?, &self.children[*node_id]))
    }
    fn height(&self, node_id: &usize) -> Option<u16> {
        self.heights.get(*node_id).copied()
    }
    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        *self.parents.get(*node_id)?
    }
}

fn is_mode(size: &Option<BenchmarkSize>, mode: SizeMode) -> bool {
    size.is_some_and(|size| size.mode == mode)
}

/// `is_main` is whether the size is along the main axis of the parent of the node
fn size(size: &Option<BenchmarkSize>, is_main: bool) -> Option<Size> {
    Some(match size {
        Some(BenchmarkSize { mode: SizeMode::Pixels, value }) => Size::px(*value),
        Some(BenchmarkSize { mode: SizeMode::Percentage, value }) => Size::percent(*value * 100.0),
        Some(BenchmarkSize { mode: SizeMode::Expand, value }) if is_main => Size::flex(*value),
        Some(BenchmarkSize { mode: SizeMode::Expand, .. }) => Size::fill(),
        Some(BenchmarkSize { mode: SizeMode::Ratio, .. }) => return None,
        Some(BenchmarkSize { mode: SizeMode::Fit, .. }) | None => Size::auto(),
    })
}

fn min_max_size(size: &Option<BenchmarkSize>, property: &str) -> Size {
    match size {
        Some(BenchmarkSize { mode: SizeMode::Pixels, value }) => Size::px(*value),
        Some(BenchmarkSize { mode: SizeMode::Percentage, value }) => Size::percent(*value * 100.0),
        Some(BenchmarkSize { mode, .. }) => panic!("{mode:?} is not supported as a value of {property}"),
        None => Size::auto(),
    }
}

/// Translate a node of a PanGUI template into a Torin node, or `None` if it can't be translated.
/// `parent_is_row` is whether the parent of the node lays out its children horizontally.
fn node(template: &LayoutNode, parent_is_row: bool) -> Option<Node> {
    let width = size(&template.width, parent_is_row)?;
    let height = size(&template.height, !parent_is_row)?;

    // A node which has `Expand` as its minimum size along the main axis of its parent grows from its own size.
    // Torin has no equivalent, so it flexes with its own size as its minimum size.
    let (width, minimum_width) = match (&template.min_width, parent_is_row) {
        (Some(BenchmarkSize { mode: SizeMode::Expand, value }), true) => (Size::flex(*value), width),
        (min, _) => (width, min_max_size(min, "MinWidth")),
    };
    let (height, minimum_height) = match (&template.min_height, parent_is_row) {
        (Some(BenchmarkSize { mode: SizeMode::Expand, value }), false) => (Size::flex(*value), height),
        (min, _) => (height, min_max_size(min, "MinHeight")),
    };

    let is_row = template.flow_direction == Some(FlowDirection::Horizontal);
    let gap = template.gap.unwrap_or(0.0);
    let content = if template.wrap == Some(WrapMode::Auto) {
        Content::wrap_spacing(gap)
    } else if template.children.iter().any(|child| {
        let main_size = if is_row { (&child.width, &child.min_width) } else { (&child.height, &child.min_height) };
        is_mode(main_size.0, SizeMode::Expand) || is_mode(main_size.1, SizeMode::Expand)
    }) {
        Content::flex()
    } else {
        Content::normal()
    };

    let gaps = |left: Option<f32>, right: Option<f32>, top: Option<f32>, bottom: Option<f32>| {
        Gaps::new(top.unwrap_or(0.0), right.unwrap_or(0.0), bottom.unwrap_or(0.0), left.unwrap_or(0.0))
    };

    Some(Node {
        width,
        height,
        minimum_width,
        minimum_height,
        maximum_width: min_max_size(&template.max_width, "MaxWidth"),
        maximum_height: min_max_size(&template.max_height, "MaxHeight"),
        direction: if is_row { Direction::Horizontal } else { Direction::Vertical },
        content,
        spacing: Length::new(gap),
        padding: gaps(template.padding_left, template.padding_right, template.padding_top, template.padding_bottom),
        margin: gaps(template.margin_left, template.margin_right, template.margin_top, template.margin_bottom),
        ..Node::default()
    })
}

/// A Torin tree built from a PanGUI template
pub struct TorinTree {
    pub tree: Tree,
    pub layout: Torin<usize>,
    root_area: Area,
}

impl TorinTree {
    /// Returns `None` if the template uses a feature which Torin doesn't support
    pub fn new(root: &LayoutNode, width: f32, height: f32) -> Option<Self> {
        let mut tree = Tree::default();
        // PanGUI's own translation treats the root node as the child of a column
        add_template_node(&mut tree, root, false, None)?;
        let root_area = Area::new(Point2D::new(0.0, 0.0), Size2D::new(width, height));
        Some(Self { tree, layout: Torin::new(), root_area })
    }

    /// Discard the cached layout of every node
    pub fn mark_all_dirty(&mut self) {
        self.layout.reset();
    }

    pub fn compute_layout(&mut self) {
        self.layout.measure(0, self.root_area, &mut None::<NoopMeasurer>, &self.tree);
    }
}

fn add_template_node(tree: &mut Tree, template: &LayoutNode, parent_is_row: bool, parent: Option<usize>) -> Option<()> {
    let id = tree.nodes.len();
    tree.nodes.push(node(template, parent_is_row)?);
    tree.children.push(Vec::new());
    tree.parents.push(parent);
    tree.heights.push(parent.map_or(1, |parent| tree.heights[parent] + 1));
    if let Some(parent) = parent {
        tree.children[parent].push(id);
    }
    let is_row = template.flow_direction == Some(FlowDirection::Horizontal);
    for child in &template.children {
        add_template_node(tree, child, is_row, Some(id))?;
    }
    Some(())
}
