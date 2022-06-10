pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl.new_node(sprawl::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                display: sprawl::style::Display::None,
                flex_grow: 1f32,
                position: sprawl::geometry::Rect { top: sprawl::style::Dimension::Points(10f32), ..Default::default() },
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
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
