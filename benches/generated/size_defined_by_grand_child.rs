pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node0 = stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node00]).unwrap();
    let node = stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node0]).unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
