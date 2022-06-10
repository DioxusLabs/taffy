pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), ..Default::default() },
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
