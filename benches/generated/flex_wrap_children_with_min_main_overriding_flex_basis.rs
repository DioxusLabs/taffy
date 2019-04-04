pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            children: vec![
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50f32),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(55f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50f32),
                    size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(50f32),
                        ..Default::default()
                    },
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(55f32),
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
