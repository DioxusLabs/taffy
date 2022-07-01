pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(70f32),
                    height: taffy::style::Dimension::Points(100f32),
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
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node10 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(100f32),
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
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
