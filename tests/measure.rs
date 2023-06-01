#[cfg(test)]
mod measure {
    use taffy::prelude::*;
    use taffy::tree::MeasureFunc;

    #[test]
    fn measure_root() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let node = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();

        let child = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(50.0), height: auto() }, ..Default::default() },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        // Parent
        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
        // Child
        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

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
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

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
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(10.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Length(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| {
                    let width = known_dimensions.width.unwrap_or(10.0);
                    let height = known_dimensions.height.unwrap_or(width * 2.0);
                    Size { width, height }
                }),
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

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();

        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| {
                    let width = known_dimensions.width.unwrap_or(100.0);
                    let height = known_dimensions.height.unwrap_or(width * 2.0);
                    Size { width, height }
                }),
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

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 200.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();

        let child = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| {
                    let height = known_dimensions.height.unwrap_or(50.0);
                    let width = known_dimensions.width.unwrap_or(height);
                    Size { width, height }
                }),
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

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { size: Size { width: Dimension::Length(50.0), height: auto() }, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { size: Size { width: auto(), height: Dimension::Length(50.0) }, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_with_children(Style::default(), &[child]).unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child0 = taffy
            .new_leaf(Style { flex_basis: Dimension::Length(50.0), flex_grow: 1.0, ..Default::default() })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { flex_basis: Dimension::Length(50.0), flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
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

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style::default(),
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(50.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
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

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { position: Position::Absolute, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space, _context| Size {
                    width: known_dimensions.width.unwrap_or(50.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
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

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
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

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }
}
