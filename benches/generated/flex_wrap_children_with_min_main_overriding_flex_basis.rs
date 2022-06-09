pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_with_children(
            sprawl::style::Style {
                flex_basis: sprawl::style::Dimension::Points(50f32),
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_with_children(
            sprawl::style::Style {
                flex_basis: sprawl::style::Dimension::Points(50f32),
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_with_children(
            sprawl::style::Style {
                flex_wrap: sprawl::style::FlexWrap::Wrap,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
