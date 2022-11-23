pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.1f32),
                    height: taffy::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.1f32),
                    height: taffy::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: taffy::style::AlignItems::FlexStart,
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
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
