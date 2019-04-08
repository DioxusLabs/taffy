pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(87.4f32),
                height: stretch::style::Dimension::Points(113.4f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 0.7f32,
                    flex_basis: stretch::style::Dimension::Points(50.3f32),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(20.3f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            flex_grow: 1f32,
                            flex_basis: stretch::style::Dimension::Points(0.3f32),
                            size: stretch::geometry::Size {
                                height: stretch::style::Dimension::Points(9.9f32),
                                ..Default::default()
                            },
                            position: stretch::geometry::Rect {
                                bottom: stretch::style::Dimension::Points(13.3f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            flex_grow: 4f32,
                            flex_basis: stretch::style::Dimension::Points(0.3f32),
                            size: stretch::geometry::Size {
                                height: stretch::style::Dimension::Points(1.1f32),
                                ..Default::default()
                            },
                            position: stretch::geometry::Rect {
                                top: stretch::style::Dimension::Points(13.3f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 1.6f32,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 1.1f32,
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(10.7f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
