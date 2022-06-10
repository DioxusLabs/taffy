pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_basis: sprawl::style::Dimension::Points(50f32),
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node0]).unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
