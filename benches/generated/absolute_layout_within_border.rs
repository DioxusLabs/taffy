pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(0f32)),
                    top: Some(taffy::style::Dimension::Points(0f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    end: Some(taffy::style::Dimension::Points(0f32)),
                    bottom: Some(taffy::style::Dimension::Points(0f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(10f32)),
                    end: Some(taffy::style::Dimension::Points(10f32)),
                    top: Some(taffy::style::Dimension::Points(10f32)),
                    bottom: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(0f32)),
                    top: Some(taffy::style::Dimension::Points(0f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(50f32)),
                    height: Some(taffy::style::Dimension::Points(50f32)),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(10f32)),
                    end: Some(taffy::style::Dimension::Points(10f32)),
                    top: Some(taffy::style::Dimension::Points(10f32)),
                    bottom: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    end: Some(taffy::style::Dimension::Points(0f32)),
                    bottom: Some(taffy::style::Dimension::Points(0f32)),
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
                    width: Some(taffy::style::Dimension::Points(100f32)),
                    height: Some(taffy::style::Dimension::Points(100f32)),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(10f32)),
                    end: Some(taffy::style::Dimension::Points(10f32)),
                    top: Some(taffy::style::Dimension::Points(10f32)),
                    bottom: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                border: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(10f32)),
                    end: Some(taffy::style::Dimension::Points(10f32)),
                    top: Some(taffy::style::Dimension::Points(10f32)),
                    bottom: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
