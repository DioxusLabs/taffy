pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                position_type: sprawl::style::PositionType::Absolute,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10f32),
                    height: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(10f32),
                    top: sprawl::style::Dimension::Points(10f32),
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
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
