pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_shrink: 0f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(40f32),
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
                flex_shrink: 0f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(40f32),
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
                flex_shrink: 0f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(20f32),
                    height: taffy::style::Dimension::Points(40f32),
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
                gap: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.2f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
