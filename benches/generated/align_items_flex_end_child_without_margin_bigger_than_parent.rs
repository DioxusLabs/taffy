pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(70f32),
                    height: sprawl::style::Dimension::Points(70f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style { align_items: sprawl::style::AlignItems::FlexEnd, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                align_items: sprawl::style::AlignItems::Center,
                justify_content: sprawl::style::JustifyContent::Center,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
