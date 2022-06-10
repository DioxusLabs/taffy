pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Percent(0.1f32),
                max_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.6f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 4f32,
                flex_basis: sprawl::style::Dimension::Percent(0.15f32),
                max_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.2f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(200f32),
                    height: sprawl::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
