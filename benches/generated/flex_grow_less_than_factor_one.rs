pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.2f32,
                    flex_shrink: 0f32,
                    flex_basis: stretch::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() },
                stretch::style::Node { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
