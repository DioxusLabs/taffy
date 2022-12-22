pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(70f32),
                height: taffy::style::Dimension::Points(100f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(20f32),
                height: taffy::style::Dimension::Points(100f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
