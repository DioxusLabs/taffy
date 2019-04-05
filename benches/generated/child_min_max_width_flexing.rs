pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(120f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_shrink: 0f32,
                    flex_basis: stretch::style::Dimension::Points(0f32),
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_shrink: 0f32,
                    flex_basis: stretch::style::Dimension::Percent(0.5f32),
                    max_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
