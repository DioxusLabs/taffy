pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    flex_grow: 0.2f32,
                    flex_shrink: 0f32,
                    flex_basis: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() },
                vec![],
            ),
        ],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
