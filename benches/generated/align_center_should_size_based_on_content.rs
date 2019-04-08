pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0f32,
                flex_shrink: 1f32,
                ..Default::default()
            },
            vec![&stretch::node::Node::new(
                stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
                vec![&stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(20f32),
                            height: stretch::style::Dimension::Points(20f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![],
                )],
            )],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
