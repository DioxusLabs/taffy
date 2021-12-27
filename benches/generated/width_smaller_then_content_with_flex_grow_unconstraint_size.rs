pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(70f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Points(0f32), ..Default::default() },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node10 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(20f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Points(0f32), ..Default::default() },
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[node0, node1]).unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
