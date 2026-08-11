#[cfg(test)]
mod measure {
    use taffy::prelude::*;
    use taffy::{LayoutInput, LayoutOutput};
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    const HUNDRED_HUNDRED: TestNodeContext = TestNodeContext::fixed(100.0, 100.0);
    const HUNDRED_FIFTY: TestNodeContext = TestNodeContext::fixed(100.0, 50.0);
    const FIFTY_FIFTY: TestNodeContext = TestNodeContext::fixed(50.0, 50.0);

    #[test]
    fn measure_root() {
        let mut taffy = new_test_tree();
        let node = taffy.new_leaf_with_context(Style::default(), HUNDRED_HUNDRED).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let mut taffy = new_test_tree();

        let child = taffy.new_leaf_with_context(Style::default(), HUNDRED_HUNDRED).unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut taffy = new_test_tree();
        let child = taffy.new_leaf_with_context(Style::default(), HUNDRED_HUNDRED).unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::from_length(50.0), height: auto() }, ..Default::default() },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        // Parent
        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
        // Child
        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut taffy = new_test_tree();
        let child = taffy.new_leaf_with_context(Style::default(), HUNDRED_HUNDRED).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(50.0), height: auto() },
                    padding: Rect {
                        left: LengthPercentage::from_length(10.0),
                        right: LengthPercentage::from_length(10.0),
                        top: LengthPercentage::from_length(10.0),
                        bottom: LengthPercentage::from_length(10.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(node).unwrap().location.x, 0.0);
        assert_eq!(taffy.layout(node).unwrap().location.y, 0.0);
        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 120.0);

        assert_eq!(taffy.layout(child).unwrap().location.x, 10.0);
        assert_eq!(taffy.layout(child).unwrap().location.y, 10.0);
        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let mut taffy = new_test_tree();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy.new_leaf_with_context(Style { flex_grow: 1.0, ..Default::default() }, FIFTY_FIFTY).unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::from_length(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut taffy = new_test_tree();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy.new_leaf_with_context(Style::default(), HUNDRED_FIFTY).unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::from_length(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let mut taffy = new_test_tree();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_context(
                Style { flex_grow: 1.0, ..Default::default() },
                TestNodeContext::aspect_ratio(10.0, 2.0),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(100.0), height: auto() },
                    align_items: Some(AlignItems::START),
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let mut taffy = new_test_tree();

        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy.new_leaf_with_context(Style::default(), TestNodeContext::aspect_ratio(100.0, 2.0)).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(100.0), height: auto() },
                    align_items: Some(AlignItems::START),
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 200.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();

        fn custom_measure_function(
            inputs: LayoutInput,
            _node_id: NodeId,
            _node_context: Option<&mut ()>,
            style: &Style,
        ) -> LayoutOutput {
            taffy::compute_leaf_layout(
                inputs,
                style,
                |_, _| 0.0,
                |known_dimensions, _available_space| {
                    let height = known_dimensions.height.unwrap_or(50.0);
                    let width = known_dimensions.width.unwrap_or(height);
                    Size { width, height }
                },
            )
        }

        let child = taffy.new_leaf_with_context(Style::default(), ()).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, custom_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(
                Style { size: Size { width: Dimension::from_length(50.0), height: auto() }, ..Default::default() },
                HUNDRED_HUNDRED,
            )
            .unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(
                Style { size: Size { width: auto(), height: Dimension::from_length(50.0) }, ..Default::default() },
                HUNDRED_HUNDRED,
            )
            .unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut taffy = new_test_tree();
        let child0 = taffy
            .new_leaf(Style { flex_basis: Dimension::from_length(50.0), flex_grow: 1.0, ..Default::default() })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_context(
                Style { flex_basis: Dimension::from_length(50.0), flex_grow: 1.0, ..Default::default() },
                HUNDRED_HUNDRED,
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(200.0), height: Dimension::from_length(100.0) },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let mut taffy = new_test_tree();
        let child = taffy.new_leaf_with_context(Style::default(), FIFTY_FIFTY).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn explicit_min_main_size_skips_min_content_measurement() {
        fn assert_not_min_content(
            inputs: LayoutInput,
            _node_id: NodeId,
            _context: Option<&mut ()>,
            _style: &Style,
        ) -> LayoutOutput {
            assert_ne!(inputs.available_space.width, AvailableSpace::MinContent);
            LayoutOutput::from_outer_size(Size { width: 10.0, height: 10.0 })
        }

        let mut taffy = TaffyTree::new();
        let child = taffy
            .new_leaf_with_context(
                Style { min_size: Size { width: length(10.0), height: auto() }, ..Default::default() },
                (),
            )
            .unwrap();
        let root = taffy.new_with_children(Style::default(), &[child]).unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, assert_not_min_content).unwrap();
    }

    #[test]
    fn measure_absolute_child() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(Style { position: Position::Absolute, ..Default::default() }, FIFTY_FIFTY)
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let mut taffy = new_test_tree();
        let child = taffy.new_leaf(Style { flex_grow: 1.0, ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    /// A flex child whose style width resolves wider than its own max_width
    /// must have its content measured at the clamped (used) width. The text
    /// wraps to 3 lines at the used width (200px) but only 2 at the unclamped
    /// style width (300px); the stale 2-line height used to leak into the
    /// child's flex base size, leaving the child (and its ancestors) too
    /// short while the text inside is laid out taller.
    /// Regression test for the measure-function consequence of the clamp
    /// added in #989.
    #[test]
    fn measure_child_with_percent_width_clamped_by_max_width() {
        let mut taffy = new_test_tree();

        // Ahem text: twelve 40px segments; wraps to 2 lines at 300px
        // (7 + 5 segments), 3 lines at 200px (5 + 5 + 2).
        let text = ["HHHH"; 12].join("\u{200B}");
        let text_node = taffy
            .new_leaf_with_context(
                Style::default(),
                TestNodeContext::ahem_text(text, taffy_test_helpers::WritingMode::Horizontal),
            )
            .unwrap();

        // width: 100% of a 300px parent, clamped by max-width: 200px.
        let column = taffy
            .new_with_children(
                Style {
                    flex_direction: FlexDirection::Column,
                    size: Size { width: percent(1.0), height: auto() },
                    max_size: Size { width: Dimension::from_length(200.0), height: auto() },
                    ..Default::default()
                },
                &[text_node],
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    flex_direction: FlexDirection::Column,
                    size: Size { width: Dimension::from_length(300.0), height: auto() },
                    ..Default::default()
                },
                &[column],
            )
            .unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();

        // The column is 200 wide, so the text wraps to 3 lines of 10px.
        assert_eq!(taffy.layout(text_node).unwrap().size.width, 200.0);
        assert_eq!(taffy.layout(text_node).unwrap().size.height, 30.0);
        // The column must be tall enough to contain the wrapped text.
        assert_eq!(taffy.layout(column).unwrap().size.width, 200.0);
        assert_eq!(taffy.layout(column).unwrap().size.height, 30.0);
    }

    /// Same as above, but the container's main size is determined under a
    /// max-content constraint (the intrinsic-sizing path measures children
    /// with their own known dimensions too).
    #[test]
    fn measure_child_with_fixed_width_clamped_by_max_width_intrinsic() {
        let mut taffy = new_test_tree();

        let text = ["HHHH"; 12].join("\u{200B}");
        let text_node = taffy
            .new_leaf_with_context(
                Style::default(),
                TestNodeContext::ahem_text(text, taffy_test_helpers::WritingMode::Horizontal),
            )
            .unwrap();

        // width: 300px, clamped by max-width: 200px.
        let column = taffy
            .new_with_children(
                Style {
                    flex_direction: FlexDirection::Column,
                    size: Size { width: Dimension::from_length(300.0), height: auto() },
                    max_size: Size { width: Dimension::from_length(200.0), height: auto() },
                    ..Default::default()
                },
                &[text_node],
            )
            .unwrap();

        // Root is content-sized: its height comes from the intrinsic path.
        let root = taffy
            .new_with_children(
                Style { flex_direction: FlexDirection::Column, ..Default::default() },
                &[column],
            )
            .unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(text_node).unwrap().size.height, 30.0);
        assert_eq!(taffy.layout(column).unwrap().size.height, 30.0);
        assert_eq!(taffy.layout(root).unwrap().size.height, 30.0);
    }
}
