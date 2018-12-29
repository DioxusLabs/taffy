#[cfg(test)]
mod measure {
    use stretch::number::OrElse;

    #[test]
    fn measure_root() {
        let layout = stretch::compute(&stretch::style::Node {
            measure: Some(&|constraint| stretch::geometry::Size {
                width: constraint.width.or_else(100.0),
                height: constraint.height.or_else(100.0),
            }),
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                measure: Some(&|constraint| stretch::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
            children: vec![stretch::style::Node {
                measure: Some(&|constraint| stretch::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0);
        assert_eq!(layout.size.height, 100.0);

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
            },
            children: vec![stretch::style::Node {
                measure: Some(&|constraint| stretch::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0);
        assert_eq!(layout.size.height, 120.0);

        assert_eq!(layout.children[0].size.width, 30.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0),
                        height: stretch::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    measure: Some(&|constraint| stretch::geometry::Size {
                        width: constraint.width.or_else(10.0),
                        height: constraint.height.or_else(50.0),
                    }),
                    flex_grow: 1.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0),
                        height: stretch::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                stretch::style::Node {
                    measure: Some(&|constraint| stretch::geometry::Size {
                        width: constraint.width.or_else(100.0),
                        height: constraint.height.or_else(50.0),
                    }),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
            align_items: stretch::style::AlignItems::FlexStart,
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0),
                        height: stretch::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    measure: Some(&|constraint| {
                        let width = constraint.width.or_else(10.0);
                        let height = constraint.height.or_else(width * 2.0);
                        stretch::geometry::Size { width, height }
                    }),
                    flex_grow: 1.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
            align_items: stretch::style::AlignItems::FlexStart,
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0),
                        height: stretch::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                stretch::style::Node {
                    measure: Some(&|constraint| {
                        let width = constraint.width.or_else(100.0);
                        let height = constraint.height.or_else(width * 2.0);
                        stretch::geometry::Size { width, height }
                    }),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
                measure: Some(&|constraint| stretch::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size { height: stretch::style::Dimension::Points(50.0), ..Default::default() },
                measure: Some(&|constraint| stretch::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0),
                height: stretch::style::Dimension::Points(100.0),
            },
            children: vec![
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    measure: Some(&|constraint| stretch::geometry::Size {
                        width: constraint.width.or_else(100.0),
                        height: constraint.height.or_else(100.0),
                    }),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 100.0);
        assert_eq!(layout.children[1].size.width, 100.0);
        assert_eq!(layout.children[1].size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
            },
            children: vec![stretch::style::Node {
                measure: Some(&|constraint| stretch::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                measure: Some(&|constraint| stretch::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 50.0);
    }
}
