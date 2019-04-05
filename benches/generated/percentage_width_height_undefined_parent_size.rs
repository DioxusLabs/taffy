pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5f32),
                    height: stretch::style::Dimension::Percent(0.5f32),
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
