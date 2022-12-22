#[test]
fn gap_row_gap_row_wrap_child_margins() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Points(2f32),
                bottom: taffy::style::LengthPercentageAuto::Points(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Points(10f32),
                bottom: taffy::style::LengthPercentageAuto::Points(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Points(15f32),
                bottom: taffy::style::LengthPercentageAuto::Points(15f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                gap: taffy::geometry::Size { width: zero(), height: taffy::style::LengthPercentage::Points(10f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(200f32),
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
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 200f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 42f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 2f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 42f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 66f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 42f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 143f32);
}
