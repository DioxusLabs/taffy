#[cfg(test)]
mod measure {
    use taffy::node::Measure;
    use taffy::node::Node;
    use taffy::node::Taffy;
    use taffy::prelude::Size;
    use taffy::style::FlexboxLayout;

    #[test]
    fn measure_root() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

        let mut taffy = Taffy::new();

        let node = taffy.new_leaf_with_measure(FlexboxLayout::default(), (100., 100.)).unwrap();

        taffy.compute_measured_layout(node, &measure, Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

        let mut taffy = taffy::node::Taffy::new();

        let child =
            taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, (100., 100.)).unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }
        let measure = ConstraintOrStored();
        let mut taffy = taffy::node::Taffy::new();
        let child =
            taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, (100., 100.)).unwrap();

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
        struct ConstraintOrStored();

        impl Measure<i32> for ConstraintOrStored {
            fn measure(&self, _node: Node, _data: &i32, constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(100.0), height: constraint.height.unwrap_or(100.0) }
            }
        }

        let measure = ConstraintOrStored();
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, 1).unwrap();

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
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 120.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 30.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

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
            .new_leaf_with_measure(taffy::style::FlexboxLayout { flex_grow: 1.0, ..Default::default() }, (10., 50.))
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

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

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
            taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, (100., 50.)).unwrap();

        let node = taffy
            .new_with_children(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        struct HeightIsTwiceWidth();

        impl Measure<f32> for HeightIsTwiceWidth {
            fn measure(&self, _node: Node, size: &f32, constraint: Size<Option<f32>>) -> Size<f32> {
                let width = constraint.width.unwrap_or(*size);
                let height = constraint.height.unwrap_or(width * 2.0);
                taffy::geometry::Size { width, height }
            }
        }

        let measure = HeightIsTwiceWidth();

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
            .new_leaf_with_measure(taffy::style::FlexboxLayout { flex_grow: 1.0, ..Default::default() }, 10.)
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

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        struct HeightIsTwiceWidth();

        impl Measure<f32> for HeightIsTwiceWidth {
            fn measure(&self, _node: Node, size: &f32, constraint: Size<Option<f32>>) -> Size<f32> {
                let width = constraint.width.unwrap_or(*size);
                let height = constraint.height.unwrap_or(width * 2.0);
                taffy::geometry::Size { width, height }
            }
        }

        let measure = HeightIsTwiceWidth();

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

        let child1 = taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, 100.).unwrap();

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

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        struct WidthEqualsHeight();

        impl Measure<f32> for WidthEqualsHeight {
            fn measure(&self, _node: Node, size: &f32, constraint: Size<Option<f32>>) -> Size<f32> {
                let height = constraint.height.unwrap_or(*size);
                let width = constraint.width.unwrap_or(height);
                taffy::geometry::Size { width, height }
            }
        }

        let measure = WidthEqualsHeight();

        let mut taffy = taffy::node::Taffy::new();

        let child = taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, 50.).unwrap();

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

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    ..Default::default()
                },
                (100., 100.),
            )
            .unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                taffy::style::FlexboxLayout {
                    size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    ..Default::default()
                },
                (100., 100.),
            )
            .unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                flex_basis: taffy::style::Dimension::Points(50.0),
                flex_grow: 1.0,
                ..Default::default()
            })
            .unwrap();

        let child1 = taffy
            .new_leaf_with_measure(
                taffy::style::FlexboxLayout {
                    flex_basis: taffy::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                (100., 100.),
            )
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

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

        let mut taffy = taffy::node::Taffy::new();
        let child =
            taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, (50., 50.)).unwrap();

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

        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        struct ConstraintOrStored();

        impl Measure<(f32, f32)> for ConstraintOrStored {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = ConstraintOrStored();

        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf_with_measure(
                taffy::style::FlexboxLayout {
                    position_type: taffy::style::PositionType::Absolute,
                    ..Default::default()
                },
                (50., 50.),
            )
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
            num_measures: atomic::AtomicU32,
        }

        impl Measure<(f32, f32)> for CountingMeasure {
            fn measure(&self, _node: Node, (w, h): &(f32, f32), constraint: Size<Option<f32>>) -> Size<f32> {
                self.num_measures.fetch_add(1, atomic::Ordering::Relaxed);
                Size { width: constraint.width.unwrap_or(*w), height: constraint.height.unwrap_or(*h) }
            }
        }

        let measure = CountingMeasure { num_measures: atomic::AtomicU32::new(0) };

        let mut taffy = taffy::node::Taffy::new();

        let grandchild =
            taffy.new_leaf_with_measure(taffy::style::FlexboxLayout { ..Default::default() }, (50., 50.)).unwrap();

        let child =
            taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[grandchild]).unwrap();

        let node = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[child]).unwrap();
        taffy.compute_measured_layout(node, &measure, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(measure.num_measures.load(atomic::Ordering::Relaxed), 2);
    }
}
