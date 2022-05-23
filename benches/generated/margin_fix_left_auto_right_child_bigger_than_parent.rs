pub fn compute() {
    let mut stretch = sprawl::Stretch::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(72f32),
                    height: sprawl::style::Dimension::Points(72f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(10f32),
                    end: sprawl::style::Dimension::Auto,
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                justify_content: sprawl::style::JustifyContent::Center,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(52f32),
                    height: sprawl::style::Dimension::Points(52f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
