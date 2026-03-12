#[test]
#[allow(non_snake_case)]
fn gap_column_gap_wrap_align_stretch__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_content: Some(taffy::style::AlignContent::Stretch),
                gap: taffy::geometry::Size { width: length(5f32), height: zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(300f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node, 300f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node0, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node0, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node1, 72f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node1, 150f32, size.height);
    assert_eq!(location.x, 76f32, "x of node {:?}. Expected {}. Actual {}", node1, 76f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node2, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node2, 150f32, size.height);
    assert_eq!(location.x, 153f32, "x of node {:?}. Expected {}. Actual {}", node2, 153f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node3, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node3, 150f32, size.height);
    assert_eq!(location.x, 229f32, "x of node {:?}. Expected {}. Actual {}", node3, 229f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node4, 300f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node4, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 150f32, "y of node {:?}. Expected {}. Actual {}", node4, 150f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gap_column_gap_wrap_align_stretch__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_content: Some(taffy::style::AlignContent::Stretch),
                gap: taffy::geometry::Size { width: length(5f32), height: zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(300f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node, 300f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node0, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node0, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node1, 72f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node1, 150f32, size.height);
    assert_eq!(location.x, 76f32, "x of node {:?}. Expected {}. Actual {}", node1, 76f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node2, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node2, 150f32, size.height);
    assert_eq!(location.x, 153f32, "x of node {:?}. Expected {}. Actual {}", node2, 153f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node3, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node3, 150f32, size.height);
    assert_eq!(location.x, 229f32, "x of node {:?}. Expected {}. Actual {}", node3, 229f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node4, 300f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node4, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 150f32, "y of node {:?}. Expected {}. Actual {}", node4, 150f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gap_column_gap_wrap_align_stretch__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_content: Some(taffy::style::AlignContent::Stretch),
                gap: taffy::geometry::Size { width: length(5f32), height: zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(300f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node, 300f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node0, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node0, 150f32, size.height);
    assert_eq!(location.x, 229f32, "x of node {:?}. Expected {}. Actual {}", node0, 229f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node1, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node1, 150f32, size.height);
    assert_eq!(location.x, 153f32, "x of node {:?}. Expected {}. Actual {}", node1, 153f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node2, 72f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node2, 150f32, size.height);
    assert_eq!(location.x, 76f32, "x of node {:?}. Expected {}. Actual {}", node2, 76f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node3, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node3, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node4, 300f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node4, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 150f32, "y of node {:?}. Expected {}. Actual {}", node4, 150f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gap_column_gap_wrap_align_stretch__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(60f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_content: Some(taffy::style::AlignContent::Stretch),
                gap: taffy::geometry::Size { width: length(5f32), height: zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(300f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node, 300f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node0, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node0, 150f32, size.height);
    assert_eq!(location.x, 229f32, "x of node {:?}. Expected {}. Actual {}", node0, 229f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node1, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node1, 150f32, size.height);
    assert_eq!(location.x, 153f32, "x of node {:?}. Expected {}. Actual {}", node1, 153f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node2, 72f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node2, 150f32, size.height);
    assert_eq!(location.x, 76f32, "x of node {:?}. Expected {}. Actual {}", node2, 76f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 71f32, "width of node {:?}. Expected {}. Actual {}", node3, 71f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node3, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 300f32, "width of node {:?}. Expected {}. Actual {}", node4, 300f32, size.width);
    assert_eq!(size.height, 150f32, "height of node {:?}. Expected {}. Actual {}", node4, 150f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 150f32, "y of node {:?}. Expected {}. Actual {}", node4, 150f32, location.y);
}
