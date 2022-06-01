pub fn compute() {
    let mut stretch = sprawl::Sprawl::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(0f32),
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch.new_node(sprawl::style::Style { ..Default::default() }, &[node0]).unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
