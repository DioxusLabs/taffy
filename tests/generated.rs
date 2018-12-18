#[cfg(test)]
mod generated {
    #[test]
    fn justify_content_row_space_around() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::SpaceAround,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 12.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 45.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 78.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 45.0000);
    }

    #[test]
    fn absolute_layout_justify_content_center() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 25.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_bottom_and_top() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Auto,
                        bottom: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn margin_auto_left_and_right_strech() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 50.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn rounding_flex_basis_overrides_main_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(113.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 113.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 64.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 25.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 64.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 24.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 89.0000);
    }

    #[test]
    fn percentage_flex_basis() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.5000),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.2500),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 125.0000);
        assert_eq!(layout.children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 75.0000);
        assert_eq!(layout.children[1].size.height, 200.0000);
        assert_eq!(layout.children[1].location.x, 125.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn rounding_fractial_input_3() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(113.4000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 113.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 64.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 25.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 64.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 24.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 89.0000);
    }

    #[test]
    fn margin_and_stretch_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0000,
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    end: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn wrapped_column_max_height_flex() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            align_content: stretch::style::AlignContent::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(700.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(500.0000),
                        ..Default::default()
                    },
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(200.0000),
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(20.0000),
                        end: stretch::style::Dimension::Points(20.0000),
                        top: stretch::style::Dimension::Points(20.0000),
                        bottom: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 700.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 180.0000);
        assert_eq!(layout.children[0].location.x, 300.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 200.0000);
        assert_eq!(layout.children[1].size.height, 180.0000);
        assert_eq!(layout.children[1].location.x, 250.0000);
        assert_eq!(layout.children[1].location.y, 200.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 300.0000);
        assert_eq!(layout.children[2].location.y, 400.0000);
    }

    #[test]
    fn align_baseline_nested_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Baseline,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0000),
                            height: stretch::style::Dimension::Points(10.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 40.0000);

        assert_eq!(layout.children[1].children[0].size.width, 50.0000);
        assert_eq!(layout.children[1].children[0].size.height, 10.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn max_height_overrides_height_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(200.0000), ..Default::default() },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);
    }

    #[test]
    fn padding_center_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(20.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(20.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 40.0000);
        assert_eq!(layout.children[0].location.y, 40.0000);
    }

    #[test]
    fn align_flex_start_with_stretching_children() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    children: vec![stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_shrink: 1.0000,
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 500.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].size.height, 500.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 500.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn min_width_overrides_width() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 0.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_wrap_align_stretch_fits_one_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(150.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 150.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn justify_content_row_space_evenly() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::SpaceEvenly,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 25.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 0.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 75.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_child_order() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 55.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);

        assert_eq!(layout.children[1].size.width, 60.0000);
        assert_eq!(layout.children[1].size.height, 40.0000);
        assert_eq!(layout.children[1].location.x, 25.0000);
        assert_eq!(layout.children[1].location.y, 30.0000);

        assert_eq!(layout.children[2].size.width, 55.0000);
        assert_eq!(layout.children[2].size.height, 40.0000);
        assert_eq!(layout.children[2].location.x, 55.0000);
        assert_eq!(layout.children[2].location.y, 30.0000);
    }

    #[test]
    fn absolute_layout_percentage_bottom_based_on_parent_height() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.5000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        bottom: stretch::style::Dimension::Percent(0.5000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.1000),
                        bottom: stretch::style::Dimension::Percent(0.1000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 100.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 90.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 160.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 20.0000);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 50.0000);
        assert_eq!(layout.children[0].location.y, 60.0000);
    }

    #[test]
    fn nested_overflowing_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(200.0000),
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_direction_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 10.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 20.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn child_min_max_width_flexing() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(120.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.5000),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 120.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 20.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 100.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_row_container_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                align_self: stretch::style::AlignSelf::FlexEnd,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_and_flex_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0000,
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn flex_basis_flex_shrink_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(100.0000), ..Default::default() },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(50.0000), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 67.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 33.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 67.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn justify_content_column_min_height_and_margin_top() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 20.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 20.0000);
    }

    #[test]
    fn justify_content_overflow_min_max() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(110.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0000);
        assert_eq!(layout.size.height, 110.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, -20.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 30.0000);

        assert_eq!(layout.children[2].size.width, 50.0000);
        assert_eq!(layout.children[2].size.height, 50.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 80.0000);
    }

    #[test]
    fn wrap_reverse_column_fixed_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 135.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 135.0000);
        assert_eq!(layout.children[1].location.y, 10.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 135.0000);
        assert_eq!(layout.children[2].location.y, 30.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 40.0000);
        assert_eq!(layout.children[3].location.x, 135.0000);
        assert_eq!(layout.children[3].location.y, 60.0000);

        assert_eq!(layout.children[4].size.width, 30.0000);
        assert_eq!(layout.children[4].size.height, 50.0000);
        assert_eq!(layout.children[4].location.x, 35.0000);
        assert_eq!(layout.children[4].location.y, 0.0000);
    }

    #[test]
    fn max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn justify_content_row_center() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 35.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 45.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 55.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_root_minimized() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0000,
                min_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(500.0000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_basis: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 300.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 300.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 100.0000);
        assert_eq!(layout.children[0].children[1].size.height, 100.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 200.0000);
    }

    #[test]
    fn container_with_unsized_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node { ..Default::default() }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_within_constrained_min_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_shrink_by_outer_margin_with_max_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(80.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 20.0000);
        assert_eq!(layout.size.height, 80.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 100.0000);
    }

    #[test]
    fn absolute_layout_width_height_start_top_end_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    end: stretch::style::Dimension::Points(10.0000),
                    top: stretch::style::Dimension::Points(10.0000),
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn overflow_main_axis() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_shrink: 0.0000,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_to_min() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, flex_shrink: 1.0000, ..Default::default() },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 50.0000);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_top_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 25.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn min_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    min_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(60.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 80.0000);
    }

    #[test]
    fn justify_content_column_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 10.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 20.0000);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_column_container() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 80.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_should_not_be_part_of_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(250.0000),
                height: stretch::style::Dimension::Points(250.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 250.0000);
        assert_eq!(layout.size.height, 250.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 20.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn border_flex_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0000,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn wrap_reverse_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(31.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(32.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(33.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(34.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 60.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 31.0000);
        assert_eq!(layout.children[0].size.height, 30.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);

        assert_eq!(layout.children[1].size.width, 32.0000);
        assert_eq!(layout.children[1].size.height, 30.0000);
        assert_eq!(layout.children[1].location.x, 31.0000);
        assert_eq!(layout.children[1].location.y, 30.0000);

        assert_eq!(layout.children[2].size.width, 33.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 63.0000);
        assert_eq!(layout.children[2].location.y, 30.0000);

        assert_eq!(layout.children[3].size.width, 34.0000);
        assert_eq!(layout.children[3].size.height, 30.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 0.0000);
    }

    #[test]
    fn margin_right() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 80.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn display_none_with_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    ..Default::default()
                },
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    children: vec![stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_shrink: 1.0000,
                        flex_basis: stretch::style::Dimension::Percent(0.0000),
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 0.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 0.0000);
        assert_eq!(layout.children[1].children[0].size.height, 0.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 50.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 50.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn align_flex_start_with_shrinking_children_with_stretch() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexStart,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    children: vec![stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_shrink: 1.0000,
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 500.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_smaller_then_content_with_flex_grow_large_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 70.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn align_items_stretch() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn parent_wrap_child_size_overflowing_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn padding_stretch_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn wrap_nodes_with_content_sizing_overflowing_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(85.0000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40.0000),
                                height: stretch::style::Dimension::Points(40.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        margin: stretch::geometry::Rect {
                            end: stretch::style::Dimension::Points(10.0000),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40.0000),
                                height: stretch::style::Dimension::Points(40.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 85.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 40.0000);
        assert_eq!(layout.children[0].children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 40.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 40.0000);
        assert_eq!(layout.children[0].children[1].size.height, 40.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 40.0000);

        assert_eq!(layout.children[0].children[1].children[0].size.width, 40.0000);
        assert_eq!(layout.children[0].children[1].children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_shrink_flex_grow_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.0000,
                    flex_shrink: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(500.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 0.0000,
                    flex_shrink: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(500.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 250.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 250.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 250.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_flex_shrink_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(100.0000), ..Default::default() },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(50.0000), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 67.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 33.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 67.0000);
    }

    #[test]
    fn align_items_min_max() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(60.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 60.0000);
        assert_eq!(layout.children[0].location.x, 20.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_smaller_then_content_with_flex_grow_unconstraint_size() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 90.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 20.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 70.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn justify_content_row_min_width_and_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0000);
        assert_eq!(layout.size.height, 20.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 20.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn align_self_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::FlexStart,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_position_left_top() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(400.0000),
                height: stretch::style::Dimension::Points(400.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.4500),
                    height: stretch::style::Dimension::Percent(0.5500),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1000),
                    top: stretch::style::Dimension::Percent(0.2000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 400.0000);
        assert_eq!(layout.size.height, 400.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 180.0000);
        assert_eq!(layout.children[0].size.height, 220.0000);
        assert_eq!(layout.children[0].location.x, 40.0000);
        assert_eq!(layout.children[0].location.y, 80.0000);
    }

    #[test]
    fn border_stretch_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn rounding_fractial_input_2() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(113.6000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 114.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 65.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 24.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 65.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 25.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 89.0000);
    }

    #[test]
    fn percentage_flex_basis_main_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1500),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 120.0000);
        assert_eq!(layout.children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 40.0000);
        assert_eq!(layout.children[1].size.height, 200.0000);
        assert_eq!(layout.children[1].location.x, 120.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_top_and_bottom_strech() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Auto,
                        bottom: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 50.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 150.0000);
    }

    #[test]
    fn percentage_multiple_nested_with_padding_margin_and_percentage_values() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(5.0000),
                        end: stretch::style::Dimension::Points(5.0000),
                        top: stretch::style::Dimension::Points(5.0000),
                        bottom: stretch::style::Dimension::Points(5.0000),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(3.0000),
                        end: stretch::style::Dimension::Points(3.0000),
                        top: stretch::style::Dimension::Points(3.0000),
                        bottom: stretch::style::Dimension::Points(3.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(0.5000),
                            ..Default::default()
                        },
                        margin: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Points(5.0000),
                            end: stretch::style::Dimension::Points(5.0000),
                            top: stretch::style::Dimension::Points(5.0000),
                            bottom: stretch::style::Dimension::Points(5.0000),
                            ..Default::default()
                        },
                        padding: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Percent(0.0300),
                            end: stretch::style::Dimension::Percent(0.0300),
                            top: stretch::style::Dimension::Percent(0.0300),
                            bottom: stretch::style::Dimension::Percent(0.0300),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Percent(0.4500),
                                ..Default::default()
                            },
                            margin: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Percent(0.0500),
                                end: stretch::style::Dimension::Percent(0.0500),
                                top: stretch::style::Dimension::Percent(0.0500),
                                bottom: stretch::style::Dimension::Percent(0.0500),
                                ..Default::default()
                            },
                            padding: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Points(3.0000),
                                end: stretch::style::Dimension::Points(3.0000),
                                top: stretch::style::Dimension::Points(3.0000),
                                bottom: stretch::style::Dimension::Points(3.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1500),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 190.0000);
        assert_eq!(layout.children[0].size.height, 48.0000);
        assert_eq!(layout.children[0].location.x, 5.0000);
        assert_eq!(layout.children[0].location.y, 5.0000);

        assert_eq!(layout.children[0].children[0].size.width, 92.0000);
        assert_eq!(layout.children[0].children[0].size.height, 25.0000);
        assert_eq!(layout.children[0].children[0].location.x, 8.0000);
        assert_eq!(layout.children[0].children[0].location.y, 8.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 36.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 6.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 10.0000);

        assert_eq!(layout.children[1].size.width, 200.0000);
        assert_eq!(layout.children[1].size.height, 142.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 58.0000);
    }

    #[test]
    fn flex_grow_within_constrained_min_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 50.0000);
    }

    #[test]
    fn flex_grow_in_at_most_container() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexStart,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_left() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { start: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 100.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn percentage_flex_basis_cross() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.5000),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.2500),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 125.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 200.0000);
        assert_eq!(layout.children[1].size.height, 75.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 125.0000);
    }

    #[test]
    fn percentage_absolute_position() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.3000),
                    top: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 60.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn flex_basis_flex_grow_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 75.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 25.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 75.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_column_container_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                align_self: stretch::style::AlignSelf::FlexEnd,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn justify_content_row_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 10.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 20.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_left_right_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(52.0000),
                height: stretch::style::Dimension::Points(52.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(72.0000),
                    height: stretch::style::Dimension::Points(72.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Auto,
                    end: stretch::style::Dimension::Auto,
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 52.0000);
        assert_eq!(layout.size.height, 52.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 52.0000);
        assert_eq!(layout.children[0].size.height, 72.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_larger_than_content_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_basis: stretch::style::Dimension::Points(50.0000),
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_fix_left_auto_right_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(52.0000),
                height: stretch::style::Dimension::Points(52.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(72.0000),
                    height: stretch::style::Dimension::Points(72.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    end: stretch::style::Dimension::Auto,
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 52.0000);
        assert_eq!(layout.size.height, 52.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 42.0000);
        assert_eq!(layout.children[0].size.height, 72.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_container_in_wrapping_container() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(1.0000),
                        ..Default::default()
                    },
                    children: vec![
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50.0000),
                                height: stretch::style::Dimension::Points(50.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50.0000),
                                height: stretch::style::Dimension::Points(50.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 50.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[1].size.width, 50.0000);
        assert_eq!(layout.children[0].children[0].children[1].size.height, 50.0000);
        assert_eq!(layout.children[0].children[0].children[1].location.x, 50.0000);
        assert_eq!(layout.children[0].children[0].children[1].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_left_stretching_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    margin: stretch::geometry::Rect { start: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 150.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 100.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn flex_basis_smaller_then_content_with_flex_grow_small_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(10.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 10.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 20.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 70.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_flex_basis_cross_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1500),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 120.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 40.0000);
        assert_eq!(layout.children[1].size.height, 150.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 50.0000);
    }

    #[test]
    fn align_items_flex_end_child_without_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexEnd,
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(70.0000),
                        height: stretch::style::Dimension::Points(70.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 70.0000);
        assert_eq!(layout.children[0].location.x, -10.0000);
        assert_eq!(layout.children[0].location.y, -10.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 70.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn width_smaller_then_content_with_flex_grow_very_large_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(200.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 100.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn size_defined_by_child_with_padding() {
        let layout = stretch::compute(&stretch::style::Node {
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 30.0000);
        assert_eq!(layout.size.height, 30.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn absolute_layout_within_border() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(0.0000),
                        top: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        end: stretch::style::Dimension::Points(0.0000),
                        bottom: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10.0000),
                        end: stretch::style::Dimension::Points(10.0000),
                        top: stretch::style::Dimension::Points(10.0000),
                        bottom: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(0.0000),
                        top: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10.0000),
                        end: stretch::style::Dimension::Points(10.0000),
                        top: stretch::style::Dimension::Points(10.0000),
                        bottom: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        end: stretch::style::Dimension::Points(0.0000),
                        bottom: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 40.0000);
        assert_eq!(layout.children[1].location.y, 40.0000);

        assert_eq!(layout.children[2].size.width, 50.0000);
        assert_eq!(layout.children[2].size.height, 50.0000);
        assert_eq!(layout.children[2].location.x, 20.0000);
        assert_eq!(layout.children[2].location.y, 20.0000);

        assert_eq!(layout.children[3].size.width, 50.0000);
        assert_eq!(layout.children[3].size.height, 50.0000);
        assert_eq!(layout.children[3].location.x, 30.0000);
        assert_eq!(layout.children[3].location.y, 30.0000);
    }

    #[test]
    fn margin_auto_left_fix_right_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(52.0000),
                height: stretch::style::Dimension::Points(52.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(72.0000),
                    height: stretch::style::Dimension::Points(72.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Auto,
                    end: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 52.0000);
        assert_eq!(layout.size.height, 52.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 42.0000);
        assert_eq!(layout.children[0].size.height, 72.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn justify_content_row_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 70.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 80.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 90.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn margin_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 80.0000);
    }

    #[test]
    fn margin_should_not_be_part_of_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(250.0000),
                height: stretch::style::Dimension::Points(250.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 250.0000);
        assert_eq!(layout.size.height, 250.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 20.0000);
    }

    #[test]
    fn flex_basis_smaller_than_main_dimen_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_basis: stretch::style::Dimension::Points(10.0000),
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50.0000),
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_flex_basis_cross_min_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    min_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 2.0000,
                    min_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Percent(0.1000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 140.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 200.0000);
        assert_eq!(layout.children[1].size.height, 60.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 140.0000);
    }

    #[test]
    fn nested_overflowing_child_in_constraint_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(200.0000),
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn wrapped_row_within_align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                children: vec![
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(150.0000),
                            height: stretch::style::Dimension::Points(80.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(80.0000),
                            height: stretch::style::Dimension::Points(80.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 160.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 150.0000);
        assert_eq!(layout.children[0].children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 80.0000);
        assert_eq!(layout.children[0].children[1].size.height, 80.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 80.0000);
    }

    #[test]
    fn justify_content_column_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 35.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 45.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 55.0000);
    }

    #[test]
    fn align_items_center_child_with_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::Center,
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10.0000),
                        end: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, -10.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_width_height() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.3000),
                    height: stretch::style::Dimension::Percent(0.3000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 60.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_mutiple_children_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 75.0000);
        assert_eq!(layout.children[0].location.y, 25.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 75.0000);
        assert_eq!(layout.children[1].location.y, 100.0000);

        assert_eq!(layout.children[2].size.width, 50.0000);
        assert_eq!(layout.children[2].size.height, 50.0000);
        assert_eq!(layout.children[2].location.x, 75.0000);
        assert_eq!(layout.children[2].location.y, 150.0000);
    }

    #[test]
    fn min_width_overrides_width_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 0.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);
    }

    #[test]
    fn flex_wrap_wrap_to_child_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            children: vec![
                stretch::style::Node {
                    flex_wrap: stretch::style::FlexWrap::Wrap,
                    align_items: stretch::style::AlignItems::FlexStart,
                    children: vec![stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(100.0000),
                                height: stretch::style::Dimension::Points(100.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 100.0000);
    }

    #[test]
    fn wrap_nodes_with_content_sizing_margin_cross() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(70.0000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40.0000),
                                height: stretch::style::Dimension::Points(40.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        margin: stretch::geometry::Rect {
                            top: stretch::style::Dimension::Points(10.0000),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40.0000),
                                height: stretch::style::Dimension::Points(40.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 90.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 40.0000);
        assert_eq!(layout.children[0].children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 40.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 40.0000);
        assert_eq!(layout.children[0].children[1].size.height, 40.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 50.0000);

        assert_eq!(layout.children[0].children[1].children[0].size.width, 40.0000);
        assert_eq!(layout.children[0].children[1].children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn align_baseline() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Baseline,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 30.0000);
    }

    #[test]
    fn margin_with_sibling_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    margin: stretch::geometry::Rect {
                        end: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 45.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 45.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 55.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn wrap_reverse_row_align_content_space_around() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::SpaceAround,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 80.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 70.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 30.0000);
        assert_eq!(layout.children[1].location.y, 60.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 60.0000);
        assert_eq!(layout.children[2].location.y, 50.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 40.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 10.0000);

        assert_eq!(layout.children[4].size.width, 30.0000);
        assert_eq!(layout.children[4].size.height, 50.0000);
        assert_eq!(layout.children[4].location.x, 30.0000);
        assert_eq!(layout.children[4].location.y, 0.0000);
    }

    #[test]
    fn wrap_reverse_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(31.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(32.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(33.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(34.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 31.0000);
        assert_eq!(layout.children[0].location.x, 70.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 32.0000);
        assert_eq!(layout.children[1].location.x, 70.0000);
        assert_eq!(layout.children[1].location.y, 31.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 33.0000);
        assert_eq!(layout.children[2].location.x, 70.0000);
        assert_eq!(layout.children[2].location.y, 63.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 34.0000);
        assert_eq!(layout.children[3].location.x, 20.0000);
        assert_eq!(layout.children[3].location.y, 0.0000);
    }

    #[test]
    fn align_items_center_with_child_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 50.0000);
    }

    #[test]
    fn flex_direction_column_reverse() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::ColumnReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 90.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 80.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 70.0000);
    }

    #[test]
    fn flex_direction_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 10.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 20.0000);
    }

    #[test]
    fn align_items_center_child_without_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::Center,
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(70.0000),
                        height: stretch::style::Dimension::Points(70.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 70.0000);
        assert_eq!(layout.children[0].location.x, -10.0000);
        assert_eq!(layout.children[0].location.y, -10.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 70.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn align_self_flex_end_override_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexStart,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::FlexEnd,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 90.0000);
    }

    #[test]
    fn rounding_total_fractial() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(87.4000),
                height: stretch::style::Dimension::Points(113.4000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.7000,
                    flex_basis: stretch::style::Dimension::Points(50.3000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.3000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.6000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.1000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.7000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 87.0000);
        assert_eq!(layout.size.height, 113.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 87.0000);
        assert_eq!(layout.children[0].size.height, 59.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 87.0000);
        assert_eq!(layout.children[1].size.height, 30.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 59.0000);

        assert_eq!(layout.children[2].size.width, 87.0000);
        assert_eq!(layout.children[2].size.height, 24.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 89.0000);
    }

    #[test]
    fn align_baseline_child_multiline() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Baseline,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(60.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_wrap: stretch::style::FlexWrap::Wrap,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    children: vec![
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(20.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(10.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(20.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25.0000),
                                height: stretch::style::Dimension::Points(10.0000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 80.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 60.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 40.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 40.0000);

        assert_eq!(layout.children[1].children[0].size.width, 25.0000);
        assert_eq!(layout.children[1].children[0].size.height, 20.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].children[1].size.width, 25.0000);
        assert_eq!(layout.children[1].children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].children[1].location.x, 25.0000);
        assert_eq!(layout.children[1].children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[2].size.width, 25.0000);
        assert_eq!(layout.children[1].children[2].size.height, 20.0000);
        assert_eq!(layout.children[1].children[2].location.x, 0.0000);
        assert_eq!(layout.children[1].children[2].location.y, 20.0000);

        assert_eq!(layout.children[1].children[3].size.width, 25.0000);
        assert_eq!(layout.children[1].children[3].size.height, 10.0000);
        assert_eq!(layout.children[1].children[3].location.x, 25.0000);
        assert_eq!(layout.children[1].children[3].location.y, 20.0000);
    }

    #[test]
    fn absolute_layout_align_items_center_on_child_only() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                align_self: stretch::style::AlignSelf::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);
    }

    #[test]
    fn wrap_reverse_row_single_line_different_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::FlexStart,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(300.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 300.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 40.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 30.0000);
        assert_eq!(layout.children[1].location.y, 30.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 60.0000);
        assert_eq!(layout.children[2].location.y, 20.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 40.0000);
        assert_eq!(layout.children[3].location.x, 90.0000);
        assert_eq!(layout.children[3].location.y, 10.0000);

        assert_eq!(layout.children[4].size.width, 30.0000);
        assert_eq!(layout.children[4].size.height, 50.0000);
        assert_eq!(layout.children[4].location.x, 120.0000);
        assert_eq!(layout.children[4].location.y, 0.0000);
    }

    #[test]
    fn rounding_fractial_input_4() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(113.4000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 113.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 64.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 25.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 64.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 24.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 89.0000);
    }

    #[test]
    fn width_smaller_then_content_with_flex_grow_unconstraint_size() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn padding_flex_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0000,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn flex_direction_row_reverse() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::RowReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 90.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 80.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 70.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn percentage_size_based_on_parent_inner_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(20.0000),
                end: stretch::style::Dimension::Points(20.0000),
                top: stretch::style::Dimension::Points(20.0000),
                bottom: stretch::style::Dimension::Points(20.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5000),
                    height: stretch::style::Dimension::Percent(0.5000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 20.0000);
        assert_eq!(layout.children[0].location.y, 20.0000);
    }

    #[test]
    fn wrap_reverse_row_align_content_stretch() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 80.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 70.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 30.0000);
        assert_eq!(layout.children[1].location.y, 60.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 60.0000);
        assert_eq!(layout.children[2].location.y, 50.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 40.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 10.0000);

        assert_eq!(layout.children[4].size.width, 30.0000);
        assert_eq!(layout.children[4].size.height, 50.0000);
        assert_eq!(layout.children[4].location.x, 30.0000);
        assert_eq!(layout.children[4].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_top() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 150.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn flex_grow_within_constrained_max_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(200.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        flex_shrink: 1.0000,
                        flex_basis: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 67.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 33.0000);
        assert_eq!(layout.children[0].children[1].size.height, 100.0000);
        assert_eq!(layout.children[0].children[1].location.x, 67.0000);
        assert_eq!(layout.children[0].children[1].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 25.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);
    }

    #[test]
    fn align_self_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::FlexEnd,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 90.0000);
    }

    #[test]
    fn margin_auto_left_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(52.0000),
                height: stretch::style::Dimension::Points(52.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(72.0000),
                    height: stretch::style::Dimension::Points(72.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect { start: stretch::style::Dimension::Auto, ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 52.0000);
        assert_eq!(layout.size.height, 52.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 52.0000);
        assert_eq!(layout.children[0].size.height, 72.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn wrapped_row_within_align_items_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                children: vec![
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(150.0000),
                            height: stretch::style::Dimension::Points(80.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(80.0000),
                            height: stretch::style::Dimension::Points(80.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 160.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 150.0000);
        assert_eq!(layout.children[0].children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 80.0000);
        assert_eq!(layout.children[0].children[1].size.height, 80.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 80.0000);
    }

    #[test]
    fn min_height_overrides_height() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn align_strech_should_size_based_on_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0.0000,
                flex_shrink: 1.0000,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(20.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn wrap_row_align_items_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::FlexEnd,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 60.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 20.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 30.0000);
        assert_eq!(layout.children[1].location.y, 10.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 60.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 30.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 30.0000);
    }

    #[test]
    fn size_defined_by_child_with_border() {
        let layout = stretch::compute(&stretch::style::Node {
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 30.0000);
        assert_eq!(layout.size.height, 30.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn margin_auto_bottom_and_top_justify_center() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Auto,
                        bottom: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 50.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 100.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_overrides_main_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 20.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 60.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 20.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 80.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn wrapped_row_within_align_items_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexStart,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                children: vec![
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(150.0000),
                            height: stretch::style::Dimension::Points(80.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(80.0000),
                            height: stretch::style::Dimension::Points(80.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 160.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 150.0000);
        assert_eq!(layout.children[0].children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 80.0000);
        assert_eq!(layout.children[0].children[1].size.height, 80.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 80.0000);
    }

    #[test]
    fn align_center_should_size_based_on_content() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0.0000,
                flex_shrink: 1.0000,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(20.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 40.0000);

        assert_eq!(layout.children[0].children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_right() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { end: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn wrap_reverse_row_align_content_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 80.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 70.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 30.0000);
        assert_eq!(layout.children[1].location.y, 60.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 60.0000);
        assert_eq!(layout.children[2].location.y, 50.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 40.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 10.0000);

        assert_eq!(layout.children[4].size.width, 30.0000);
        assert_eq!(layout.children[4].size.height, 50.0000);
        assert_eq!(layout.children[4].location.x, 30.0000);
        assert_eq!(layout.children[4].location.y, 0.0000);
    }

    #[test]
    fn padding_no_child() {
        let layout = stretch::compute(&stretch::style::Node {
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.size.width, 20.0000);
        assert_eq!(layout.size.height, 20.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);
    }

    #[test]
    fn margin_with_sibling_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    margin: stretch::geometry::Rect {
                        bottom: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 45.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 45.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 55.0000);
    }

    #[test]
    fn flex_direction_row_no_width() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 30.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 10.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 20.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn max_width_overrides_width_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(200.0000), ..Default::default() },
            max_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 0.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);
    }

    #[test]
    fn margin_auto_left_and_right() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 50.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_direction_column_no_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 30.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 10.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 20.0000);
    }

    #[test]
    fn flex_basis_unconstraint_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            children: vec![stretch::style::Node {
                flex_basis: stretch::style::Dimension::Points(50.0000),
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn width_smaller_then_content_with_flex_grow_small_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(10.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 10.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 5.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 5.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 5.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_row_container() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 80.0000);
    }

    #[test]
    fn flex_basis_and_main_dimen_set_when_flexing() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(10.0000),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(10.0000),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_smaller_than_main_dimen_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_basis: stretch::style::Dimension::Points(10.0000),
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50.0000),
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_margin_should_calculate_based_only_on_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0000,
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1000),
                    end: stretch::style::Dimension::Percent(0.1000),
                    top: stretch::style::Dimension::Percent(0.1000),
                    bottom: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 160.0000);
        assert_eq!(layout.children[0].size.height, 60.0000);
        assert_eq!(layout.children[0].location.x, 20.0000);
        assert_eq!(layout.children[0].location.y, 20.0000);

        assert_eq!(layout.children[0].children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_start_top_end_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    end: stretch::style::Dimension::Points(10.0000),
                    top: stretch::style::Dimension::Points(10.0000),
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn min_width() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 20.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 80.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn min_height_overrides_height_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(50.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_width_height_end_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Points(10.0000),
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 80.0000);
        assert_eq!(layout.children[0].location.y, 80.0000);
    }

    #[test]
    fn align_self_center() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 45.0000);
    }

    #[test]
    fn wrapped_column_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            align_content: stretch::style::AlignContent::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(700.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(500.0000),
                        ..Default::default()
                    },
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(200.0000),
                        height: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(20.0000),
                        end: stretch::style::Dimension::Points(20.0000),
                        top: stretch::style::Dimension::Points(20.0000),
                        bottom: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 700.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].location.x, 250.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);

        assert_eq!(layout.children[1].size.width, 200.0000);
        assert_eq!(layout.children[1].size.height, 200.0000);
        assert_eq!(layout.children[1].location.x, 200.0000);
        assert_eq!(layout.children[1].location.y, 250.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 420.0000);
        assert_eq!(layout.children[2].location.y, 200.0000);
    }

    #[test]
    fn wrap_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(31.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(32.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(33.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(34.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 60.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 31.0000);
        assert_eq!(layout.children[0].size.height, 30.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 32.0000);
        assert_eq!(layout.children[1].size.height, 30.0000);
        assert_eq!(layout.children[1].location.x, 31.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 33.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 63.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);

        assert_eq!(layout.children[3].size.width, 34.0000);
        assert_eq!(layout.children[3].size.height, 30.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 30.0000);
    }

    #[test]
    fn max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_child() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                flex_grow: 1.0000,
                flex_basis: stretch::style::Dimension::Points(0.0000),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_size_of_flex_basis() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_basis: stretch::style::Dimension::Points(50.0000),
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(1.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn align_items_flex_end_child_with_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexEnd,
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10.0000),
                        end: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 50.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, -10.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn justify_content_row_max_width_and_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            max_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(80.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 80.0000);
        assert_eq!(layout.size.height, 20.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 90.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn margin_left() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn size_defined_by_child() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn rounding_total_fractial_nested() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(87.4000),
                height: stretch::style::Dimension::Points(113.4000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 0.7000,
                    flex_basis: stretch::style::Dimension::Points(50.3000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.3000),
                        ..Default::default()
                    },
                    children: vec![
                        stretch::style::Node {
                            flex_grow: 1.0000,
                            flex_basis: stretch::style::Dimension::Points(0.3000),
                            size: stretch::geometry::Size {
                                height: stretch::style::Dimension::Points(9.9000),
                                ..Default::default()
                            },
                            position: stretch::geometry::Rect {
                                bottom: stretch::style::Dimension::Points(13.3000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        stretch::style::Node {
                            flex_grow: 4.0000,
                            flex_basis: stretch::style::Dimension::Points(0.3000),
                            size: stretch::geometry::Size {
                                height: stretch::style::Dimension::Points(1.1000),
                                ..Default::default()
                            },
                            position: stretch::geometry::Rect {
                                top: stretch::style::Dimension::Points(13.3000),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.6000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.1000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.7000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 87.0000);
        assert_eq!(layout.size.height, 113.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 87.0000);
        assert_eq!(layout.children[0].size.height, 59.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 87.0000);
        assert_eq!(layout.children[0].children[0].size.height, 12.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, -13.0000);

        assert_eq!(layout.children[0].children[1].size.width, 87.0000);
        assert_eq!(layout.children[0].children[1].size.height, 47.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 25.0000);

        assert_eq!(layout.children[1].size.width, 87.0000);
        assert_eq!(layout.children[1].size.height, 30.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 59.0000);

        assert_eq!(layout.children[2].size.width, 87.0000);
        assert_eq!(layout.children[2].size.height, 24.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 89.0000);
    }

    #[test]
    fn min_max_percent_no_width_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexStart,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.1000),
                    height: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.1000),
                    height: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_smaller_then_content_with_flex_grow_very_large_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(200.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 100.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn percent_within_flex_grow() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(350.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(1.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 350.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 150.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 100.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 150.0000);
        assert_eq!(layout.children[1].children[0].size.height, 0.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 250.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn margin_and_stretch_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0000,
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 80.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn margin_top() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn percentage_flex_basis_main_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Percent(0.2000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 52.0000);
        assert_eq!(layout.children[0].size.height, 120.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 148.0000);
        assert_eq!(layout.children[1].size.height, 40.0000);
        assert_eq!(layout.children[1].location.x, 52.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_left_and_right_column() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 50.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn margin_and_flex_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0000,
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    end: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_wrap_children_with_min_main_overriding_flex_basis() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(55.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(55.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 55.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 55.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 50.0000);
    }

    #[test]
    fn wrap_row_align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 60.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 30.0000);
        assert_eq!(layout.children[1].location.y, 5.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 60.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 30.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 30.0000);
    }

    #[test]
    fn display_none_with_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10.0000),
                        end: stretch::style::Dimension::Points(10.0000),
                        top: stretch::style::Dimension::Points(10.0000),
                        bottom: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_flex_grow_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 75.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 25.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_right_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Points(5.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 45.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);
    }

    #[test]
    fn overflow_cross_axis() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    height: stretch::style::Dimension::Points(200.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn display_none_with_position() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    flex_grow: 1.0000,
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 0.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn wrap_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(31.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(32.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(33.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(34.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 31.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 32.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 31.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 33.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 63.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 34.0000);
        assert_eq!(layout.children[3].location.x, 50.0000);
        assert_eq!(layout.children[3].location.y, 0.0000);
    }

    #[test]
    fn justify_content_column_space_evenly() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::SpaceEvenly,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 18.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 45.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 73.0000);
    }

    #[test]
    fn flex_grow_within_constrained_max_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 67.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 33.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 67.0000);
    }

    #[test]
    fn justify_content_column_space_around() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::SpaceAround,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 12.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 45.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 78.0000);
    }

    #[test]
    fn flex_basis_larger_than_content_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_basis: stretch::style::Dimension::Points(50.0000),
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_left_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(5.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 5.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);
    }

    #[test]
    fn percentage_flex_basis_main_min_width() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1500),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 120.0000);
        assert_eq!(layout.children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 80.0000);
        assert_eq!(layout.children[1].size.height, 200.0000);
        assert_eq!(layout.children[1].location.x, 120.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { bottom: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn flex_grow_shrink_at_most() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node { flex_grow: 1.0000, flex_shrink: 1.0000, ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_padding_should_calculate_based_only_on_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0000,
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.1000),
                    end: stretch::style::Dimension::Percent(0.1000),
                    top: stretch::style::Dimension::Percent(0.1000),
                    bottom: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].children[0].location.x, 20.0000);
        assert_eq!(layout.children[0].children[0].location.y, 20.0000);
    }

    #[test]
    fn flex_shrink_to_zero() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(75.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 0.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 75.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 50.0000);
        assert_eq!(layout.children[2].size.height, 50.0000);
        assert_eq!(layout.children[2].location.x, 50.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn margin_auto_mutiple_children_row() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { end: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect { end: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 75.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);

        assert_eq!(layout.children[2].size.width, 50.0000);
        assert_eq!(layout.children[2].size.height, 50.0000);
        assert_eq!(layout.children[2].location.x, 150.0000);
        assert_eq!(layout.children[2].location.y, 75.0000);
    }

    #[test]
    fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(1080.0000),
                height: stretch::style::Dimension::Points(1584.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(400.0000),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(100.0000),
                        end: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(199.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 1080.0000);
        assert_eq!(layout.size.height, 1584.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 1080.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 400.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 199.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 101.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn size_defined_by_grand_child() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_bottom_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 25.0000);
        assert_eq!(layout.children[0].location.y, 50.0000);
    }

    #[test]
    fn justify_content_min_max() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(60.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 60.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 20.0000);
    }

    #[test]
    fn padding_align_end_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(20.0000),
                    end: stretch::style::Dimension::Points(20.0000),
                    top: stretch::style::Dimension::Points(20.0000),
                    bottom: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 100.0000);
        assert_eq!(layout.children[0].location.y, 100.0000);
    }

    #[test]
    fn justify_content_row_space_between() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::SpaceBetween,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 10.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 45.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 10.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 90.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn align_self_baseline() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    align_self: stretch::style::AlignSelf::Baseline,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    align_self: stretch::style::AlignSelf::Baseline,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0000),
                            height: stretch::style::Dimension::Points(10.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 40.0000);

        assert_eq!(layout.children[1].children[0].size.width, 50.0000);
        assert_eq!(layout.children[1].children[0].size.height, 10.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn rounding_fractial_input_1() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(113.4000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(50.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 113.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 64.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 25.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 64.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 24.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 89.0000);
    }

    #[test]
    fn absolute_layout_align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60.0000),
                    height: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 110.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 40.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 30.0000);
    }

    #[test]
    fn flex_basis_slightly_smaller_then_content_with_flex_grow_large_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(60.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 80.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 20.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 80.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_flex_basis_cross_min_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1500),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 200.0000);
        assert_eq!(layout.children[1].size.height, 150.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 50.0000);
    }

    #[test]
    fn margin_auto_left_and_right_column_and_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 50.0000);
        assert_eq!(layout.children[0].location.y, 75.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn flex_grow_less_than_factor_one() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.2000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Points(40.0000),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 0.2000, flex_shrink: 0.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 0.4000, flex_shrink: 0.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 132.0000);
        assert_eq!(layout.children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 92.0000);
        assert_eq!(layout.children[1].size.height, 200.0000);
        assert_eq!(layout.children[1].location.x, 132.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 184.0000);
        assert_eq!(layout.children[2].size.height, 200.0000);
        assert_eq!(layout.children[2].location.x, 224.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_within_constrained_min_max_column() {
        let layout = stretch::compute(&stretch::style::Node {
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn percentage_flex_basis_cross_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Percent(0.6000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.1000),
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Percent(0.2000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 120.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 200.0000);
        assert_eq!(layout.children[1].size.height, 40.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 120.0000);
    }

    #[test]
    fn justify_content_column_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 70.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 80.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 90.0000);
    }

    #[test]
    fn rounding_flex_basis_flex_grow_row_width_of_100() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 33.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 34.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 33.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 33.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 67.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn display_none_fixed_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 0.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn wrap_reverse_row_align_content_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::FlexStart,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(30.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(40.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(30.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 80.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 70.0000);

        assert_eq!(layout.children[1].size.width, 30.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 30.0000);
        assert_eq!(layout.children[1].location.y, 60.0000);

        assert_eq!(layout.children[2].size.width, 30.0000);
        assert_eq!(layout.children[2].size.height, 30.0000);
        assert_eq!(layout.children[2].location.x, 60.0000);
        assert_eq!(layout.children[2].location.y, 50.0000);

        assert_eq!(layout.children[3].size.width, 30.0000);
        assert_eq!(layout.children[3].size.height, 40.0000);
        assert_eq!(layout.children[3].location.x, 0.0000);
        assert_eq!(layout.children[3].location.y, 10.0000);

        assert_eq!(layout.children[4].size.width, 30.0000);
        assert_eq!(layout.children[4].size.height, 50.0000);
        assert_eq!(layout.children[4].location.x, 30.0000);
        assert_eq!(layout.children[4].location.y, 0.0000);
    }

    #[test]
    fn flex_shrink_flex_grow_child_flex_shrink_other_child() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.0000,
                    flex_shrink: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(500.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(500.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 250.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 250.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 250.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_unconstraint_row() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                flex_basis: stretch::style::Dimension::Points(50.0000),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn align_items_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 90.0000);
    }

    #[test]
    fn absolute_layout_width_height_start_top() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10.0000),
                    top: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 10.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

    #[test]
    fn rounding_flex_basis_flex_shrink_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(101.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(25.0000), ..Default::default() },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(25.0000), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 101.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 67.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 17.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 67.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 17.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 84.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);
    }

    #[test]
    fn border_center_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            border: stretch::geometry::Rect {
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(20.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 45.0000);
        assert_eq!(layout.children[0].location.y, 40.0000);
    }

    #[test]
    fn align_items_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexStart,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn border_no_child() {
        let layout = stretch::compute(&stretch::style::Node {
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.size.width, 20.0000);
        assert_eq!(layout.size.height, 20.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);
    }

    #[test]
    fn rounding_flex_basis_flex_grow_row_prime_number_width() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(113.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 113.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 23.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 22.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 23.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[2].size.width, 23.0000);
        assert_eq!(layout.children[2].size.height, 100.0000);
        assert_eq!(layout.children[2].location.x, 45.0000);
        assert_eq!(layout.children[2].location.y, 0.0000);

        assert_eq!(layout.children[3].size.width, 22.0000);
        assert_eq!(layout.children[3].size.height, 100.0000);
        assert_eq!(layout.children[3].location.x, 68.0000);
        assert_eq!(layout.children[3].location.y, 0.0000);

        assert_eq!(layout.children[4].size.width, 23.0000);
        assert_eq!(layout.children[4].size.height, 100.0000);
        assert_eq!(layout.children[4].location.x, 90.0000);
        assert_eq!(layout.children[4].location.y, 0.0000);
    }

    #[test]
    fn align_items_center_with_child_top() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 10.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 55.0000);
    }

    #[test]
    fn width_smaller_then_content_with_flex_grow_large_size() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(70.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(0.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 50.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 70.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 50.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);

        assert_eq!(layout.children[1].children[0].size.width, 20.0000);
        assert_eq!(layout.children[1].children[0].size.height, 100.0000);
        assert_eq!(layout.children[1].children[0].location.x, 0.0000);
        assert_eq!(layout.children[1].children[0].location.y, 0.0000);
    }

    #[test]
    fn display_none() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    flex_grow: 1.0000,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 0.0000);
        assert_eq!(layout.children[1].size.height, 0.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_smaller_than_content_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_basis: stretch::style::Dimension::Points(50.0000),
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_width_height_undefined_parent_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5000),
                    height: stretch::style::Dimension::Percent(0.5000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 0.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_within_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_height_maximized() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0000,
                min_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(500.0000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_basis: stretch::style::Dimension::Points(200.0000),
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 500.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 400.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 100.0000);
        assert_eq!(layout.children[0].children[1].size.height, 100.0000);
        assert_eq!(layout.children[0].children[1].location.x, 0.0000);
        assert_eq!(layout.children[0].children[1].location.y, 400.0000);
    }

    #[test]
    fn max_height_overrides_height() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(200.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 0.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn relative_position_should_not_nudge_siblings() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Points(15.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Points(15.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 15.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 25.0000);
    }

    #[test]
    fn justify_content_min_width_with_padding_child_width_greater_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(1000.0000),
                height: stretch::style::Dimension::Points(1584.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(400.0000),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(100.0000),
                        end: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(300.0000),
                            height: stretch::style::Dimension::Points(100.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 1000.0000);
        assert_eq!(layout.size.height, 1584.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 1000.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 500.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 300.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 100.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_grow_flex_basis_percent_min_max() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(120.0000), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Points(0.0000),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 0.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.5000),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20.0000),
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 120.0000);
        assert_eq!(layout.size.height, 20.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 20.0000);
        assert_eq!(layout.children[1].size.height, 20.0000);
        assert_eq!(layout.children[1].location.x, 100.0000);
        assert_eq!(layout.children[1].location.y, 0.0000);
    }

    #[test]
    fn percent_absolute_position() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(60.0000),
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(1.0000),
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.5000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(1.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(1.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 60.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 60.0000);
        assert_eq!(layout.children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].location.x, 30.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 30.0000);
        assert_eq!(layout.children[0].children[0].size.height, 50.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[1].size.width, 30.0000);
        assert_eq!(layout.children[0].children[1].size.height, 50.0000);
        assert_eq!(layout.children[0].children[1].location.x, 30.0000);
        assert_eq!(layout.children[0].children[1].location.y, 0.0000);
    }

    #[test]
    fn flex_root_ignored() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_basis: stretch::style::Dimension::Points(200.0000),
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 300.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 200.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 100.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 200.0000);
    }

    #[test]
    fn justify_content_column_space_between() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::SpaceBetween,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 10.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[1].size.width, 100.0000);
        assert_eq!(layout.children[1].size.height, 10.0000);
        assert_eq!(layout.children[1].location.x, 0.0000);
        assert_eq!(layout.children[1].location.y, 45.0000);

        assert_eq!(layout.children[2].size.width, 100.0000);
        assert_eq!(layout.children[2].size.height, 10.0000);
        assert_eq!(layout.children[2].location.x, 0.0000);
        assert_eq!(layout.children[2].location.y, 90.0000);
    }

    #[test]
    fn flex_grow_within_constrained_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(300.0000),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 200.0000);
        assert_eq!(layout.children[0].children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn flex_basis_smaller_than_content_row() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_basis: stretch::style::Dimension::Points(50.0000),
                children: vec![stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 100.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].children[0].size.height, 100.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn max_width_overrides_width() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 100.0000);
        assert_eq!(layout.size.height, 0.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 100.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);
    }

    #[test]
    fn percentage_position_bottom_right() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5500),
                    height: stretch::style::Dimension::Percent(0.1500),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Percent(0.2000),
                    bottom: stretch::style::Dimension::Percent(0.1000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 275.0000);
        assert_eq!(layout.children[0].size.height, 75.0000);
        assert_eq!(layout.children[0].location.x, -100.0000);
        assert_eq!(layout.children[0].location.y, -50.0000);
    }

    #[test]
    fn margin_auto_top_stretching_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    margin: stretch::geometry::Rect { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 200.0000);
        assert_eq!(layout.size.height, 200.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 150.0000);
        assert_eq!(layout.children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 200.0000);

        assert_eq!(layout.children[1].size.width, 50.0000);
        assert_eq!(layout.children[1].size.height, 50.0000);
        assert_eq!(layout.children[1].location.x, 150.0000);
        assert_eq!(layout.children[1].location.y, 75.0000);
    }

    #[test]
    fn align_flex_start_with_shrinking_children() {
        let layout = stretch::compute(&stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexStart,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    children: vec![stretch::style::Node {
                        flex_grow: 1.0000,
                        flex_shrink: 1.0000,
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 500.0000);
        assert_eq!(layout.size.height, 500.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].size.height, 500.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].location.y, 0.0000);

        assert_eq!(layout.children[0].children[0].children[0].size.width, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].size.height, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].children[0].children[0].location.y, 0.0000);
    }

    #[test]
    fn justify_content_column_min_height_and_margin_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(50.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20.0000),
                    height: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    bottom: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.size.width, 20.0000);
        assert_eq!(layout.size.height, 50.0000);
        assert_eq!(layout.location.x, 0.0000);
        assert_eq!(layout.location.y, 0.0000);

        assert_eq!(layout.children[0].size.width, 20.0000);
        assert_eq!(layout.children[0].size.height, 20.0000);
        assert_eq!(layout.children[0].location.x, 0.0000);
        assert_eq!(layout.children[0].location.y, 10.0000);
    }

}
