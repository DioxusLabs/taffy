pub fn compute() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_shrink: 1f32,
                flex_basis: sprawl::style::Dimension::Points(100f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                max_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(200f32), ..Default::default() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
