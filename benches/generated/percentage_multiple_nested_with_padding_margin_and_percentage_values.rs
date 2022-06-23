pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), ..Default::default() },
                margin: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Percent(0.05f32)),
                    end: Some(taffy::style::Dimension::Percent(0.05f32)),
                    top: Some(taffy::style::Dimension::Percent(0.05f32)),
                    bottom: Some(taffy::style::Dimension::Percent(0.05f32)),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(3f32)),
                    end: Some(taffy::style::Dimension::Points(3f32)),
                    top: Some(taffy::style::Dimension::Points(3f32)),
                    bottom: Some(taffy::style::Dimension::Points(3f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), ..Default::default() },
                margin: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(5f32)),
                    end: Some(taffy::style::Dimension::Points(5f32)),
                    top: Some(taffy::style::Dimension::Points(5f32)),
                    bottom: Some(taffy::style::Dimension::Points(5f32)),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Percent(0.03f32)),
                    end: Some(taffy::style::Dimension::Percent(0.03f32)),
                    top: Some(taffy::style::Dimension::Percent(0.03f32)),
                    bottom: Some(taffy::style::Dimension::Percent(0.03f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0.1f32),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.6f32),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(5f32)),
                    end: Some(taffy::style::Dimension::Points(5f32)),
                    top: Some(taffy::style::Dimension::Points(5f32)),
                    bottom: Some(taffy::style::Dimension::Points(5f32)),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(3f32)),
                    end: Some(taffy::style::Dimension::Points(3f32)),
                    top: Some(taffy::style::Dimension::Points(3f32)),
                    bottom: Some(taffy::style::Dimension::Points(3f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 4f32,
                flex_basis: taffy::style::Dimension::Percent(0.15f32),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.2f32),
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
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
