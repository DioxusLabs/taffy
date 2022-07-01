pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Points(100f32),
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout { flex_basis: taffy::style::Dimension::Points(25f32), ..Default::default() },
        &[],
    );
    let node2 = taffy.new_with_children(
        taffy::style::FlexboxLayout { flex_basis: taffy::style::Dimension::Points(25f32), ..Default::default() },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(101f32),
                height: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0, node1, node2],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
