#[cfg(test)]
mod measure {
    use taffy::node::MeasureFunc;
    use taffy::prelude::*;

    #[test]
    fn measure_root() {
        let mut taffy = Taffy::new();
        let node = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
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
        let mut taffy = Taffy::new();

        let child = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_with_children(Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut taffy = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Points(50.0), height: auto() }, ..Default::default() },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut taffy = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(50.0), height: auto() },
                    padding: Rect {
                        left: LengthPercentage::Points(10.0),
                        right: LengthPercentage::Points(10.0),
                        top: LengthPercentage::Points(10.0),
                        bottom: LengthPercentage::Points(10.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 120.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 30.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let mut taffy = Taffy::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Points(50.0), height: Dimension::Points(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(10.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Points(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut taffy = Taffy::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Points(50.0), height: Dimension::Points(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style { size: Size { width: Dimension::Points(100.0), height: auto() }, ..Default::default() },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let mut taffy = Taffy::new();
        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Points(50.0), height: Dimension::Points(50.0) },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| {
                    let width = known_dimensions.width.unwrap_or(10.0);
                    let height = known_dimensions.height.unwrap_or(width * 2.0);
                    Size { width, height }
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(100.0), height: auto() },
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
        let mut taffy = Taffy::new();

        let child0 = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Points(50.0), height: Dimension::Points(50.0) },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| {
                    let width = known_dimensions.width.unwrap_or(100.0);
                    let height = known_dimensions.height.unwrap_or(width * 2.0);
                    Size { width, height }
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(100.0), height: auto() },
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
    fn remeasure_child_after_stretching() {
        let mut taffy = Taffy::new();

        let child = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| {
                    let height = known_dimensions.height.unwrap_or(50.0);
                    let width = known_dimensions.width.unwrap_or(height);
                    Size { width, height }
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
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
        let mut taffy = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { size: Size { width: Dimension::Points(50.0), height: auto() }, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_with_children(Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut taffy = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { size: Size { width: auto(), height: Dimension::Points(50.0) }, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_with_children(Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut taffy = Taffy::new();
        let child0 = taffy
            .new_leaf(Style { flex_basis: Dimension::Points(50.0), flex_grow: 1.0, ..Default::default() })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                Style { flex_basis: Dimension::Points(50.0), flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(100.0),
                    height: known_dimensions.height.unwrap_or(100.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(200.0), height: Dimension::Points(100.0) },
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
        let mut taffy = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(50.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
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
        let mut taffy = Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                Style { position_type: PositionType::Absolute, ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| Size {
                    width: known_dimensions.width.unwrap_or(50.0),
                    height: known_dimensions.height.unwrap_or(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
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
        let mut taffy = Taffy::new();
        let child = taffy.new_leaf(Style { flex_grow: 1.0, ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                Style {
                    size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
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
    fn only_measure_once() {
        use std::sync::atomic;

        let mut taffy = Taffy::new();
        static NUM_MEASURES: atomic::AtomicU32 = atomic::AtomicU32::new(0);

        let grandchild = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| {
                    NUM_MEASURES.fetch_add(1, atomic::Ordering::SeqCst);
                    Size {
                        width: known_dimensions.width.unwrap_or(50.0),
                        height: known_dimensions.height.unwrap_or(50.0),
                    }
                }),
            )
            .unwrap();

        let child = taffy.new_with_children(Style { ..Default::default() }, &[grandchild]).unwrap();

        let node = taffy.new_with_children(Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(NUM_MEASURES.load(atomic::Ordering::SeqCst), 1);
    }
}
