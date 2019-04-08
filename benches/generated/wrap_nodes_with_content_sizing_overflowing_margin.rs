pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(85f32), ..Default::default() },
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        flex_direction: stretch::style::FlexDirection::Column,
                        ..Default::default()
                    },
                    vec![&stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40f32),
                                height: stretch::style::Dimension::Points(40f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    )],
                ),
                &stretch::node::Node::new(
                    stretch::style::Style {
                        flex_direction: stretch::style::FlexDirection::Column,
                        margin: stretch::geometry::Rect {
                            end: stretch::style::Dimension::Points(10f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![&stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(40f32),
                                height: stretch::style::Dimension::Points(40f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    )],
                ),
            ],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
