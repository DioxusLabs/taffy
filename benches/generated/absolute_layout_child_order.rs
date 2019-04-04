pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        height: stretch::style::Dimension::Points(40f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        height: stretch::style::Dimension::Points(40f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(60f32),
                        height: stretch::style::Dimension::Points(40f32),
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
