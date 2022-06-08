pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
