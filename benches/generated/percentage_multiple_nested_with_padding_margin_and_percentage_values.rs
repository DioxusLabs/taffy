pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0.1f32),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.6f32),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(5f32),
                        end: stretch::style::Dimension::Points(5f32),
                        top: stretch::style::Dimension::Points(5f32),
                        bottom: stretch::style::Dimension::Points(5f32),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(3f32),
                        end: stretch::style::Dimension::Points(3f32),
                        top: stretch::style::Dimension::Points(3f32),
                        bottom: stretch::style::Dimension::Points(3f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![&stretch::node::Node::new(
                    stretch::style::Style {
                        flex_direction: stretch::style::FlexDirection::Column,
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Percent(0.5f32),
                            ..Default::default()
                        },
                        margin: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Points(5f32),
                            end: stretch::style::Dimension::Points(5f32),
                            top: stretch::style::Dimension::Points(5f32),
                            bottom: stretch::style::Dimension::Points(5f32),
                            ..Default::default()
                        },
                        padding: stretch::geometry::Rect {
                            start: stretch::style::Dimension::Percent(0.03f32),
                            end: stretch::style::Dimension::Percent(0.03f32),
                            top: stretch::style::Dimension::Percent(0.03f32),
                            bottom: stretch::style::Dimension::Percent(0.03f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![&stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Percent(0.45f32),
                                ..Default::default()
                            },
                            margin: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Percent(0.05f32),
                                end: stretch::style::Dimension::Percent(0.05f32),
                                top: stretch::style::Dimension::Percent(0.05f32),
                                bottom: stretch::style::Dimension::Percent(0.05f32),
                                ..Default::default()
                            },
                            padding: stretch::geometry::Rect {
                                start: stretch::style::Dimension::Points(3f32),
                                end: stretch::style::Dimension::Points(3f32),
                                top: stretch::style::Dimension::Points(3f32),
                                bottom: stretch::style::Dimension::Points(3f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    )],
                )],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 4f32,
                    flex_basis: stretch::style::Dimension::Percent(0.15f32),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(0.2f32),
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
