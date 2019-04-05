pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            align_content: stretch::style::AlignContent::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(700f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_shrink: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0f32),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100f32),
                        height: stretch::style::Dimension::Points(500f32),
                        ..Default::default()
                    },
                    max_size: stretch::geometry::Size {
                        height: stretch::style::Dimension::Points(200f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1f32,
                    flex_shrink: 1f32,
                    flex_basis: stretch::style::Dimension::Percent(0f32),
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(200f32),
                        height: stretch::style::Dimension::Points(200f32),
                        ..Default::default()
                    },
                    margin: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(20f32),
                        end: stretch::style::Dimension::Points(20f32),
                        top: stretch::style::Dimension::Points(20f32),
                        bottom: stretch::style::Dimension::Points(20f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(100f32),
                        height: stretch::style::Dimension::Points(100f32),
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
