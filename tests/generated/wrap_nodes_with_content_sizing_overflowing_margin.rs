#[test]
fn wrap_nodes_with_content_sizing_overflowing_margin() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(40f32),
                height: taffy::style::Dimension::Points(40f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
            &[node000],
        )
        .unwrap();
    let node010 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(40f32),
                height: taffy::style::Dimension::Points(40f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: taffy::style::LengthPercentageAuto::Points(10f32),
                    top: zero(),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node010],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(85f32), height: auto() },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(500f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 500f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 500f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 85f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 80f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node000).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node01).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node01).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().location.y, 40f32);
    assert_eq!(taffy.layout(node010).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node010).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node010).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node010).unwrap().location.y, 0f32);
}
