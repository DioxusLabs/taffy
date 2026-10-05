//! `fit-content()` with a `calc()` limit. Taffy stores the calc() as an opaque pointer which is resolved
//! through `LayoutPartialTree::resolve_calc_value`, so these tests use a minimal custom tree whose calc
//! values are `pct * basis + px`.
#[cfg(all(feature = "calc", feature = "grid", feature = "block_layout"))]
mod fit_content_calc {
    use taffy::{
        compute_block_layout, compute_cached_layout, compute_grid_layout, compute_leaf_layout, compute_root_layout,
        prelude::*, Cache, CacheTree, Layout, LayoutInput, LayoutOutput, Style,
    };

    /// A calc() value of the form `pct * basis + px`. 8-byte aligned so that its address can be tagged.
    #[repr(align(8))]
    struct Calc {
        px: f32,
        pct: f32,
    }

    fn calc(px: f32, pct: f32) -> LengthPercentage {
        let ptr: &'static Calc = Box::leak(Box::new(Calc { px, pct }));
        LengthPercentage::calc(ptr as *const Calc as *const ())
    }

    fn resolve_calc(val: *const (), basis: f32) -> f32 {
        // SAFETY: every calc pointer in this test is created by `calc` above and never freed
        let calc = unsafe { &*(val as *const Calc) };
        calc.pct * basis + calc.px
    }

    enum Kind {
        Block,
        Grid,
        /// A leaf with the given min-content width, max-content width and height
        Leaf {
            min_width: f32,
            max_width: f32,
            height: f32,
        },
    }

    struct Node {
        kind: Kind,
        style: Style,
        cache: Cache,
        layout: Layout,
        children: Vec<Node>,
        hoisted_children: Vec<NodeId>,
    }

    const ROOT: NodeId = NodeId::new(u64::MAX);

    impl Node {
        fn new(kind: Kind, style: Style, children: Vec<Node>) -> Node {
            Node {
                kind,
                style,
                cache: Cache::new(),
                layout: Layout::with_order(0),
                children,
                hoisted_children: Vec::new(),
            }
        }

        fn leaf(min_width: f32, max_width: f32, height: f32, style: Style) -> Node {
            Node::new(Kind::Leaf { min_width, max_width, height }, style, Vec::new())
        }

        fn node_from_id(&self, node_id: NodeId) -> &Node {
            if node_id == ROOT {
                self
            } else {
                &self.children[usize::from(node_id)]
            }
        }

        fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut Node {
            if node_id == ROOT {
                self
            } else {
                &mut self.children[usize::from(node_id)]
            }
        }

        fn compute_layout(&mut self, available_space: Size<AvailableSpace>) {
            compute_root_layout(self, ROOT, available_space);
        }
    }

    struct ChildIter(std::ops::Range<usize>);
    impl Iterator for ChildIter {
        type Item = NodeId;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(NodeId::from)
        }
    }

    impl taffy::TraversePartialTree for Node {
        type ChildIter<'a> = ChildIter;
        fn child_ids(&self, _node_id: NodeId) -> Self::ChildIter<'_> {
            ChildIter(0..self.children.len())
        }
        fn child_count(&self, _node_id: NodeId) -> usize {
            self.children.len()
        }
        fn get_child_id(&self, _node_id: NodeId, index: usize) -> NodeId {
            NodeId::from(index)
        }
    }

    impl taffy::LayoutPartialTree for Node {
        type CoreContainerStyle<'a>
            = &'a Style
        where
            Self: 'a;
        type CustomIdent = String;

        fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
            &self.node_from_id(node_id).style
        }
        fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
            self.node_from_id_mut(node_id).layout = *layout
        }
        fn resolve_calc_value(&self, val: *const (), basis: f32) -> f32 {
            resolve_calc(val, basis)
        }
        fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
            compute_cached_layout(self, node_id, inputs, |parent, node_id, inputs| {
                let node = parent.node_from_id_mut(node_id);
                match node.kind {
                    Kind::Block => compute_block_layout(node, node_id, inputs, None),
                    Kind::Grid => compute_grid_layout(node, node_id, inputs),
                    Kind::Leaf { min_width, max_width, height } => {
                        compute_leaf_layout(inputs, &node.style, resolve_calc, |known_dimensions, available_space| {
                            let width = known_dimensions.width.unwrap_or(match available_space.width {
                                AvailableSpace::Definite(width) => width.clamp(min_width, max_width),
                                AvailableSpace::MinContent => min_width,
                                AvailableSpace::MaxContent => max_width,
                            });
                            Size { width, height: known_dimensions.height.unwrap_or(height) }
                        })
                    }
                }
            })
        }
    }

    impl taffy::LayoutContainingBlock for Node {
        type OofItemStyle<'a>
            = &'a Style
        where
            Self: 'a;
        fn get_oof_item_style(&self, node_id: NodeId) -> Self::OofItemStyle<'_> {
            &self.node_from_id(node_id).style
        }
        fn clear_hoisted_children(&mut self, node_id: NodeId) {
            self.node_from_id_mut(node_id).hoisted_children.clear();
        }
        fn add_hoisted_children(&mut self, node_id: NodeId, hoisted: &[NodeId]) {
            self.node_from_id_mut(node_id).hoisted_children.extend_from_slice(hoisted);
        }
    }

    impl CacheTree for Node {
        fn cache_get(&mut self, node_id: NodeId, inputs: &LayoutInput) -> Option<LayoutOutput> {
            self.node_from_id_mut(node_id).cache.get(inputs)
        }
        fn cache_store(&mut self, node_id: NodeId, inputs: &LayoutInput, layout_output: LayoutOutput) {
            self.node_from_id_mut(node_id).cache.store(inputs, layout_output)
        }
        fn cache_clear(&mut self, node_id: NodeId) {
            self.node_from_id_mut(node_id).cache.clear();
        }
    }

    impl taffy::LayoutBlockContainer for Node {
        type BlockContainerStyle<'a>
            = &'a Style
        where
            Self: 'a;
        type BlockItemStyle<'a>
            = &'a Style
        where
            Self: 'a;
        fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_> {
            &self.node_from_id(node_id).style
        }
        fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_> {
            &self.node_from_id(child_node_id).style
        }
    }

    impl taffy::LayoutGridContainer for Node {
        type GridContainerStyle<'a>
            = &'a Style
        where
            Self: 'a;
        type GridItemStyle<'a>
            = &'a Style
        where
            Self: 'a;
        fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_> {
            &self.node_from_id(node_id).style
        }
        fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_> {
            &self.node_from_id(child_node_id).style
        }
    }

    /// Mirrors WPT css/css-grid/grid-definition/fit-content-track-sizing.html (`calculatedArgument`):
    /// a 300px tall grid with `grid-template-rows: fit-content(calc(10% + 5px))` and a 400px tall item
    /// with `min-height: 0` sizes the row to 35px.
    #[test]
    fn grid_track_fit_content_calc_limit() {
        let mut root = Node::new(
            Kind::Grid,
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(300.0) },
                grid_template_rows: vec![fit_content(calc(5.0, 0.1))],
                ..Default::default()
            },
            vec![Node::leaf(
                0.0,
                0.0,
                400.0,
                Style { min_size: Size { width: auto(), height: length(0.0) }, ..Default::default() },
            )],
        );
        root.compute_layout(Size::MAX_CONTENT);
        assert_eq!(root.children[0].layout.size.height, 35.0);
    }

    /// `width: fit-content(calc(10% + 70px))` on a block-level box in a 300px wide container is
    /// `fit-content(100px)`: the max-content width clamped by the limit and floored by the min-content width.
    #[test]
    fn block_item_fit_content_calc_width() {
        let limit = Dimension::fit_content_calc(calc(70.0, 0.1).into_raw().calc_value());
        let mut root = Node::new(
            Kind::Block,
            Style {
                display: Display::Block,
                size: Size { width: length(300.0), height: auto() },
                ..Default::default()
            },
            vec![Node::leaf(
                50.0,
                400.0,
                10.0,
                Style { size: Size { width: limit, height: auto() }, ..Default::default() },
            )],
        );
        root.compute_layout(Size::MAX_CONTENT);
        assert_eq!(root.children[0].layout.size.width, 100.0);
    }

    /// Returns (container width, item width) for a single-column grid with the given column sizing
    fn layout_single_column_grid(
        column: GridTemplateComponent<String>,
        container_width: Dimension,
        available_width: AvailableSpace,
    ) -> (f32, f32) {
        let mut root = Node::new(
            Kind::Grid,
            Style {
                display: Display::Grid,
                size: Size { width: container_width, height: length(40.0) },
                grid_template_columns: vec![column],
                ..Default::default()
            },
            vec![Node::leaf(50.0, 150.0, 10.0, Style::default())],
        );
        root.compute_layout(Size { width: available_width, height: AvailableSpace::MaxContent });
        (root.layout.size.width, root.children[0].layout.size.width)
    }

    /// A calc() limit resolves to the same value as the equivalent length or percentage limit, including
    /// when the container's width is indefinite (which re-runs track sizing against the container's own width).
    #[test]
    fn grid_track_fit_content_calc_matches_equivalent_length_and_percentage() {
        // Definite container: calc(20% + 10px) of 200px == 50px
        let calc_result =
            layout_single_column_grid(fit_content(calc(10.0, 0.2)), length(200.0), AvailableSpace::MaxContent);
        let px_result = layout_single_column_grid(fit_content(length(50.0)), length(200.0), AvailableSpace::MaxContent);
        assert_eq!(calc_result, px_result);
        assert_eq!(calc_result.1, 50.0);

        // Indefinite (shrink-to-fit) container: calc(50% + 0px) == 50%
        for available_width in [AvailableSpace::Definite(200.0), AvailableSpace::MaxContent, AvailableSpace::MinContent]
        {
            let calc_result = layout_single_column_grid(fit_content(calc(0.0, 0.5)), auto(), available_width);
            let pct_result = layout_single_column_grid(fit_content(percent(0.5)), auto(), available_width);
            assert_eq!(calc_result, pct_result, "available width: {available_width:?}");
        }
        // ... and the percentage really is resolved against the container's width
        let (container_width, item_width) =
            layout_single_column_grid(fit_content(calc(0.0, 0.5)), auto(), AvailableSpace::MaxContent);
        assert_eq!(container_width, 150.0);
        assert_eq!(item_width, 75.0);
    }
}
