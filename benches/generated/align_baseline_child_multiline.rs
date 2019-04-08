pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Baseline,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        height: stretch::style::Dimension::Points(60f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_wrap: stretch::style::FlexWrap::Wrap,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25f32),
                                height: stretch::style::Dimension::Points(20f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25f32),
                                height: stretch::style::Dimension::Points(10f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25f32),
                                height: stretch::style::Dimension::Points(20f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(25f32),
                                height: stretch::style::Dimension::Points(10f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            ),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
