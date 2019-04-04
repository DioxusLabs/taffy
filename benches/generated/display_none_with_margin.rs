pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(20f32),
                        height: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(10f32),
                        end: stretch::style::Dimension::Points(10f32),
                        top: stretch::style::Dimension::Points(10f32),
                        bottom: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1f32, ..Default::default() },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}
