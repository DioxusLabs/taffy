pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    height: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    bottom: stretch::style::Dimension::Points(10f32),
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
