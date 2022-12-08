pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::Points(0.3f32),
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(9.9f32) },
            position: taffy::geometry::Rect {
                left: auto(),
                right: auto(),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Points(13.3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 4f32,
            flex_basis: taffy::style::Dimension::Points(0.3f32),
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(1.1f32) },
            position: taffy::geometry::Rect {
                left: auto(),
                right: auto(),
                top: taffy::style::LengthPercentageAuto::Points(13.3f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 0.7f32,
                flex_basis: taffy::style::Dimension::Points(50.3f32),
                size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(20.3f32) },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1.6f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1.1f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(10.7f32) },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(87.4f32),
                    height: taffy::style::Dimension::Points(113.4f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
