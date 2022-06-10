pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Percent(0.3f32),
                    top: taffy::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
