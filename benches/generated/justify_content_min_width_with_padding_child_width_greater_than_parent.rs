pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(300f32),
                height: taffy::style::Dimension::Points(100f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                justify_content: Some(taffy::style::JustifyContent::Center),
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(400f32), height: auto() },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(100f32),
                    right: taffy::style::LengthPercentage::Points(100f32),
                    top: zero(),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { align_content: Some(taffy::style::AlignContent::Stretch), ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(1000f32),
                    height: taffy::style::Dimension::Points(1584f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
