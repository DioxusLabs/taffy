#[test]
fn margin_fix_left_auto_right_child_bigger_than_parent() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(72f32),
                height: taffy::style::Dimension::Points(72f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(10f32),
                right: taffy::style::LengthPercentageAuto::Auto,
                top: zero(),
                bottom: zero(),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                justify_content: Some(taffy::style::JustifyContent::Center),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(52f32),
                    height: taffy::style::Dimension::Points(52f32),
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
    assert_eq!(taffy.layout(node).unwrap().size.width, 52f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 52f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 42f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 72f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
}
