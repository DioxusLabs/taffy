pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            &stretch::node::Node::new(
                stretch::style::Style {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.5f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        height: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        bottom: stretch::style::Dimension::Percent(0.5f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![],
            ),
            &stretch::node::Node::new(
                stretch::style::Style {
                    position_type: stretch::style::PositionType::Absolute,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(10f32),
                        ..Default::default()
                    },
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Percent(0.1f32),
                        bottom: stretch::style::Dimension::Percent(0.1f32),
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
