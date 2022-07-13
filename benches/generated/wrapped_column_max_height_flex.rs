pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(100f32)),
                    height: Some(taffy::style::Dimension::Points(500f32)),
                    ..Default::default()
                },
                max_size: taffy::geometry::Size {
                    height: taffy::style::Dimension::Points(200f32),
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
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(200f32)),
                    height: Some(taffy::style::Dimension::Points(200f32)),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(20f32)),
                    end: Some(taffy::style::Dimension::Points(20f32)),
                    top: Some(taffy::style::Dimension::Points(20f32)),
                    bottom: Some(taffy::style::Dimension::Points(20f32)),
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
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(100f32)),
                    height: Some(taffy::style::Dimension::Points(100f32)),
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
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_items: taffy::style::AlignItems::Center,
                align_content: taffy::style::AlignContent::Center,
                justify_content: taffy::style::JustifyContent::Center,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(700f32)),
                    height: Some(taffy::style::Dimension::Points(500f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
