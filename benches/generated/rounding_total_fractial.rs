pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 0.7f32,
            flex_basis: taffy::style::Dimension::Points(50.3f32),
            size: taffy::geometry::Size { height: taffy::style::Dimension::Points(20.3f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 1.6f32,
            size: taffy::geometry::Size { height: taffy::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node2 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 1.1f32,
            size: taffy::geometry::Size { height: taffy::style::Dimension::Points(10.7f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(87.4f32),
                height: taffy::style::Dimension::Points(113.4f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0, node1, node2],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
