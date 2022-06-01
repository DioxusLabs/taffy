pub fn compute() {
    let mut stretch = sprawl::Sprawl::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style { position_type: sprawl::style::PositionType::Absolute, ..Default::default() },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
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
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
