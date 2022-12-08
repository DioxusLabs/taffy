#[test]
fn wrapped_column_max_height_flex() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(500f32),
            },
            max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(200f32) },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(200f32),
                height: taffy::style::Dimension::Points(200f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(20f32),
                right: taffy::style::LengthPercentageAuto::Points(20f32),
                top: taffy::style::LengthPercentageAuto::Points(20f32),
                bottom: taffy::style::LengthPercentageAuto::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(100f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_items: Some(taffy::style::AlignItems::Center),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(700f32),
                    height: taffy::style::Dimension::Points(500f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 700f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 500f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 180f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 300f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 180f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 250f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 200f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 300f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 400f32);
}
