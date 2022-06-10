pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                padding: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Percent(0.1f32),
                    end: taffy::style::Dimension::Percent(0.1f32),
                    top: taffy::style::Dimension::Percent(0.1f32),
                    bottom: taffy::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
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
