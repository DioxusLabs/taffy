pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    flex_grow: 1f32,
                    flex_shrink: 1f32,
                    children: vec![stretch::style::Node { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
