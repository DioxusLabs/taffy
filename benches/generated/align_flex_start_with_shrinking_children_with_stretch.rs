pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style { align_items: stretch::style::AlignItems::FlexStart, ..Default::default() },
            vec![&stretch::node::Node::new(
                stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
                vec![&stretch::node::Node::new(
                    stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
                    vec![],
                )],
            )],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
