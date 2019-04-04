pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(20f32),
                    end: stretch::style::Dimension::Points(20f32),
                    top: stretch::style::Dimension::Points(20f32),
                    bottom: stretch::style::Dimension::Points(20f32),
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
