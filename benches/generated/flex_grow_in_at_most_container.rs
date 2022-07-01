pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::Points(0f32),
            ..Default::default()
        },
        &[],
    );
    let node0 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node00]);
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            align_items: taffy::style::AlignItems::FlexStart,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
