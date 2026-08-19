//! Tests for `calc()` gaps on grid containers, including re-resolution against
//! the container's resolved content-box size when the percentage basis is
//! initially indefinite.
//!
//! These use a custom tree because `TaffyTree`'s calc resolver always returns zero.
use taffy::{
    compute_cached_layout, compute_grid_layout, compute_leaf_layout, compute_root_layout, prelude::*, round_layout,
    Cache, CacheTree, Point,
};

/// A calc value of the form `calc(<length>px + <percent>%)`
struct CalcValue {
    length: f32,
    /// Percentage as a fraction (e.g. 0.05 for 5%)
    percent: f32,
}

fn calc_length_percentage(length: f32, percent: f32) -> LengthPercentage {
    let ptr: &'static CalcValue = Box::leak(Box::new(CalcValue { length, percent }));
    LengthPercentage::calc(ptr as *const CalcValue as *const ())
}

struct Node {
    style: Style,
    is_grid: bool,
    cache: Cache,
    unrounded_layout: Layout,
    final_layout: Layout,
    children: Vec<usize>,
}

impl Node {
    fn new_grid(style: Style) -> Node {
        Node {
            style: Style { display: Display::Grid, ..style },
            is_grid: true,
            cache: Cache::new(),
            unrounded_layout: Layout::with_order(0),
            final_layout: Layout::with_order(0),
            children: Vec::new(),
        }
    }

    fn new_leaf(style: Style) -> Node {
        Node {
            style,
            is_grid: false,
            cache: Cache::new(),
            unrounded_layout: Layout::with_order(0),
            final_layout: Layout::with_order(0),
            children: Vec::new(),
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Tree {
        Tree { nodes: Vec::new() }
    }

    fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn append_child(&mut self, parent: usize, child: usize) {
        self.nodes[parent].children.push(child);
    }

    fn compute_layout(&mut self, root: usize, available_space: Size<AvailableSpace>) {
        compute_root_layout(self, NodeId::from(root), available_space);
        round_layout(self, NodeId::from(root));
    }

    fn layout(&self, node: usize) -> &Layout {
        &self.nodes[node].final_layout
    }
}

struct ChildIter<'a>(std::slice::Iter<'a, usize>);
impl Iterator for ChildIter<'_> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(NodeId::from)
    }
}

impl taffy::TraversePartialTree for Tree {
    type ChildIter<'a> = ChildIter<'a>;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.nodes[usize::from(node_id)].children.iter())
    }

    fn child_count(&self, node_id: NodeId) -> usize {
        self.nodes[usize::from(node_id)].children.len()
    }

    fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(self.nodes[usize::from(node_id)].children[index])
    }
}

impl taffy::TraverseTree for Tree {}

impl taffy::LayoutPartialTree for Tree {
    type CustomIdent = String;

    type CoreContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.nodes[usize::from(node_id)].style
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.nodes[usize::from(node_id)].unrounded_layout = *layout;
    }

    #[allow(unsafe_code)]
    fn resolve_calc_value(&self, val: *const (), basis: f32) -> f32 {
        let calc = unsafe { &*(val as *const CalcValue) };
        calc.length + calc.percent * basis
    }

    fn compute_child_layout(&mut self, node_id: NodeId, inputs: taffy::tree::LayoutInput) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            let node = &tree.nodes[usize::from(node_id)];
            if node.is_grid {
                compute_grid_layout(tree, node_id, inputs)
            } else {
                compute_leaf_layout(
                    inputs,
                    &node.style,
                    |_val, _basis| 0.0,
                    |_known_dimensions, _available_space| Size::ZERO,
                )
            }
        })
    }
}

impl CacheTree for Tree {
    fn cache_get(&self, node_id: NodeId, inputs: &taffy::LayoutInput) -> Option<taffy::LayoutOutput> {
        self.nodes[usize::from(node_id)].cache.get(inputs)
    }

    fn cache_store(&mut self, node_id: NodeId, inputs: &taffy::LayoutInput, layout_output: taffy::LayoutOutput) {
        self.nodes[usize::from(node_id)].cache.store(inputs, layout_output)
    }

    fn cache_clear(&mut self, node_id: NodeId) {
        self.nodes[usize::from(node_id)].cache.clear();
    }
}

impl taffy::LayoutGridContainer for Tree {
    type GridContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type GridItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_> {
        &self.nodes[usize::from(node_id)].style
    }

    fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_> {
        &self.nodes[usize::from(child_node_id)].style
    }
}

impl taffy::RoundTree for Tree {
    fn get_unrounded_layout(&self, node_id: NodeId) -> Layout {
        self.nodes[usize::from(node_id)].unrounded_layout
    }

    fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.nodes[usize::from(node_id)].final_layout = *layout;
    }
}

fn build_grid(gap: LengthPercentage, width: Dimension) -> (Tree, usize, [usize; 4]) {
    let mut tree = Tree::new();
    let grid = tree.add_node(Node::new_grid(Style {
        size: Size { width, height: Dimension::auto() },
        gap: Size { width: gap, height: gap },
        grid_template_columns: vec![length(90.0), length(90.0)],
        grid_template_rows: vec![length(90.0), length(90.0)],
        ..Style::default()
    }));
    let mut children = [0; 4];
    for child in &mut children {
        *child = tree.add_node(Node::new_leaf(Style::default()));
        tree.append_child(grid, *child);
    }
    (tree, grid, children)
}

/// Mirrors the second grid of WPT css/css-grid/alignment/grid-gutters-009.html and
/// grid-gutters-010.html: `display: inline-grid; width: auto; gap: calc(20px + 5%)`
/// with 90px tracks in both axes.
///
/// During intrinsic sizing the percentage part of the gap resolves to zero, so
/// the container sizes to 90 + 20 + 90 = 200 in both axes. The gaps then
/// re-resolve against that content-box size to calc(20px + 5% * 200) = 30px,
/// which offsets track positions without resizing the container.
#[test]
fn calc_gap_indefinite_both_axes() {
    let (mut tree, grid, children) = build_grid(calc_length_percentage(20.0, 0.05), Dimension::auto());
    tree.compute_layout(grid, Size::MAX_CONTENT);

    assert_eq!(tree.layout(grid).size, Size { width: 200.0, height: 200.0 });
    let expected_locations = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 120.0, y: 0.0 },
        Point { x: 0.0, y: 120.0 },
        Point { x: 120.0, y: 120.0 },
    ];
    for (child, expected_location) in children.iter().zip(expected_locations) {
        assert_eq!(tree.layout(*child).size, Size { width: 90.0, height: 90.0 });
        assert_eq!(tree.layout(*child).location, expected_location);
    }
}

/// Like the first grid of WPT grid-gutters-009/010 but with a calc() gap:
/// `width: 200px; gap: calc(20px + 5%)`. The column gap resolves against the
/// definite width immediately, while the row gap initially resolves its
/// percentage part against zero and then re-resolves against the resolved
/// content-box height of 200px.
#[test]
fn calc_gap_definite_width_indefinite_height() {
    let (mut tree, grid, children) = build_grid(calc_length_percentage(20.0, 0.05), length(200.0));
    tree.compute_layout(grid, Size { width: AvailableSpace::Definite(800.0), height: AvailableSpace::Definite(600.0) });

    assert_eq!(tree.layout(grid).size, Size { width: 200.0, height: 200.0 });
    let expected_locations = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 120.0, y: 0.0 },
        Point { x: 0.0, y: 120.0 },
        Point { x: 120.0, y: 120.0 },
    ];
    for (child, expected_location) in children.iter().zip(expected_locations) {
        assert_eq!(tree.layout(*child).size, Size { width: 90.0, height: 90.0 });
        assert_eq!(tree.layout(*child).location, expected_location);
    }
}
