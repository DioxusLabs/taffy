pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 1f32,
                flex_basis: stretch2::style::Dimension::Points(0.3f32),
                size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(9.9f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    bottom: stretch2::style::Dimension::Points(13.3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 4f32,
                flex_basis: stretch2::style::Dimension::Points(0.3f32),
                size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(1.1f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    top: stretch2::style::Dimension::Points(13.3f32),
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
                flex_grow: 0.7f32,
                flex_basis: stretch2::style::Dimension::Points(50.3f32),
                size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(20.3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 1.6f32,
                size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 1.1f32,
                size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(10.7f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(87.4f32),
                    height: stretch2::style::Dimension::Points(113.4f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
