pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Percent(1f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Percent(1f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Percent(1f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(60f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
