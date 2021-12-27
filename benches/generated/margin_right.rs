pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size { width: stretch2::style::Dimension::Points(10f32), ..Default::default() },
                margin: stretch2::geometry::Rect { end: stretch2::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                justify_content: stretch2::style::JustifyContent::FlexEnd,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
