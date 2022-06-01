pub fn compute() {
    let mut stretch = sprawl::Sprawl::new();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                border: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(10f32),
                    end: sprawl::style::Dimension::Points(10f32),
                    top: sprawl::style::Dimension::Points(10f32),
                    bottom: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
