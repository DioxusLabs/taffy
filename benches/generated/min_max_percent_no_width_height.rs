pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexStart,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.1f32),
                    height: stretch::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.1f32),
                    height: stretch::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
