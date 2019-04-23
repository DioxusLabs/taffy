#[cfg(test)]
mod measure {
    use stretch::number::OrElse;

    #[test]
    fn measure_root() {
        let layout = stretch::node::Node::new_leaf(
            stretch::style::Style { ..Default::default() },
            Box::new(|constraint| {
                Ok(stretch::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                })
            }),
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let layout = stretch::node::Node::new(
            stretch::style::Style { ..Default::default() },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { ..Default::default() },
                Box::new(|constraint| {
                    Ok(stretch::geometry::Size {
                        width: constraint.width.or_else(100.0),
                        height: constraint.height.or_else(100.0),
                    })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
                ..Default::default()
            },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { ..Default::default() },
                Box::new(|constraint| {
                    Ok(stretch::geometry::Size {
                        width: constraint.width.or_else(100.0),
                        height: constraint.height.or_else(100.0),
                    })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.size.width, 50.0);
        assert_eq!(layout.size.height, 100.0);

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0),
                    end: stretch::style::Dimension::Points(10.0),
                    top: stretch::style::Dimension::Points(10.0),
                    bottom: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { ..Default::default() },
                Box::new(|constraint| {
                    Ok(stretch::geometry::Size {
                        width: constraint.width.or_else(100.0),
                        height: constraint.height.or_else(100.0),
                    })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.size.width, 50.0);
        assert_eq!(layout.size.height, 120.0);

        assert_eq!(layout.children[0].size.width, 30.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0),
                            height: stretch::style::Dimension::Points(50.0),
                        },
                        ..Default::default()
                    },
                    vec![],
                ),
                &stretch::node::Node::new_leaf(
                    stretch::style::Style { flex_grow: 1.0, ..Default::default() },
                    Box::new(|constraint| {
                        Ok(stretch::geometry::Size {
                            width: constraint.width.or_else(10.0),
                            height: constraint.height.or_else(50.0),
                        })
                    }),
                ),
            ],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0),
                            height: stretch::style::Dimension::Points(50.0),
                        },
                        flex_shrink: 0.0,
                        ..Default::default()
                    },
                    vec![],
                ),
                &stretch::node::Node::new_leaf(
                    stretch::style::Style { ..Default::default() },
                    Box::new(|constraint| {
                        Ok(stretch::geometry::Size {
                            width: constraint.width.or_else(100.0),
                            height: constraint.height.or_else(50.0),
                        })
                    }),
                ),
            ],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
                align_items: stretch::style::AlignItems::FlexStart,
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0),
                            height: stretch::style::Dimension::Points(50.0),
                        },
                        ..Default::default()
                    },
                    vec![],
                ),
                &stretch::node::Node::new_leaf(
                    stretch::style::Style { flex_grow: 1.0, ..Default::default() },
                    Box::new(|constraint| {
                        let width = constraint.width.or_else(10.0);
                        let height = constraint.height.or_else(width * 2.0);
                        Ok(stretch::geometry::Size { width, height })
                    }),
                ),
            ],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
                align_items: stretch::style::AlignItems::FlexStart,
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0),
                            height: stretch::style::Dimension::Points(50.0),
                        },
                        flex_shrink: 0.0,
                        ..Default::default()
                    },
                    vec![],
                ),
                &stretch::node::Node::new_leaf(
                    stretch::style::Style { ..Default::default() },
                    Box::new(|constraint| {
                        let width = constraint.width.or_else(100.0);
                        let height = constraint.height.or_else(width * 2.0);
                        Ok(stretch::geometry::Size { width, height })
                    }),
                ),
            ],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[1].size.width, 50.0);
        assert_eq!(layout.children[1].size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                },
                ..Default::default()
            },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { ..Default::default() },
                Box::new(|constraint| {
                    let height = constraint.height.or_else(50.0);
                    let width = constraint.width.or_else(height);
                    Ok(stretch::geometry::Size { width, height })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let layout = stretch::node::Node::new(
            stretch::style::Style { ..Default::default() },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Box::new(|constraint| {
                    Ok(stretch::geometry::Size {
                        width: constraint.width.or_else(100.0),
                        height: constraint.height.or_else(100.0),
                    })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let layout = stretch::node::Node::new(
            stretch::style::Style { ..Default::default() },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Box::new(|constraint| {
                    Ok(stretch::geometry::Size {
                        width: constraint.width.or_else(100.0),
                        height: constraint.height.or_else(100.0),
                    })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200.0),
                    height: stretch::style::Dimension::Points(100.0),
                },
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        flex_basis: stretch::style::Dimension::Points(50.0),
                        flex_grow: 1.0,
                        ..Default::default()
                    },
                    vec![],
                ),
                &stretch::node::Node::new_leaf(
                    stretch::style::Style {
                        flex_basis: stretch::style::Dimension::Points(50.0),
                        flex_grow: 1.0,
                        ..Default::default()
                    },
                    Box::new(|constraint| {
                        Ok(stretch::geometry::Size {
                            width: constraint.width.or_else(100.0),
                            height: constraint.height.or_else(100.0),
                        })
                    }),
                ),
            ],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 100.0);
        assert_eq!(layout.children[1].size.width, 100.0);
        assert_eq!(layout.children[1].size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                },
                ..Default::default()
            },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { ..Default::default() },
                Box::new(|constraint| {
                    Ok(stretch::geometry::Size {
                        width: constraint.width.or_else(50.0),
                        height: constraint.height.or_else(50.0),
                    })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                },
                ..Default::default()
            },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { position_type: stretch::style::PositionType::Absolute, ..Default::default() },
                Box::new(|constraint| {
                    Ok(stretch::geometry::Size {
                        width: constraint.width.or_else(50.0),
                        height: constraint.height.or_else(50.0),
                    })
                }),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[0].size.width, 50.0);
        assert_eq!(layout.children[0].size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                },
                ..Default::default()
            },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { flex_grow: 1.0, ..Default::default() },
                Box::new(|_| Ok(stretch::geometry::Size { width: 200.0, height: 200.0 })),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(layout.children[0].size.width, 100.0);
        assert_eq!(layout.children[0].size.height, 100.0);
    }

    #[test]
    fn only_measure_once() {
        let mut num_measure = 0;
        let num_measure_ptr = &mut num_measure as *mut i32;

        stretch::node::Node::new(
            stretch::style::Style { ..Default::default() },
            vec![&stretch::node::Node::new(
                stretch::style::Style { ..Default::default() },
                vec![&stretch::node::Node::new_leaf(
                    stretch::style::Style { ..Default::default() },
                    Box::new(move |constraint| {
                        unsafe { (*num_measure_ptr) += 1 };
                        Ok(stretch::geometry::Size {
                            width: constraint.width.or_else(50.0),
                            height: constraint.height.or_else(50.0),
                        })
                    }),
                )],
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined())
        .unwrap();

        assert_eq!(num_measure, 1);
    }

    #[test]
    fn propagate_measure_error() {
        let layout = stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                },
                ..Default::default()
            },
            vec![&stretch::node::Node::new_leaf(
                stretch::style::Style { flex_grow: 1.0, ..Default::default() },
                Box::new(|_| Err(Box::new(""))),
            )],
        )
        .compute_layout(stretch::geometry::Size::undefined());

        assert_eq!(layout.is_err(), true);
    }
}
