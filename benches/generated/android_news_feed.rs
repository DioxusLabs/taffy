pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(120f32),
                height: taffy::style::Dimension::Points(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: taffy::style::LengthPercentageAuto::Points(36f32),
                    top: zero(),
                    bottom: zero(),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(36f32),
                    right: taffy::style::LengthPercentage::Points(36f32),
                    top: taffy::style::LengthPercentage::Points(21f32),
                    bottom: taffy::style::LengthPercentage::Points(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Start),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(36f32),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Points(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(72f32),
                height: taffy::style::Dimension::Points(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: taffy::style::LengthPercentageAuto::Points(36f32),
                    top: zero(),
                    bottom: zero(),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(36f32),
                    right: taffy::style::LengthPercentage::Points(36f32),
                    top: taffy::style::LengthPercentage::Points(21f32),
                    bottom: taffy::style::LengthPercentage::Points(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Start),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(174f32),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Points(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(1080f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
