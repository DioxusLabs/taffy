pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(113f32),
                    height: taffy::style::Dimension::Points(100f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
