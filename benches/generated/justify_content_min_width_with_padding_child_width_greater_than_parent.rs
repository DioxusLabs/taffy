pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(300f32),
                    height: taffy::style::Dimension::Points(100f32),
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
                justify_content: taffy::style::JustifyContent::Center,
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(100f32),
                    end: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node00]).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(1000f32),
                    height: taffy::style::Dimension::Points(1584f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
