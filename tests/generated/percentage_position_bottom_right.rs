#[test]
fn percentage_position_bottom_right() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.55f32),
                height: taffy::style::Dimension::Percent(0.15f32),
            },
            position: taffy::geometry::Rect {
                left: auto(),
                right: taffy::style::LengthPercentageAuto::Percent(0.2f32),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
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
    assert_eq!(taffy.layout(node0).unwrap().size.width, 275f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 75f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, -100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, -50f32);
}
