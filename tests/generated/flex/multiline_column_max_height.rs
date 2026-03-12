#[test]
#[allow(non_snake_case)]
fn multiline_column_max_height__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node9 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node16 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node17 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node18 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node19 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::from_length(200f32) },
                ..Default::default()
            },
            &[
                node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13,
                node14, node15, node16, node17, node18, node19,
            ],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node, 80f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node2, 40f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node3, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node3, 60f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node4, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node4, 80f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node5, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node5, 0f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node5, 100f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node6, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node6, 0f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node6, 120f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node7, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node7, 0f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node7, 140f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node8, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node8, 0f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node8, 160f32, location.y);
    let layout = taffy.layout(node9).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node9, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node9, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node9, 0f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node9, 180f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node10, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node10, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node10, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node11, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node11, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node11, 40f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node12, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node12, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node12, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node12, 40f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node13, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node13, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node13, 40f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node13, 60f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node14, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node14, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node14, 40f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node14, 80f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node15, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node15, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node15, 40f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node15, 100f32, location.y);
    let layout = taffy.layout(node16).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node16, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node16, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node16, 40f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node16, 120f32, location.y);
    let layout = taffy.layout(node17).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node17, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node17, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node17, 40f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node17, 140f32, location.y);
    let layout = taffy.layout(node18).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node18, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node18, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node18, 40f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node18, 160f32, location.y);
    let layout = taffy.layout(node19).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node19, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node19, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node19, 40f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node19, 180f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn multiline_column_max_height__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node9 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node16 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node17 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node18 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node19 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::from_length(200f32) },
                ..Default::default()
            },
            &[
                node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13,
                node14, node15, node16, node17, node18, node19,
            ],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node, 80f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node2, 40f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node3, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node3, 60f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node4, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node4, 80f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node5, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node5, 0f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node5, 100f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node6, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node6, 0f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node6, 120f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node7, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node7, 0f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node7, 140f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node8, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node8, 0f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node8, 160f32, location.y);
    let layout = taffy.layout(node9).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node9, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node9, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node9, 0f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node9, 180f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node10, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node10, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node10, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node11, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node11, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node11, 40f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node12, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node12, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node12, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node12, 40f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node13, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node13, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node13, 40f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node13, 60f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node14, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node14, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node14, 40f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node14, 80f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node15, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node15, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node15, 40f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node15, 100f32, location.y);
    let layout = taffy.layout(node16).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node16, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node16, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node16, 40f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node16, 120f32, location.y);
    let layout = taffy.layout(node17).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node17, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node17, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node17, 40f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node17, 140f32, location.y);
    let layout = taffy.layout(node18).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node18, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node18, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node18, 40f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node18, 160f32, location.y);
    let layout = taffy.layout(node19).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node19, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node19, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node19, 40f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node19, 180f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn multiline_column_max_height__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node9 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node16 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node17 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node18 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node19 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::from_length(200f32) },
                ..Default::default()
            },
            &[
                node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13,
                node14, node15, node16, node17, node18, node19,
            ],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node, 80f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node2, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node2, 40f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node3, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node3, 40f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node3, 60f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node4, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node4, 40f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node4, 80f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node5, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node5, 40f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node5, 100f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node6, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node6, 40f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node6, 120f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node7, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node7, 40f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node7, 140f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node8, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node8, 40f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node8, 160f32, location.y);
    let layout = taffy.layout(node9).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node9, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node9, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node9, 40f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node9, 180f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node10, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node10, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node11, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node11, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node11, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node12, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node12, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node12, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node12, 40f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node13, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node13, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node13, 0f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node13, 60f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node14, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node14, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node14, 0f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node14, 80f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node15, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node15, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node15, 0f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node15, 100f32, location.y);
    let layout = taffy.layout(node16).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node16, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node16, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node16, 0f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node16, 120f32, location.y);
    let layout = taffy.layout(node17).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node17, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node17, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node17, 0f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node17, 140f32, location.y);
    let layout = taffy.layout(node18).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node18, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node18, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node18, 0f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node18, 160f32, location.y);
    let layout = taffy.layout(node19).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node19, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node19, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node19, 0f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node19, 180f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn multiline_column_max_height__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node9 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node16 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node17 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node18 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node19 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::from_length(200f32) },
                ..Default::default()
            },
            &[
                node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13,
                node14, node15, node16, node17, node18, node19,
            ],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node, 80f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node2, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node2, 40f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node3, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node3, 40f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node3, 60f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node4, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node4, 40f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node4, 80f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node5, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node5, 40f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node5, 100f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node6, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node6, 40f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node6, 120f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node7, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node7, 40f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node7, 140f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node8, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node8, 40f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node8, 160f32, location.y);
    let layout = taffy.layout(node9).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node9, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node9, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node9, 40f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node9, 180f32, location.y);
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node10, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node10, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node11, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node11, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node11, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node12, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node12, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node12, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node12, 40f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node13, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node13, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node13, 0f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node13, 60f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node14, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node14, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node14, 0f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node14, 80f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node15, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node15, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node15, 0f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node15, 100f32, location.y);
    let layout = taffy.layout(node16).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node16, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node16, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node16, 0f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node16, 120f32, location.y);
    let layout = taffy.layout(node17).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node17, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node17, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node17, 0f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node17, 140f32, location.y);
    let layout = taffy.layout(node18).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node18, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node18, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node18, 0f32, location.x);
    assert_eq!(location.y, 160f32, "y of node {:?}. Expected {}. Actual {}", node18, 160f32, location.y);
    let layout = taffy.layout(node19).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node19, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node19, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node19, 0f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node19, 180f32, location.y);
}
