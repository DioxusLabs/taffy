#[test]
fn rounding_total_fractial_nested() {
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
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 87f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 113f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 87f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 59f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 87f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 12f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, -13f32);
    assert_eq!(taffy.layout(node01).unwrap().size.width, 87f32);
    assert_eq!(taffy.layout(node01).unwrap().size.height, 47f32);
    assert_eq!(taffy.layout(node01).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().location.y, 25f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 87f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 30f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 59f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 87f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 24f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 89f32);
}
