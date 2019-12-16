pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node = stretch
        .new_node(
            stretch::style::Style {
                border: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10f32),
                    end: stretch::style::Dimension::Points(10f32),
                    top: stretch::style::Dimension::Points(10f32),
                    bottom: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
