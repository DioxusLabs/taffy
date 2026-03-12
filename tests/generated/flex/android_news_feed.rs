#[test]
#[allow(non_snake_case)]
fn android_news_feed__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(120f32),
                height: taffy::style::Dimension::from_length(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(36f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(72f32),
                height: taffy::style::Dimension::from_length(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(174f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(1080f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node0, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node0, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node00, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node00, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node000, 1080f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node000, 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node0000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1044f32, "width of node {:?}. Expected {}. Actual {}", node0000, 1044f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node0000, 120f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node0000, 36f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0000, 24f32, location.y);
    let layout = taffy.layout(node00000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00000, 0f32, location.y);
    let layout = taffy.layout(node000000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.y);
    let layout = taffy.layout(node00001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00001, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00001, 39f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node00001, 120f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00001, 0f32, location.y);
    let layout = taffy.layout(node000010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000010, 21f32, location.y);
    let layout = taffy.layout(node000011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000011, 21f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node001, 1080f32, size.width);
    assert_eq!(size.height, 96f32, "height of node {:?}. Expected {}. Actual {}", node001, 96f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001, 0f32, location.x);
    assert_eq!(location.y, 144f32, "y of node {:?}. Expected {}. Actual {}", node001, 144f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 906f32, "width of node {:?}. Expected {}. Actual {}", node0010, 906f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node0010, 72f32, size.height);
    assert_eq!(location.x, 174f32, "x of node {:?}. Expected {}. Actual {}", node0010, 174f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0010, 24f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node001000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00101, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00101, 39f32, size.height);
    assert_eq!(location.x, 72f32, "x of node {:?}. Expected {}. Actual {}", node00101, 72f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node001010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001010, 21f32, location.y);
    let layout = taffy.layout(node001011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001011, 21f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn android_news_feed__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(120f32),
                height: taffy::style::Dimension::from_length(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(36f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(72f32),
                height: taffy::style::Dimension::from_length(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(174f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(1080f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node0, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node0, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node00, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node00, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node000, 1080f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node000, 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node0000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1044f32, "width of node {:?}. Expected {}. Actual {}", node0000, 1044f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node0000, 120f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node0000, 36f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0000, 24f32, location.y);
    let layout = taffy.layout(node00000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00000, 0f32, location.y);
    let layout = taffy.layout(node000000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.y);
    let layout = taffy.layout(node00001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00001, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00001, 39f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node00001, 120f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00001, 0f32, location.y);
    let layout = taffy.layout(node000010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000010, 21f32, location.y);
    let layout = taffy.layout(node000011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000011, 21f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node001, 1080f32, size.width);
    assert_eq!(size.height, 96f32, "height of node {:?}. Expected {}. Actual {}", node001, 96f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001, 0f32, location.x);
    assert_eq!(location.y, 144f32, "y of node {:?}. Expected {}. Actual {}", node001, 144f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 906f32, "width of node {:?}. Expected {}. Actual {}", node0010, 906f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node0010, 72f32, size.height);
    assert_eq!(location.x, 174f32, "x of node {:?}. Expected {}. Actual {}", node0010, 174f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0010, 24f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node001000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00101, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00101, 39f32, size.height);
    assert_eq!(location.x, 72f32, "x of node {:?}. Expected {}. Actual {}", node00101, 72f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node001010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001010, 21f32, location.y);
    let layout = taffy.layout(node001011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001011, 21f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn android_news_feed__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(120f32),
                height: taffy::style::Dimension::from_length(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(36f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(72f32),
                height: taffy::style::Dimension::from_length(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(174f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(1080f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node0, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node0, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node00, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node00, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node000, 1080f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node000, 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node0000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1044f32, "width of node {:?}. Expected {}. Actual {}", node0000, 1044f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node0000, 120f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node0000, 36f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0000, 24f32, location.y);
    let layout = taffy.layout(node00000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.height);
    assert_eq!(location.x, 924f32, "x of node {:?}. Expected {}. Actual {}", node00000, 924f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00000, 0f32, location.y);
    let layout = taffy.layout(node000000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.y);
    let layout = taffy.layout(node00001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00001, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00001, 39f32, size.height);
    assert_eq!(location.x, 816f32, "x of node {:?}. Expected {}. Actual {}", node00001, 816f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00001, 0f32, location.y);
    let layout = taffy.layout(node000010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000010, 21f32, location.y);
    let layout = taffy.layout(node000011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000011, 21f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node001, 1080f32, size.width);
    assert_eq!(size.height, 96f32, "height of node {:?}. Expected {}. Actual {}", node001, 96f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001, 0f32, location.x);
    assert_eq!(location.y, 144f32, "y of node {:?}. Expected {}. Actual {}", node001, 144f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 906f32, "width of node {:?}. Expected {}. Actual {}", node0010, 906f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node0010, 72f32, size.height);
    assert_eq!(location.x, 174f32, "x of node {:?}. Expected {}. Actual {}", node0010, 174f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0010, 24f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.height);
    assert_eq!(location.x, 834f32, "x of node {:?}. Expected {}. Actual {}", node00100, 834f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node001000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00101, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00101, 39f32, size.height);
    assert_eq!(location.x, 726f32, "x of node {:?}. Expected {}. Actual {}", node00101, 726f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node001010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001010, 21f32, location.y);
    let layout = taffy.layout(node001011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001011, 21f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn android_news_feed__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(120f32),
                height: taffy::style::Dimension::from_length(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(36f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(72f32),
                height: taffy::style::Dimension::from_length(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(174f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(1080f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node0, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node0, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node00, 1080f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node00, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node000, 1080f32, size.width);
    assert_eq!(size.height, 144f32, "height of node {:?}. Expected {}. Actual {}", node000, 144f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node0000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1044f32, "width of node {:?}. Expected {}. Actual {}", node0000, 1044f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node0000, 120f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node0000, 36f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0000, 24f32, location.y);
    let layout = taffy.layout(node00000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node00000, 120f32, size.height);
    assert_eq!(location.x, 924f32, "x of node {:?}. Expected {}. Actual {}", node00000, 924f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00000, 0f32, location.y);
    let layout = taffy.layout(node000000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node000000, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000000, 0f32, location.y);
    let layout = taffy.layout(node00001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00001, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00001, 39f32, size.height);
    assert_eq!(location.x, 816f32, "x of node {:?}. Expected {}. Actual {}", node00001, 816f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00001, 0f32, location.y);
    let layout = taffy.layout(node000010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000010, 21f32, location.y);
    let layout = taffy.layout(node000011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node000011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node000011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node000011, 21f32, location.y);
    let layout = taffy.layout(node001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node001, 1080f32, size.width);
    assert_eq!(size.height, 96f32, "height of node {:?}. Expected {}. Actual {}", node001, 96f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001, 0f32, location.x);
    assert_eq!(location.y, 144f32, "y of node {:?}. Expected {}. Actual {}", node001, 144f32, location.y);
    let layout = taffy.layout(node0010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 906f32, "width of node {:?}. Expected {}. Actual {}", node0010, 906f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node0010, 72f32, size.height);
    assert_eq!(location.x, 174f32, "x of node {:?}. Expected {}. Actual {}", node0010, 174f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node0010, 24f32, location.y);
    let layout = taffy.layout(node00100).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node00100, 72f32, size.height);
    assert_eq!(location.x, 834f32, "x of node {:?}. Expected {}. Actual {}", node00100, 834f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00100, 0f32, location.y);
    let layout = taffy.layout(node001000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node001000, 72f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node001000, 0f32, location.y);
    let layout = taffy.layout(node00101).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node00101, 72f32, size.width);
    assert_eq!(size.height, 39f32, "height of node {:?}. Expected {}. Actual {}", node00101, 39f32, size.height);
    assert_eq!(location.x, 726f32, "x of node {:?}. Expected {}. Actual {}", node00101, 726f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00101, 0f32, location.y);
    let layout = taffy.layout(node001010).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001010, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001010, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001010, 21f32, location.y);
    let layout = taffy.layout(node001011).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node001011, 0f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node001011, 36f32, location.x);
    assert_eq!(location.y, 21f32, "y of node {:?}. Expected {}. Actual {}", node001011, 21f32, location.y);
}
