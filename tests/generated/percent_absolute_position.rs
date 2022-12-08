#[test]
fn percent_absolute_position() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Points(50f32),
                },
                position: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Percent(0.5f32),
                    right: auto(),
                    top: auto(),
                    bottom: auto(),
                },
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
                    width: taffy::style::Dimension::Points(60f32),
                    height: taffy::style::Dimension::Points(50f32),
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
    assert_eq!(taffy.layout(node).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node01).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node01).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node01).unwrap().location.y, 0f32);
}
