pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                padding: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
                    end: stretch2::style::Dimension::Points(10f32),
                    top: stretch2::style::Dimension::Points(10f32),
                    bottom: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
