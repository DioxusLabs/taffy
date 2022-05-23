pub fn compute() {
    let mut stretch = sprawl::Stretch::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_shrink: 0f32,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                flex_shrink: 0f32,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            sprawl::style::Style {
                flex_shrink: 0f32,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                justify_content: sprawl::style::JustifyContent::Center,
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(110f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
