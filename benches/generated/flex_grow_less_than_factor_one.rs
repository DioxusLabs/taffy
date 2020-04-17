pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 0.2f32,
                flex_shrink: 0f32,
                flex_basis: stretch::style::Dimension::Points(40f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(stretch::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node2 = stretch
        .new_node(stretch::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(500f32),
                    height: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
