pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_shrink: 1f32,
                flex_basis: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style { flex_basis: stretch::style::Dimension::Points(25f32), ..Default::default() },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch::style::Style { flex_basis: stretch::style::Dimension::Points(25f32), ..Default::default() },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(101f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
