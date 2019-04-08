pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
            vec![&stretch::node::Node::new(
                stretch::style::Style {
                    justify_content: stretch::style::JustifyContent::Center,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Percent(1f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50f32),
                                height: stretch::style::Dimension::Points(50f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style {
                            size: stretch::geometry::Size {
                                width: stretch::style::Dimension::Points(50f32),
                                height: stretch::style::Dimension::Points(50f32),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        vec![],
                    ),
                ],
            )],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
