pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(0.3f32),
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(9.9f32), ..Default::default() },
                position: sprawl::geometry::Rect {
                    bottom: sprawl::style::Dimension::Points(13.3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 4f32,
                flex_basis: sprawl::style::Dimension::Points(0.3f32),
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(1.1f32), ..Default::default() },
                position: sprawl::geometry::Rect {
                    top: sprawl::style::Dimension::Points(13.3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 0.7f32,
                flex_basis: sprawl::style::Dimension::Points(50.3f32),
                size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(20.3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 1.6f32,
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 1.1f32,
                size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(10.7f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(87.4f32),
                    height: sprawl::style::Dimension::Points(113.4f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
