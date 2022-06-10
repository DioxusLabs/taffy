pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.5f32),
                    height: sprawl::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout { flex_direction: sprawl::style::FlexDirection::Column, ..Default::default() },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
