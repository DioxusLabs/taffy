pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                padding: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(10f32),
                    right: taffy::style::Dimension::Points(10f32),
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
