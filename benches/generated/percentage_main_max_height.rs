pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style { flex_basis: taffy::style::Dimension::Points(15f32), ..Default::default() })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            flex_basis: taffy::style::Dimension::Points(48f32),
            max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Percent(0.33f32) },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Start),
                size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(151f32) },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(71f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
