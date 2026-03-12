#[test]
#[allow(non_snake_case)]
fn absolute_child_with_cross_margin__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(28f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                position: taffy::style::Position::Absolute,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_grow: 0f32,
                flex_shrink: 1f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_length(15f32),
                },
                margin: taffy::geometry::Rect { left: zero(), right: zero(), top: length(4f32), bottom: zero() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(25f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(0f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(36893500000000000000f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node, 311f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node0, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node1, 311f32, size.width);
    assert_eq!(size.height, 15f32, "height of node {:?}. Expected {}. Actual {}", node1, 15f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 4f32, "y of node {:?}. Expected {}. Actual {}", node1, 4f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node2, 25f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node2, 27f32, size.height);
    assert_eq!(location.x, 286f32, "x of node {:?}. Expected {}. Actual {}", node2, 286f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn absolute_child_with_cross_margin__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(28f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                position: taffy::style::Position::Absolute,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_grow: 0f32,
                flex_shrink: 1f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_length(15f32),
                },
                margin: taffy::geometry::Rect { left: zero(), right: zero(), top: length(4f32), bottom: zero() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(25f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(0f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(36893500000000000000f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node, 311f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node0, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node1, 311f32, size.width);
    assert_eq!(size.height, 15f32, "height of node {:?}. Expected {}. Actual {}", node1, 15f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 4f32, "y of node {:?}. Expected {}. Actual {}", node1, 4f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node2, 25f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node2, 27f32, size.height);
    assert_eq!(location.x, 286f32, "x of node {:?}. Expected {}. Actual {}", node2, 286f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn absolute_child_with_cross_margin__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(28f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                position: taffy::style::Position::Absolute,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_grow: 0f32,
                flex_shrink: 1f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_length(15f32),
                },
                margin: taffy::geometry::Rect { left: zero(), right: zero(), top: length(4f32), bottom: zero() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(25f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(0f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(36893500000000000000f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node, 311f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node0, 27f32, size.height);
    assert_eq!(location.x, 283f32, "x of node {:?}. Expected {}. Actual {}", node0, 283f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node1, 311f32, size.width);
    assert_eq!(size.height, 15f32, "height of node {:?}. Expected {}. Actual {}", node1, 15f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 4f32, "y of node {:?}. Expected {}. Actual {}", node1, 4f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node2, 25f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node2, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn absolute_child_with_cross_margin__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(28f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                position: taffy::style::Position::Absolute,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_grow: 0f32,
                flex_shrink: 1f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_length(15f32),
                },
                margin: taffy::geometry::Rect { left: zero(), right: zero(), top: length(4f32), bottom: zero() },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(25f32),
                height: taffy::style::Dimension::from_length(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(0f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(311f32),
                    height: taffy::style::Dimension::from_length(36893500000000000000f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node, 311f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node0, 27f32, size.height);
    assert_eq!(location.x, 283f32, "x of node {:?}. Expected {}. Actual {}", node0, 283f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node1, 311f32, size.width);
    assert_eq!(size.height, 15f32, "height of node {:?}. Expected {}. Actual {}", node1, 15f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 4f32, "y of node {:?}. Expected {}. Actual {}", node1, 4f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node2, 25f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node2, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}
