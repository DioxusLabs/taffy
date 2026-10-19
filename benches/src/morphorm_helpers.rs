//! Lays out PanGUI benchmark templates (see [`crate::pangui_template`]) with [Morphorm](https://github.com/vizia/morphorm).
//!
//! The templates are translated into Morphorm's own layout model rather than from a Taffy style: `Expand` is
//! `Stretch`, `Fit` is `Auto` and `Ratio` is Morphorm's aspect ratio. Morphorm only applies the space around a node
//! (`left`, `right`, `top` and `bottom`) to absolutely positioned nodes, so templates which use `Margin` are not
//! supported.
use morphorm::{Alignment, Cache, LayoutType, LayoutWrap, Node, PositionType, Units};

use crate::pangui_template::{BenchmarkSize, FlowDirection, LayoutNode, SizeMode, WrapMode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeId(usize);

#[derive(Clone, Copy, Default)]
pub struct NodeStyle {
    layout_type: LayoutType,
    wrap: LayoutWrap,
    width: Units,
    height: Units,
    min_width: Option<Units>,
    min_height: Option<Units>,
    max_width: Option<Units>,
    max_height: Option<Units>,
    aspect_ratio: Option<f32>,
    padding_left: Units,
    padding_right: Units,
    padding_top: Units,
    padding_bottom: Units,
    gap: Units,
}

#[derive(Default)]
pub struct Tree {
    children: Vec<Vec<NodeId>>,
    parents: Vec<Option<NodeId>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct LayoutCache(pub Vec<Bounds>);

impl Cache for LayoutCache {
    type Node = NodeId;
    fn width(&self, node: &NodeId) -> f32 {
        self.0[node.0].width
    }
    fn height(&self, node: &NodeId) -> f32 {
        self.0[node.0].height
    }
    fn posx(&self, node: &NodeId) -> f32 {
        self.0[node.0].x
    }
    fn posy(&self, node: &NodeId) -> f32 {
        self.0[node.0].y
    }
    fn set_bounds(&mut self, node: &NodeId, x: f32, y: f32, width: f32, height: f32) {
        self.0[node.0] = Bounds { x, y, width, height };
    }
}

impl Node for NodeId {
    type Store = Vec<NodeStyle>;
    type Tree = Tree;
    type ChildIter<'t> = std::slice::Iter<'t, NodeId>;
    type CacheKey = NodeId;
    type SubLayout<'a> = ();

    fn key(&self) -> NodeId {
        *self
    }
    fn children<'t>(&'t self, tree: &'t Tree) -> Self::ChildIter<'t> {
        tree.children[self.0].iter()
    }
    fn parent<'t>(&'t self, tree: &'t Tree) -> Option<&'t Self> {
        tree.parents[self.0].as_ref()
    }
    fn visible(&self, _store: &Self::Store) -> bool {
        true
    }
    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType> {
        Some(store[self.0].layout_type)
    }
    fn position_type(&self, _store: &Self::Store) -> Option<PositionType> {
        Some(PositionType::Relative)
    }
    fn wrap(&self, store: &Self::Store) -> Option<LayoutWrap> {
        Some(store[self.0].wrap)
    }
    fn alignment(&self, _store: &Self::Store) -> Option<Alignment> {
        Some(Alignment::TopLeft)
    }
    fn width(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].width)
    }
    fn height(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].height)
    }
    fn aspect_ratio(&self, store: &Self::Store) -> Option<f32> {
        store[self.0].aspect_ratio
    }
    fn left(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn right(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn top(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn bottom(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn content_size(&self, _: &Self::Store, _: &mut (), _: Option<f32>, _: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn padding_left(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_left)
    }
    fn padding_right(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_right)
    }
    fn padding_top(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_top)
    }
    fn padding_bottom(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_bottom)
    }
    fn vertical_gap(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].gap)
    }
    fn horizontal_gap(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].gap)
    }
    fn min_vertical_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn min_horizontal_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn max_vertical_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn max_horizontal_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn min_width(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].min_width
    }
    fn min_height(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].min_height
    }
    fn max_width(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].max_width
    }
    fn max_height(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].max_height
    }
    fn grid_columns(&self, _store: &Self::Store) -> Option<Vec<Units>> {
        None
    }
    fn grid_rows(&self, _store: &Self::Store) -> Option<Vec<Units>> {
        None
    }
    fn column_start(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
    fn row_start(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
    fn column_span(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
    fn row_span(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
}

fn size(size: &Option<BenchmarkSize>) -> Units {
    match size {
        Some(BenchmarkSize { mode: SizeMode::Pixels, value }) => Units::Pixels(*value),
        Some(BenchmarkSize { mode: SizeMode::Percentage, value }) => Units::Percentage(*value * 100.0),
        Some(BenchmarkSize { mode: SizeMode::Expand, value }) => Units::Stretch(*value),
        // `Ratio` is translated to an aspect ratio
        Some(BenchmarkSize { mode: SizeMode::Fit | SizeMode::Ratio, .. }) | None => Units::Auto,
    }
}

/// `None` rather than `Auto`, which is the size of the content of the node in Morphorm
fn min_max_size(size: &Option<BenchmarkSize>, property: &str) -> Option<Units> {
    match size {
        Some(BenchmarkSize { mode: SizeMode::Pixels, value }) => Some(Units::Pixels(*value)),
        Some(BenchmarkSize { mode: SizeMode::Percentage, value }) => Some(Units::Percentage(*value * 100.0)),
        Some(BenchmarkSize { mode, .. }) => panic!("{mode:?} is not supported as a value of {property}"),
        None => None,
    }
}

fn pixels(value: Option<f32>) -> Units {
    Units::Pixels(value.unwrap_or(0.0))
}

/// Translate a node of a PanGUI template into a Morphorm style, or `None` if it can't be translated.
/// `parent_is_row` is whether the parent of the node lays out its children horizontally.
fn style(node: &LayoutNode, parent_is_row: bool) -> Option<NodeStyle> {
    let margins = [node.margin_left, node.margin_right, node.margin_top, node.margin_bottom];
    if margins.iter().any(|margin| margin.is_some_and(|margin| margin != 0.0)) {
        return None;
    }

    let width = size(&node.width);
    let height = size(&node.height);

    // A node which has `Expand` as its minimum size along the main axis of its parent grows from its own size.
    // Morphorm has no equivalent, so it stretches with its own size as its minimum size.
    let (width, min_width) = match (&node.min_width, parent_is_row) {
        (Some(BenchmarkSize { mode: SizeMode::Expand, value }), true) => (Units::Stretch(*value), Some(width)),
        (min, _) => (width, min_max_size(min, "MinWidth")),
    };
    let (height, min_height) = match (&node.min_height, parent_is_row) {
        (Some(BenchmarkSize { mode: SizeMode::Expand, value }), false) => (Units::Stretch(*value), Some(height)),
        (min, _) => (height, min_max_size(min, "MinHeight")),
    };

    let aspect_ratio = match (&node.width, &node.height) {
        (Some(BenchmarkSize { mode: SizeMode::Ratio, value }), _) => Some(*value),
        (_, Some(BenchmarkSize { mode: SizeMode::Ratio, value })) => Some(1.0 / *value),
        _ => None,
    };

    Some(NodeStyle {
        layout_type: match node.flow_direction {
            Some(FlowDirection::Horizontal) => LayoutType::Row,
            Some(FlowDirection::Vertical) | None => LayoutType::Column,
        },
        wrap: match node.wrap {
            Some(WrapMode::Auto) => LayoutWrap::Wrap,
            Some(WrapMode::NoWrap) | None => LayoutWrap::NoWrap,
        },
        width,
        height,
        min_width,
        min_height,
        max_width: min_max_size(&node.max_width, "MaxWidth"),
        max_height: min_max_size(&node.max_height, "MaxHeight"),
        aspect_ratio,
        padding_left: pixels(node.padding_left),
        padding_right: pixels(node.padding_right),
        padding_top: pixels(node.padding_top),
        padding_bottom: pixels(node.padding_bottom),
        gap: pixels(node.gap),
    })
}

/// A Morphorm tree built from a PanGUI template
pub struct MorphormTree {
    pub tree: Tree,
    pub store: Vec<NodeStyle>,
    pub cache: LayoutCache,
    /// A column of the size that the root node of the template is laid out into, as Morphorm lays out
    /// the root node of the tree at its own size in pixels (or 0).
    pub viewport: NodeId,
}

impl MorphormTree {
    /// Returns `None` if the template uses a feature which Morphorm doesn't support
    pub fn new(root: &LayoutNode, width: f32, height: f32) -> Option<Self> {
        let mut tree =
            Self { tree: Tree::default(), store: Vec::new(), cache: LayoutCache::default(), viewport: NodeId(0) };
        let viewport = NodeStyle {
            layout_type: LayoutType::Column,
            width: Units::Pixels(width),
            height: Units::Pixels(height),
            ..Default::default()
        };
        tree.add_node(viewport, None);
        // PanGUI's own translation treats the root node as the child of a column
        tree.add_template_node(root, false, tree.viewport)?;
        tree.cache.0.resize(tree.store.len(), Bounds::default());
        Some(tree)
    }

    fn add_node(&mut self, style: NodeStyle, parent: Option<NodeId>) -> NodeId {
        let node = NodeId(self.store.len());
        self.store.push(style);
        self.tree.children.push(Vec::new());
        self.tree.parents.push(parent);
        if let Some(parent) = parent {
            self.tree.children[parent.0].push(node);
        }
        node
    }

    fn add_template_node(&mut self, template: &LayoutNode, parent_is_row: bool, parent: NodeId) -> Option<()> {
        let node = self.add_node(style(template, parent_is_row)?, Some(parent));
        let is_row = template.flow_direction == Some(FlowDirection::Horizontal);
        for child in &template.children {
            self.add_template_node(child, is_row, node)?;
        }
        Some(())
    }

    /// Lay out the whole tree. Morphorm has no layout cache, so this always lays out every node.
    pub fn compute_layout(&mut self) {
        self.viewport.layout(&mut self.cache, &self.tree, &self.store, &mut ());
    }
}
