pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                border: sprawl::geometry::Rect {
                    main_start: sprawl::style::Dimension::Points(10f32),
                    main_end: sprawl::style::Dimension::Points(10f32),
                    cross_start: sprawl::style::Dimension::Points(10f32),
                    cross_end: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
