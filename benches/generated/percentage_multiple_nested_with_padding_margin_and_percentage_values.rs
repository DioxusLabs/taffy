pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Points(3f32),
                right: taffy::style::LengthPercentage::Points(3f32),
                top: taffy::style::LengthPercentage::Points(3f32),
                bottom: taffy::style::LengthPercentage::Points(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(5f32),
                    right: taffy::style::LengthPercentageAuto::Points(5f32),
                    top: taffy::style::LengthPercentageAuto::Points(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Percent(0.03f32),
                    right: taffy::style::LengthPercentage::Percent(0.03f32),
                    top: taffy::style::LengthPercentage::Percent(0.03f32),
                    bottom: taffy::style::LengthPercentage::Percent(0.03f32),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0.1f32),
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.6f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(5f32),
                    right: taffy::style::LengthPercentageAuto::Points(5f32),
                    top: taffy::style::LengthPercentageAuto::Points(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(3f32),
                    right: taffy::style::LengthPercentage::Points(3f32),
                    top: taffy::style::LengthPercentage::Points(3f32),
                    bottom: taffy::style::LengthPercentage::Points(3f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 4f32,
            flex_basis: taffy::style::Dimension::Percent(0.15f32),
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.2f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
