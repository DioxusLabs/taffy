pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 0.2f32,
                flex_shrink: 0f32,
                flex_basis: sprawl::style::Dimension::Points(40f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(sprawl::style::FlexboxLayout { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node2 = sprawl
        .new_node(sprawl::style::FlexboxLayout { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
