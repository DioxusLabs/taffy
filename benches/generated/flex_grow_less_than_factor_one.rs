pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 0.2f32,
                flex_shrink: 0f32,
                flex_basis: stretch2::style::Dimension::Points(40f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(stretch2::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node2 = stretch
        .new_node(stretch2::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(500f32),
                    height: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
