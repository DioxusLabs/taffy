pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    end: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
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
