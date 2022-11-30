#[test]
fn gap_column_gap_wrap_align_stretch() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node4 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Size::auto() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_content: Some(taffy::style::AlignContent::Stretch),
                gap: taffy::geometry::Size { width: taffy::style::LengthPercentage::Points(5f32), ..Size::zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(300f32),
                    height: taffy::style::Dimension::Points(300f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 300f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 300f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 76f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 153f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 229f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node4).unwrap().size.width, 300f32);
    assert_eq!(taffy.layout(node4).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node4).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node4).unwrap().location.y, 150f32);
}
