pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Points(100f32),
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style { flex_basis: taffy::style::Dimension::Points(25f32), ..Default::default() })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style { flex_basis: taffy::style::Dimension::Points(25f32), ..Default::default() })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(101f32),
                    height: taffy::style::Dimension::Points(100f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
