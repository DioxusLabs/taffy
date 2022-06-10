pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[])
        .unwrap();
    let node0 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node00]).unwrap();
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
