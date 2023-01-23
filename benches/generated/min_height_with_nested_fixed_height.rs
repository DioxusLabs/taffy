pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(40f32),
                height: taffy::style::Dimension::Points(40f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_self: Some(taffy::style::AlignSelf::FlexStart),
                flex_shrink: 0f32,
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(28f32) },
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Points(8f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(9f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(320f32), height: auto() },
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(44f32) },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(16f32),
                    right: taffy::style::LengthPercentage::Points(16f32),
                    top: zero(),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
