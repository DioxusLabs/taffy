pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                justify_content: sprawl::style::JustifyContent::SpaceEvenly,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
