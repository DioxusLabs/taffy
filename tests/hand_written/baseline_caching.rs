//! Baselines returned by `RunMode::ComputeSize` runs must not depend on whether the result came from
//! the cache. Size-only runs are how a parent that needs a child's baseline before laying the child
//! out (such as table row sizing, or an embedder's inline layout) measures it, so these tests use a
//! minimal custom tree that runs `ComputeSize` directly, as such a parent would.
#[cfg(feature = "block_layout")]
mod baseline_caching {
    use taffy::{
        compute_block_layout, compute_cached_layout, prelude::*, Baselines, Cache, CacheTree, Layout, LayoutInput,
        LayoutOutput, RequestedAxis, RunMode, SizingMode, Style,
    };

    enum Kind {
        Block,
        /// A leaf whose measure function returns a fixed size and first baseline
        Leaf {
            size: Size<f32>,
            baseline: Option<f32>,
        },
    }

    struct Node {
        kind: Kind,
        style: Style,
        cache: Cache,
        layout: Layout,
        children: Vec<usize>,
        /// How many times the measure function of a leaf has been called
        measure_count: usize,
    }

    struct Tree {
        nodes: Vec<Node>,
    }

    impl Tree {
        fn new() -> Self {
            Tree { nodes: Vec::new() }
        }

        fn add(&mut self, kind: Kind, style: Style, children: &[NodeId]) -> NodeId {
            self.nodes.push(Node {
                kind,
                style,
                cache: Cache::new(),
                layout: Layout::with_order(0),
                children: children.iter().map(|&id| usize::from(id)).collect(),
                measure_count: 0,
            });
            NodeId::from(self.nodes.len() - 1)
        }

        fn leaf(&mut self, style: Style, size: Size<f32>, baseline: Option<f32>) -> NodeId {
            self.add(Kind::Leaf { size, baseline }, style, &[])
        }

        fn block(&mut self, style: Style, children: &[NodeId]) -> NodeId {
            self.add(Kind::Block, Style { display: Display::Block, ..style }, children)
        }

        fn node(&self, node_id: NodeId) -> &Node {
            &self.nodes[usize::from(node_id)]
        }

        fn node_mut(&mut self, node_id: NodeId) -> &mut Node {
            &mut self.nodes[usize::from(node_id)]
        }

        /// Measure `node_id` the way a table measures a cell: with a known width and an auto height
        fn measure(&mut self, node_id: NodeId, width: f32, available_height: AvailableSpace) -> LayoutOutput {
            self.compute_child_layout(
                node_id,
                LayoutInput {
                    run_mode: RunMode::ComputeSize,
                    sizing_mode: SizingMode::InherentSize,
                    axis: RequestedAxis::Both,
                    known_dimensions: Size { width: Some(width), height: None },
                    known_dimensions_are_definite: Size { width: true, height: true },
                    parent_size: Size { width: Some(width), height: None },
                    available_space: Size { width: AvailableSpace::Definite(width), height: available_height },
                    vertical_margins_are_collapsible: Line::FALSE,
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
            ChildIter(self.node(node_id).children.iter())
        }
        fn child_count(&self, node_id: NodeId) -> usize {
            self.node(node_id).children.len()
        }
        fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
            NodeId::from(self.node(node_id).children[index])
        }
    }

    impl taffy::LayoutPartialTree for Tree {
        type CoreContainerStyle<'a>
            = &'a Style
        where
            Self: 'a;
        type CustomIdent = String;

        fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
            &self.node(node_id).style
        }
        fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
            self.node_mut(node_id).layout = *layout
        }
        fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
            compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| match tree.node(node_id).kind {
                Kind::Block => compute_block_layout(tree, node_id, inputs, None),
                Kind::Leaf { size, baseline } => {
                    tree.node_mut(node_id).measure_count += 1;
                    let size = inputs.known_dimensions.unwrap_or(size);
                    LayoutOutput::from_sizes_and_baselines(size, Rect::ZERO, Baselines::from_first(baseline))
                }
            })
        }
    }

    impl taffy::LayoutContainingBlock for Tree {
        type OofItemStyle<'a>
            = &'a Style
        where
            Self: 'a;
        fn get_oof_item_style(&self, node_id: NodeId) -> Self::OofItemStyle<'_> {
            &self.node(node_id).style
        }
        fn clear_hoisted_children(&mut self, _node_id: NodeId) {}
        fn add_hoisted_children(&mut self, _node_id: NodeId, _hoisted: &[NodeId]) {}
    }

    impl CacheTree for Tree {
        fn cache_get(&mut self, node_id: NodeId, inputs: &LayoutInput) -> Option<LayoutOutput> {
            self.node_mut(node_id).cache.get(inputs)
        }
        fn cache_store(&mut self, node_id: NodeId, inputs: &LayoutInput, layout_output: LayoutOutput) {
            self.node_mut(node_id).cache.store(inputs, layout_output)
        }
        fn cache_clear(&mut self, node_id: NodeId) {
            self.node_mut(node_id).cache.clear();
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
            &self.node(node_id).style
        }
        fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_> {
            &self.node(child_node_id).style
        }
    }

    fn fixed_size(width: f32, height: f32) -> Style {
        Style { size: Size { width: length(width), height: length(height) }, ..Default::default() }
    }

    fn padding_top(top: f32) -> Style {
        Style { padding: Rect { left: zero(), right: zero(), top: length(top), bottom: zero() }, ..Default::default() }
    }

    /// A leaf's baseline comes from its measure function, and measuring the leaf again with the
    /// same inputs returns it from the cache rather than calling the measure function again
    #[test]
    fn leaf_baseline_survives_a_compute_size_cache_hit() {
        let mut tree = Tree::new();
        let leaf = tree.leaf(Style::default(), Size { width: 50.0, height: 50.0 }, Some(40.0));

        let first = tree.measure(leaf, 50.0, AvailableSpace::MaxContent);
        let second = tree.measure(leaf, 50.0, AvailableSpace::MaxContent);

        assert_eq!(tree.node(leaf).measure_count, 1);
        assert_eq!(first.baselines.first, Some(40.0));
        assert_eq!(second, first);
    }

    /// A block container's first baseline is its first child's, offset by the child's position.
    /// Measuring the block again with the same inputs is a cache hit for the block itself.
    #[test]
    fn block_baseline_survives_a_compute_size_cache_hit() {
        let mut tree = Tree::new();
        let leaf = tree.leaf(Style::default(), Size { width: 50.0, height: 50.0 }, Some(40.0));
        let cell = tree.block(padding_top(10.0), &[leaf]);

        let first = tree.measure(cell, 50.0, AvailableSpace::MaxContent);
        let second = tree.measure(cell, 50.0, AvailableSpace::MaxContent);

        assert_eq!(tree.node(leaf).measure_count, 1);
        assert_eq!(first.size, Size { width: 50.0, height: 60.0 });
        assert_eq!(first.baselines.first, Some(50.0));
        assert_eq!(second, first);
    }

    /// Measuring a block container with new inputs re-runs block layout, but its child's inputs
    /// are unchanged, so the child's measurement (and the baseline the block takes from it) comes
    /// from the child's cache
    #[test]
    fn block_baseline_survives_a_compute_size_cache_hit_of_its_child() {
        let mut tree = Tree::new();
        let leaf = tree.leaf(fixed_size(50.0, 50.0), Size { width: 50.0, height: 50.0 }, Some(40.0));
        let cell = tree.block(padding_top(10.0), &[leaf]);

        let first = tree.measure(cell, 100.0, AvailableSpace::MaxContent);
        let second = tree.measure(cell, 100.0, AvailableSpace::Definite(500.0));

        assert_eq!(tree.node(leaf).measure_count, 1);
        assert_eq!(first.baselines.first, Some(50.0));
        assert_eq!(second.baselines.first, Some(50.0));
    }

    /// A node without a baseline still has none when it is measured from the cache
    #[test]
    fn missing_baseline_survives_a_compute_size_cache_hit() {
        let mut tree = Tree::new();
        let leaf = tree.leaf(Style::default(), Size { width: 50.0, height: 50.0 }, None);
        let cell = tree.block(padding_top(10.0), &[leaf]);

        let first = tree.measure(cell, 50.0, AvailableSpace::MaxContent);
        let second = tree.measure(cell, 50.0, AvailableSpace::MaxContent);

        assert_eq!(first.baselines, Baselines::NONE);
        assert_eq!(second, first);
    }

    /// The measurement of a cell (an auto-height block measured with `ComputeSize`) agrees with its
    /// final layout at the same size, whether or not the measurement came from the cache
    #[test]
    fn compute_size_and_perform_layout_agree_on_the_baseline() {
        let mut tree = Tree::new();
        let leaf = tree.leaf(Style::default(), Size { width: 50.0, height: 50.0 }, Some(40.0));
        let cell = tree.block(padding_top(10.0), &[leaf]);

        tree.measure(cell, 50.0, AvailableSpace::MaxContent);
        let measured = tree.measure(cell, 50.0, AvailableSpace::MaxContent);
        let laid_out = tree.compute_child_layout(
            cell,
            LayoutInput {
                run_mode: RunMode::PerformLayout,
                sizing_mode: SizingMode::InherentSize,
                axis: RequestedAxis::Both,
                known_dimensions: Size { width: Some(50.0), height: Some(measured.size.height) },
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size: Size { width: Some(50.0), height: None },
                available_space: Size { width: AvailableSpace::Definite(50.0), height: AvailableSpace::MaxContent },
                vertical_margins_are_collapsible: Line::FALSE,
            },
        );

        assert_eq!(measured.baselines, laid_out.baselines);
    }
}
