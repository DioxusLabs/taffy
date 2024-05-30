#[cfg(test)]
mod measure {
    use taffy::prelude::*;

    #[derive(Debug, Clone, Copy)]
    struct FixedMeasure {
        width: f32,
        height: f32,
    }
    fn fixed_measure_function(
        known_dimensions: Size<Option<f32>>,
        _available_space: Size<AvailableSpace>,
        _node_id: NodeId,
        node_context: Option<&mut FixedMeasure>,
        _style: &Style,
    ) -> taffy::geometry::Size<f32> {
        let size = node_context.copied().unwrap_or(FixedMeasure { width: 0.0, height: 0.0 });
        Size {
            width: known_dimensions.width.unwrap_or(size.width),
            height: known_dimensions.height.unwrap_or(size.height),
        }
    }

    struct AspectRatioMeasure {
        width: f32,
        height_ratio: f32,
    }
    fn aspect_ratio_measure_function(
        known_dimensions: Size<Option<f32>>,
        _available_space: Size<AvailableSpace>,
        _node_id: NodeId,
        node_context: Option<&mut AspectRatioMeasure>,
        _style: &Style,
    ) -> taffy::geometry::Size<f32> {
        let Some(node_context) = node_context else { return Size::ZERO };
        let width = known_dimensions.width.unwrap_or(node_context.width);
        let height = known_dimensions.height.unwrap_or(width * node_context.height_ratio);
        Size { width, height }
    }

    #[test]
    fn measure_root() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let node = taffy.new_leaf_with_context(Style::default(), FixedMeasure { width: 100.0, height: 100.0 }).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();

        let child =
            taffy.new_leaf_with_context(Style::default(), FixedMeasure { width: 100.0, height: 100.0 }).unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child =
            taffy.new_leaf_with_context(Style::default(), FixedMeasure { width: 100.0, height: 100.0 }).unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(50.0), height: auto() }, ..Default::default() },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        // Parent
        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
        // Child
        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child =
            taffy.new_leaf_with_context(Style::default(), FixedMeasure { width: 100.0, height: 100.0 }).unwrap();

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
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

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
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_context(
                Style { flex_grow: 1.0, ..Default::default() },
                FixedMeasure { width: 10.0, height: 50.0 },
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 =
            taffy.new_leaf_with_context(Style::default(), FixedMeasure { width: 100.0, height: 50.0 }).unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let mut taffy: TaffyTree<AspectRatioMeasure> = TaffyTree::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_context(
                Style { flex_grow: 1.0, ..Default::default() },
                AspectRatioMeasure { width: 10.0, height_ratio: 2.0 },
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

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, aspect_ratio_measure_function).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let mut taffy: TaffyTree<AspectRatioMeasure> = TaffyTree::new();

        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_context(Style::default(), AspectRatioMeasure { width: 100.0, height_ratio: 2.0 })
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

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, aspect_ratio_measure_function).unwrap();

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
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child = taffy
            .new_leaf_with_context(
                Style { size: Size { width: Dimension::Length(50.0), height: auto() }, ..Default::default() },
                FixedMeasure { width: 100.0, height: 100.0 },
            )
            .unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child = taffy
            .new_leaf_with_context(
                Style { size: Size { width: auto(), height: Dimension::Length(50.0) }, ..Default::default() },
                FixedMeasure { width: 100.0, height: 100.0 },
            )
            .unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child0 = taffy
            .new_leaf(Style { flex_basis: Dimension::Length(50.0), flex_grow: 1.0, ..Default::default() })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_context(
                Style { flex_basis: Dimension::Length(50.0), flex_grow: 1.0, ..Default::default() },
                FixedMeasure { width: 100.0, height: 100.0 },
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

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child = taffy.new_leaf_with_context(Style::default(), FixedMeasure { width: 50.0, height: 50.0 }).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
        let child = taffy
            .new_leaf_with_context(
                Style { position: Position::Absolute, ..Default::default() },
                FixedMeasure { width: 50.0, height: 50.0 },
            )
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

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let mut taffy: TaffyTree<FixedMeasure> = TaffyTree::new();
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

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, fixed_measure_function).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }
}
