pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_basis: stretch2::style::Dimension::Points(50f32),
                size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[node0]).unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
