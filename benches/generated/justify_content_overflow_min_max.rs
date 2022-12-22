pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                justify_content: Some(taffy::style::JustifyContent::Center),
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(100f32) },
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(110f32) },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
