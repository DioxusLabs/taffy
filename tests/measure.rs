#[cfg(test)]
mod measure {
    use taffy::prelude::*;
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
                Style { size: Size { width: Dimension::Length(50.0), height: auto() }, ..Default::default() },
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
                    size: Size { width: Dimension::Length(50.0), height: auto() },
                    padding: Rect {
                        left: LengthPercentage::Length(10.0),
                        right: LengthPercentage::Length(10.0),
                        top: LengthPercentage::Length(10.0),
                        bottom: LengthPercentage::Length(10.0),
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
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy.new_leaf_with_context(Style { flex_grow: 1.0, ..Default::default() }, FIFTY_FIFTY).unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(100.0), height: auto() }, ..Default::default() },
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
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy.new_leaf_with_context(Style::default(), HUNDRED_FIFTY).unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(100.0), height: auto() }, ..Default::default() },
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
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
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
                    size: Size { width: Dimension::Length(100.0), height: auto() },
                    align_items: Some(AlignItems::Start),
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
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy.new_leaf_with_context(Style::default(), TestNodeContext::aspect_ratio(100.0, 2.0)).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Length(100.0), height: auto() },
                    align_items: Some(AlignItems::Start),
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
            known_dimensions: Size<Option<f32>>,
            _available_space: Size<AvailableSpace>,
            _node_id: NodeId,
            _node_context: Option<&mut ()>,
            _style: &Style,
        ) -> taffy::geometry::Size<f32> {
            let height = known_dimensions.height.unwrap_or(50.0);
            let width = known_dimensions.width.unwrap_or(height);
            Size { width, height }
        }

        let child = taffy.new_leaf_with_context(Style::default(), ()).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
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
                Style { size: Size { width: Dimension::Length(50.0), height: auto() }, ..Default::default() },
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
                Style { size: Size { width: auto(), height: Dimension::Length(50.0) }, ..Default::default() },
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
            .new_leaf(Style { flex_basis: Dimension::Length(50.0), flex_grow: 1.0, ..Default::default() })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_context(
                Style { flex_basis: Dimension::Length(50.0), flex_grow: 1.0, ..Default::default() },
                HUNDRED_HUNDRED,
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Length(200.0), height: Dimension::Length(100.0) },
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
                    size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
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
    fn measure_absolute_child() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(Style { position: Position::Absolute, ..Default::default() }, FIFTY_FIFTY)
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
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
                    size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }
}
