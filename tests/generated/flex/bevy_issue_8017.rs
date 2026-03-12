#[test]
#[allow(non_snake_case)]
fn bevy_issue_8017__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node10, node11],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(400f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(8f32),
                    top: length(8f32),
                    bottom: length(8f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 400f32, "height of node {:?}. Expected {}. Actual {}", node, 400f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 384f32, "width of node {:?}. Expected {}. Actual {}", node0, 384f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node0, 188f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node00, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node00, 188f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node01, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node01, 188f32, size.height);
    assert_eq!(location.x, 196f32, "x of node {:?}. Expected {}. Actual {}", node01, 196f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 384f32, "width of node {:?}. Expected {}. Actual {}", node1, 384f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node1, 188f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node1, 8f32, location.x);
    assert_eq!(location.y, 204f32, "y of node {:?}. Expected {}. Actual {}", node1, 204f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node10, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node10, 188f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node11, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node11, 188f32, size.height);
    assert_eq!(location.x, 196f32, "x of node {:?}. Expected {}. Actual {}", node11, 196f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node11, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_8017__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node10, node11],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(400f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(8f32),
                    top: length(8f32),
                    bottom: length(8f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 416f32, "width of node {:?}. Expected {}. Actual {}", node, 416f32, size.width);
    assert_eq!(size.height, 416f32, "height of node {:?}. Expected {}. Actual {}", node, 416f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node0, 400f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node0, 196f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node00, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node00, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node01, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node01, 196f32, size.height);
    assert_eq!(location.x, 204f32, "x of node {:?}. Expected {}. Actual {}", node01, 204f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node1, 400f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node1, 196f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node1, 8f32, location.x);
    assert_eq!(location.y, 212f32, "y of node {:?}. Expected {}. Actual {}", node1, 212f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node10, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node10, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node11, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node11, 196f32, size.height);
    assert_eq!(location.x, 204f32, "x of node {:?}. Expected {}. Actual {}", node11, 204f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node11, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_8017__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                direction: taffy::style::Direction::Rtl,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                direction: taffy::style::Direction::Rtl,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node10, node11],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(400f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(8f32),
                    top: length(8f32),
                    bottom: length(8f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 400f32, "height of node {:?}. Expected {}. Actual {}", node, 400f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 384f32, "width of node {:?}. Expected {}. Actual {}", node0, 384f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node0, 188f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node00, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node00, 188f32, size.height);
    assert_eq!(location.x, 196f32, "x of node {:?}. Expected {}. Actual {}", node00, 196f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node01, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node01, 188f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node01, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 384f32, "width of node {:?}. Expected {}. Actual {}", node1, 384f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node1, 188f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node1, 8f32, location.x);
    assert_eq!(location.y, 204f32, "y of node {:?}. Expected {}. Actual {}", node1, 204f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node10, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node10, 188f32, size.height);
    assert_eq!(location.x, 196f32, "x of node {:?}. Expected {}. Actual {}", node10, 196f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node11, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node11, 188f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node11, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node11, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_8017__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1f32),
                    height: taffy::style::Dimension::from_percent(0.5f32),
                },
                ..Default::default()
            },
            &[node10, node11],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                gap: taffy::geometry::Size { width: length(8f32), height: length(8f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(400f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(8f32),
                    top: length(8f32),
                    bottom: length(8f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 416f32, "width of node {:?}. Expected {}. Actual {}", node, 416f32, size.width);
    assert_eq!(size.height, 416f32, "height of node {:?}. Expected {}. Actual {}", node, 416f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node0, 400f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node0, 196f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node00, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node00, 196f32, size.height);
    assert_eq!(location.x, 204f32, "x of node {:?}. Expected {}. Actual {}", node00, 204f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node01, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node01, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node01, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node1, 400f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node1, 196f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node1, 8f32, location.x);
    assert_eq!(location.y, 212f32, "y of node {:?}. Expected {}. Actual {}", node1, 212f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node10, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node10, 196f32, size.height);
    assert_eq!(location.x, 204f32, "x of node {:?}. Expected {}. Actual {}", node10, 204f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 196f32, "width of node {:?}. Expected {}. Actual {}", node11, 196f32, size.width);
    assert_eq!(size.height, 196f32, "height of node {:?}. Expected {}. Actual {}", node11, 196f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node11, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node11, 0f32, location.y);
}
