#[test]
fn absolute_layout_within_border() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
            },
            position: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(0f32),
                right: auto(),
                top: taffy::style::LengthPercentageAuto::Points(0f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
            },
            position: taffy::geometry::Rect {
                left: auto(),
                right: taffy::style::LengthPercentageAuto::Points(0f32),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Points(0f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(10f32),
                right: taffy::style::LengthPercentageAuto::Points(10f32),
                top: taffy::style::LengthPercentageAuto::Points(10f32),
                bottom: taffy::style::LengthPercentageAuto::Points(10f32),
            },
            position: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(0f32),
                right: auto(),
                top: taffy::style::LengthPercentageAuto::Points(0f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(10f32),
                right: taffy::style::LengthPercentageAuto::Points(10f32),
                top: taffy::style::LengthPercentageAuto::Points(10f32),
                bottom: taffy::style::LengthPercentageAuto::Points(10f32),
            },
            position: taffy::geometry::Rect {
                left: auto(),
                right: taffy::style::LengthPercentageAuto::Points(0f32),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Points(0f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(10f32),
                    right: taffy::style::LengthPercentage::Points(10f32),
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(10f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(10f32),
                    right: taffy::style::LengthPercentage::Points(10f32),
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(10f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 40f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 20f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 20f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 30f32);
}
