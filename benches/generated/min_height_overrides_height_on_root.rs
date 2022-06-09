pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
