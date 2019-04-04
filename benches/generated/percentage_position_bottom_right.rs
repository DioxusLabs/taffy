pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.55f32),
                    height: stretch::style::Dimension::Percent(0.15f32),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    end: stretch::style::Dimension::Percent(0.2f32),
                    bottom: stretch::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
