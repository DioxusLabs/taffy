pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(40f32),
                    height: taffy::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = taffy
        .new_node(
            taffy::style::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
            &[node000],
        )
        .unwrap();
    let node010 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(40f32),
                    height: taffy::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                margin: taffy::geometry::Rect { top: taffy::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[node010],
        )
        .unwrap();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(70f32), ..Default::default() },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
