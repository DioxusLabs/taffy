pub fn compute() {
    let mut stretch = sprawl::Stretch::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(31f32),
                    height: sprawl::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(32f32),
                    height: sprawl::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(33f32),
                    height: sprawl::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(34f32),
                    height: sprawl::style::Dimension::Points(30f32),
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
                flex_wrap: sprawl::style::FlexWrap::Wrap,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
