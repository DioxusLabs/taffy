pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50f32),
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
