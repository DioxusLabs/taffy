pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(taffy::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]);
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            min_size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
