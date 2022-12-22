pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::Points(200f32),
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(100f32) },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(100f32) },
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(500f32) },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(100f32) },
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(500f32) },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
