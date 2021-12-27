pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node000 = stretch
        .new_node(stretch2::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[])
        .unwrap();
    let node00 = stretch
        .new_node(stretch2::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[node000])
        .unwrap();
    let node0 = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[node00]).unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(500f32),
                    height: stretch2::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
