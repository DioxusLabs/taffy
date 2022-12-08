#[test]
fn align_baseline_child_multiline() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(60f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), height: auto() },
                ..Default::default()
            },
            &[node10, node11, node12, node13],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Baseline),
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 80f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 60f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 40f32);
    assert_eq!(taffy.layout(node10).unwrap().size.width, 25f32);
    assert_eq!(taffy.layout(node10).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node10).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node10).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node11).unwrap().size.width, 25f32);
    assert_eq!(taffy.layout(node11).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node11).unwrap().location.x, 25f32);
    assert_eq!(taffy.layout(node11).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node12).unwrap().size.width, 25f32);
    assert_eq!(taffy.layout(node12).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node12).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node12).unwrap().location.y, 20f32);
    assert_eq!(taffy.layout(node13).unwrap().size.width, 25f32);
    assert_eq!(taffy.layout(node13).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node13).unwrap().location.x, 25f32);
    assert_eq!(taffy.layout(node13).unwrap().location.y, 20f32);
}
