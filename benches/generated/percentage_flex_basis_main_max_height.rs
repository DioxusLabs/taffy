pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0.1f32),
            max_size: taffy::geometry::Size { height: taffy::style::Dimension::Percent(0.6f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 4f32,
            flex_basis: taffy::style::Dimension::Percent(0.1f32),
            max_size: taffy::geometry::Size { height: taffy::style::Dimension::Percent(0.2f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(200f32),
                height: taffy::style::Dimension::Points(400f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
