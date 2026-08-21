//! Tests that layout algorithms output correct first/last baselines in `LayoutOutput`.
//!
//! These use a minimal custom tree (rather than `TaffyTree`) so that the `LayoutOutput`
//! returned for the root container can be inspected directly.
#[cfg(test)]
mod last_baseline {
    use taffy::prelude::*;
    use taffy::{
        compute_block_layout, compute_cached_layout, compute_flexbox_layout, compute_grid_layout, Baselines, Cache,
        CacheTree, LayoutInput, LayoutOutput, LayoutPartialTree, RunMode, SizingMode,
    };

    #[derive(Debug, Copy, Clone)]
    enum NodeKind {
        Flexbox,
        Grid,
        Block,
        /// A leaf with a fixed size that reports the given baselines (offsets from its top edge)
        Leaf(Baselines),
    }

    struct Node {
        kind: NodeKind,
        style: Style,
        cache: Cache,
        unrounded_layout: Layout,
        children: Vec<usize>,
    }

    struct Tree {
        nodes: Vec<Node>,
    }

    impl Tree {
        fn new() -> Tree {
            Tree { nodes: Vec::new() }
        }

        fn add_node(&mut self, kind: NodeKind, style: Style, children: &[usize]) -> usize {
            self.nodes.push(Node {
                kind,
                style,
                cache: Cache::new(),
                unrounded_layout: Layout::with_order(0),
                children: children.to_vec(),
            });
            self.nodes.len() - 1
        }

        fn add_leaf(&mut self, width: f32, height: f32, first: f32, last: f32) -> usize {
            self.add_node(
                NodeKind::Leaf(Baselines { first: Some(first), last: Some(last) }),
                Style { size: Size { width: length(width), height: length(height) }, ..Default::default() },
                &[],
            )
        }

        /// Perform layout on the given root node and return its `LayoutOutput`
        fn layout_root(&mut self, root: usize) -> LayoutOutput {
            self.compute_child_layout(
                NodeId::from(root),
                LayoutInput {
                    known_dimensions: Size::NONE,
                    known_dimensions_are_definite: taffy::geometry::Size { width: false, height: false },
                    parent_size: Size::NONE,
                    available_space: Size::MAX_CONTENT,
                    sizing_mode: SizingMode::InherentSize,
                    run_mode: RunMode::PerformLayout,
                    axis: taffy::RequestedAxis::Both,
                    vertical_margins_are_collapsible: taffy::geometry::Line::FALSE,
                },
            )
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

    impl LayoutPartialTree for Tree {
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

        fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
            0.0
        }

        fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
            compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
                let node = &tree.nodes[usize::from(node_id)];

                match node.kind {
                    NodeKind::Flexbox => compute_flexbox_layout(tree, node_id, inputs),
                    NodeKind::Grid => compute_grid_layout(tree, node_id, inputs),
                    NodeKind::Block => compute_block_layout(tree, node_id, inputs, None),
                    NodeKind::Leaf(baselines) => {
                        let style_size =
                            node.style.size.map(|dim| dim.into_option().unwrap_or(0.0)).map(Some).unwrap_or(Size::ZERO);
                        let size = inputs.known_dimensions.unwrap_or(style_size);
                        LayoutOutput::from_sizes_and_baselines(size, taffy::Rect::ZERO, baselines)
                    }
                }
            })
        }
    }

    impl CacheTree for Tree {
        fn cache_get(&self, node_id: NodeId, inputs: &LayoutInput) -> Option<LayoutOutput> {
            self.nodes[usize::from(node_id)].cache.get(inputs)
        }

        fn cache_store(&mut self, node_id: NodeId, inputs: &LayoutInput, layout_output: LayoutOutput) {
            self.nodes[usize::from(node_id)].cache.store(inputs, layout_output)
        }

        fn cache_clear(&mut self, node_id: NodeId) {
            self.nodes[usize::from(node_id)].cache.clear();
        }
    }

    impl taffy::LayoutFlexboxContainer for Tree {
        type FlexboxContainerStyle<'a>
            = &'a Style
        where
            Self: 'a;

        type FlexboxItemStyle<'a>
            = &'a Style
        where
            Self: 'a;

        fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
            &self.nodes[usize::from(node_id)].style
        }

        fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
            &self.nodes[usize::from(child_node_id)].style
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

    impl taffy::LayoutBlockContainer for Tree {
        type BlockContainerStyle<'a>
            = &'a Style
        where
            Self: 'a;

        type BlockItemStyle<'a>
            = &'a Style
        where
            Self: 'a;

        fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_> {
            &self.nodes[usize::from(node_id)].style
        }

        fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_> {
            &self.nodes[usize::from(child_node_id)].style
        }
    }

    /// A flex row's first baseline comes from its first item and its last baseline from its
    /// last item, each measured relative to the container.
    #[test]
    fn flex_row_baselines() {
        let mut tree = Tree::new();
        let child_a = tree.add_leaf(50.0, 50.0, 30.0, 40.0);
        let child_b = tree.add_leaf(60.0, 60.0, 20.0, 55.0);
        let root = tree.add_node(
            NodeKind::Flexbox,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                size: Size { width: length(200.0), height: length(100.0) },
                ..Default::default()
            },
            &[child_a, child_b],
        );

        let output = tree.layout_root(root);
        assert_eq!(output.baselines, Baselines { first: Some(30.0), last: Some(55.0) });
    }

    /// A flex column's first baseline comes from its first item and its last baseline from its
    /// last item, offset by the items' main-axis positions.
    #[test]
    fn flex_column_baselines() {
        let mut tree = Tree::new();
        let child_a = tree.add_leaf(50.0, 50.0, 30.0, 40.0);
        let child_b = tree.add_leaf(60.0, 60.0, 20.0, 55.0);
        let root = tree.add_node(
            NodeKind::Flexbox,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                size: Size { width: length(200.0), height: length(200.0) },
                ..Default::default()
            },
            &[child_a, child_b],
        );

        let output = tree.layout_root(root);
        // Child B is positioned at y=50 (below child A)
        assert_eq!(output.baselines, Baselines { first: Some(30.0), last: Some(50.0 + 55.0) });
    }

    /// A multi-line flex row's first baseline comes from the first line and its last baseline
    /// from the last line.
    #[test]
    fn flex_wrap_baselines() {
        let mut tree = Tree::new();
        let child_a = tree.add_leaf(60.0, 50.0, 30.0, 40.0);
        let child_b = tree.add_leaf(60.0, 50.0, 20.0, 45.0);
        let child_c = tree.add_leaf(60.0, 60.0, 25.0, 55.0);
        let root = tree.add_node(
            NodeKind::Flexbox,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                align_content: Some(AlignContent::FLEX_START),
                size: Size { width: length(130.0), height: length(200.0) },
                ..Default::default()
            },
            &[child_a, child_b, child_c],
        );

        let output = tree.layout_root(root);
        // Line 1 contains children A and B (height 50), line 2 contains child C at y=50
        assert_eq!(output.baselines, Baselines { first: Some(30.0), last: Some(50.0 + 55.0) });
    }

    /// For wrap-reverse containers the lines are in reverse order: the first baseline is
    /// generated from the cross-start-most line (the last line) and the last baseline from
    /// the cross-end-most line (the first line).
    #[test]
    fn flex_wrap_reverse_baselines() {
        let mut tree = Tree::new();
        let child_a = tree.add_leaf(60.0, 50.0, 30.0, 40.0);
        let child_b = tree.add_leaf(60.0, 50.0, 20.0, 45.0);
        let child_c = tree.add_leaf(60.0, 60.0, 25.0, 55.0);
        let root = tree.add_node(
            NodeKind::Flexbox,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::WrapReverse,
                align_content: Some(AlignContent::FLEX_START),
                size: Size { width: length(130.0), height: length(200.0) },
                ..Default::default()
            },
            &[child_a, child_b, child_c],
        );

        let output = tree.layout_root(root);
        // Wrap-reverse flips the cross axis, so `flex-start` packs lines at the bottom: line 1
        // (children A and B, height 50) occupies y=150..200 and line 2 (child C, height 60)
        // occupies y=90..150. The first baseline is generated from the cross-start-most line
        // (child C's line) and the last baseline from the cross-end-most line (child B's line).
        assert_eq!(output.baselines, Baselines { first: Some(90.0 + 25.0), last: Some(150.0 + 45.0) });
    }

    /// A grid container's first baseline is generated from the first row and its last baseline
    /// from the last row.
    #[test]
    fn grid_baselines() {
        let mut tree = Tree::new();
        let child_a = tree.add_leaf(50.0, 50.0, 30.0, 40.0);
        let child_b = tree.add_leaf(60.0, 60.0, 20.0, 55.0);
        let root = tree.add_node(
            NodeKind::Grid,
            Style {
                display: Display::Grid,
                grid_template_columns: vec![length(100.0)],
                grid_template_rows: vec![length(50.0), length(60.0)],
                ..Default::default()
            },
            &[child_a, child_b],
        );

        let output = tree.layout_root(root);
        // First baseline: the first row's item's first baseline (y=0 + 30).
        // Last baseline: the last row's item's last baseline (y=50 + 55).
        assert_eq!(output.baselines, Baselines { first: Some(30.0), last: Some(50.0 + 55.0) });
    }

    /// A block container's first baseline comes from its first in-flow child with a baseline,
    /// and its last baseline from its last in-flow child with one.
    #[test]
    fn block_baselines() {
        let mut tree = Tree::new();
        let child_a = tree.add_leaf(50.0, 50.0, 30.0, 40.0);
        let child_b = tree.add_leaf(60.0, 60.0, 20.0, 55.0);
        let root = tree.add_node(
            NodeKind::Block,
            Style {
                display: Display::Block,
                size: Size { width: length(200.0), height: auto() },
                ..Default::default()
            },
            &[child_a, child_b],
        );

        let output = tree.layout_root(root);
        assert_eq!(output.baselines, Baselines { first: Some(30.0), last: Some(50.0 + 55.0) });
    }
}
