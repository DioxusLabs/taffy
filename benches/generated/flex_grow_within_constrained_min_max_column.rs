pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_node(taffy::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node1 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                min_size: taffy::geometry::Size {
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
