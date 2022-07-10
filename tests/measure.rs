#[cfg(test)]
mod measure {
    use taffy::node::Measure;
    use taffy::node::Node;
    use taffy::node::Taffy;
    use taffy::prelude::Size;
    use taffy::style::FlexboxLayout;

    struct ReturnConstraintOrStored(f32, f32);

    impl Measure for ReturnConstraintOrStored {
        fn measure(&self, _node: Node, constraint: Size<Option<f32>>) -> Size<f32> {
            let ReturnConstraintOrStored(width, height) = self;
            Size { width: constraint.width.unwrap_or(*width), height: constraint.height.unwrap_or(*height) }
        }
    }

    #[test]
    fn measure_root() {
        let mut taffy = Taffy::new();

        let node = taffy.new_leaf_with_required_measure(FlexboxLayout::default()).unwrap();

        let measure = ReturnConstraintOrStored(100., 100.);
        taffy.compute_measured_layout(node, &measure, Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let mut taffy = taffy::node::Taffy::new();

        let child = taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();

        let measure = ReturnConstraintOrStored(100., 100.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let measure = ReturnConstraintOrStored(100., 100.);
        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    padding: taffy::geometry::Rect {
                        start: taffy::style::Dimension::Points(10.0),
                        end: taffy::style::Dimension::Points(10.0),
                        top: taffy::style::Dimension::Points(10.0),
                        bottom: taffy::style::Dimension::Points(10.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        let measure = ReturnConstraintOrStored(100., 100.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 120.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 30.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50.0),
                    height: taffy::style::Dimension::Points(50.0),
                },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_required_measure(taffy::style::FlexboxLayout { flex_grow: 1.0, ..Default::default() })
            .unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        let measure = ReturnConstraintOrStored(10., 50.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50.0),
                    height: taffy::style::Dimension::Points(50.0),
                },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 =
            taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        let measure = ReturnConstraintOrStored(100., 50.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        struct HeightIsTwiceWidth(f32);

        impl Measure for HeightIsTwiceWidth {
            fn measure(&self, _node: Node, constraint: Size<Option<f32>>) -> Size<f32> {
                let HeightIsTwiceWidth(size) = self;
                let width = constraint.width.unwrap_or(*size);
                let height = constraint.height.unwrap_or(width * 2.0);
                taffy::geometry::Size { width, height }
            }
        }

        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50.0),
                    height: taffy::style::Dimension::Points(50.0),
                },
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_required_measure(taffy::style::FlexboxLayout { flex_grow: 1.0, ..Default::default() })
            .unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    align_items: taffy::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        let measure = HeightIsTwiceWidth(10.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        struct HeightIsTwiceWidth(f32);

        impl Measure for HeightIsTwiceWidth {
            fn measure(&self, _node: Node, constraint: Size<Option<f32>>) -> Size<f32> {
                let HeightIsTwiceWidth(size) = self;
                let width = constraint.width.unwrap_or(*size);
                let height = constraint.height.unwrap_or(width * 2.0);
                taffy::geometry::Size { width, height }
            }
        }

        let mut taffy = taffy::node::Taffy::new();

        let child0 = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50.0),
                    height: taffy::style::Dimension::Points(50.0),
                },
                flex_shrink: 0.0,
                ..Default::default()
            })
            .unwrap();

        let child1 =
            taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    align_items: taffy::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        let measure = HeightIsTwiceWidth(100.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        struct WidthEqualsHeight(f32);

        impl Measure for WidthEqualsHeight {
            fn measure(&self, _node: Node, constraint: Size<Option<f32>>) -> Size<f32> {
                let WidthEqualsHeight(size) = self;
                let height = constraint.height.unwrap_or(*size);
                let width = constraint.width.unwrap_or(height);
                taffy::geometry::Size { width, height }
            }
        }

        let mut taffy = taffy::node::Taffy::new();

        let child = taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        let measure = WidthEqualsHeight(50.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf_with_required_measure(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50.0), ..Default::default() },
                ..Default::default()
            })
            .unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();

        let measure = ReturnConstraintOrStored(100., 100.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf_with_required_measure(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50.0), ..Default::default() },
                ..Default::default()
            })
            .unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();

        let measure = ReturnConstraintOrStored(100., 100.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                flex_basis: taffy::style::Dimension::Points(50.0),
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_required_measure(taffy::style::FlexboxLayout {
                flex_basis: taffy::style::Dimension::Points(50.0),
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(200.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        let measure = ReturnConstraintOrStored(100., 100.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        let measure = ReturnConstraintOrStored(50., 50.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf_with_required_measure(taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                ..Default::default()
            })
            .unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        let measure = ReturnConstraintOrStored(50., 50.);
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy.new_leaf(taffy::style::FlexboxLayout { flex_grow: 1.0, ..Default::default() }).unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn only_measure_once() {
        use std::sync::atomic;
        struct CountingMeasure {
            width: f32,
            height: f32,
            num_measures: atomic::AtomicU32,
        }

        impl Measure for CountingMeasure {
            fn measure(&self, _node: Node, constraint: Size<Option<f32>>) -> Size<f32> {
                self.num_measures.fetch_add(1, atomic::Ordering::Relaxed);
                Size { width: constraint.width.unwrap_or(self.width), height: constraint.height.unwrap_or(self.height) }
            }
        }

        let measure = CountingMeasure { width: 50., height: 50., num_measures: atomic::AtomicU32::new(0) };

        let mut taffy = taffy::node::Taffy::new();

        let grandchild =
            taffy.new_leaf_with_required_measure(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        let child =
            taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[grandchild]).unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(measure.num_measures.load(atomic::Ordering::Relaxed), 2);
    }
}
