pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node9 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node10 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node11 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node12 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node13 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node14 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node15 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32), auto(), points(40f32), auto()],
                grid_template_columns: vec![points(40f32), auto(), points(40f32), auto()],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                },
                ..Default::default()
            },
            &[
                node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13,
                node14, node15,
            ],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
