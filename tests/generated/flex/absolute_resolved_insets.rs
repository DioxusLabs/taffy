#[test]
#[allow(non_snake_case)]
fn absolute_resolved_insets__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node04 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node05 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Scroll,
                    y: taffy::style::Overflow::Scroll,
                },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node10, node11, node12, node13, node14, node15],
        )
        .unwrap();
    let node = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node0, node1]).unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node0, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node00, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node00, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node00, 35f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node00, 35f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node01, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node01, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node01, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node01, 20f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node02, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node02, 0f32, size.height);
    assert_eq!(location.x, 180f32, "x of node {:?}. Expected {}. Actual {}", node02, 180f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node02, 180f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node03, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node03, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node03, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node03, 20f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node04, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node04, 0f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node04, 50f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node04, 50f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node05, 160f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node05, 160f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node05, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node05, 20f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node1, 200f32, size.height);
    assert_eq!(location.x, 200f32, "x of node {:?}. Expected {}. Actual {}", node1, 200f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node10, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node10, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node10, 35f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node10, 35f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node11, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node11, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node11, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node12, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node12, 0f32, size.height);
    assert_eq!(location.x, 165f32, "x of node {:?}. Expected {}. Actual {}", node12, 165f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node12, 165f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node13, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node13, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node13, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node13, 20f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node14, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node14, 0f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node14, 50f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node14, 50f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 145f32, "width of node {:?}. Expected {}. Actual {}", node15, 145f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node15, 145f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node15, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node15, 20f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn absolute_resolved_insets__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node04 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node05 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Scroll,
                    y: taffy::style::Overflow::Scroll,
                },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node10, node11, node12, node13, node14, node15],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 540f32, "width of node {:?}. Expected {}. Actual {}", node, 540f32, size.width);
    assert_eq!(size.height, 270f32, "height of node {:?}. Expected {}. Actual {}", node, 270f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 270f32, "width of node {:?}. Expected {}. Actual {}", node0, 270f32, size.width);
    assert_eq!(size.height, 270f32, "height of node {:?}. Expected {}. Actual {}", node0, 270f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node00, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node00, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node00, 35f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node00, 35f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node01, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node01, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node01, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node01, 20f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node02, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node02, 0f32, size.height);
    assert_eq!(location.x, 250f32, "x of node {:?}. Expected {}. Actual {}", node02, 250f32, location.x);
    assert_eq!(location.y, 250f32, "y of node {:?}. Expected {}. Actual {}", node02, 250f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node03, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node03, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node03, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node03, 20f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node04, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node04, 0f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node04, 50f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node04, 50f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 230f32, "width of node {:?}. Expected {}. Actual {}", node05, 230f32, size.width);
    assert_eq!(size.height, 230f32, "height of node {:?}. Expected {}. Actual {}", node05, 230f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node05, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node05, 20f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 270f32, "width of node {:?}. Expected {}. Actual {}", node1, 270f32, size.width);
    assert_eq!(size.height, 270f32, "height of node {:?}. Expected {}. Actual {}", node1, 270f32, size.height);
    assert_eq!(location.x, 270f32, "x of node {:?}. Expected {}. Actual {}", node1, 270f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node10, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node10, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node10, 35f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node10, 35f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node11, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node11, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node11, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node12, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node12, 0f32, size.height);
    assert_eq!(location.x, 235f32, "x of node {:?}. Expected {}. Actual {}", node12, 235f32, location.x);
    assert_eq!(location.y, 235f32, "y of node {:?}. Expected {}. Actual {}", node12, 235f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node13, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node13, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node13, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node13, 20f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node14, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node14, 0f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node14, 50f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node14, 50f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 215f32, "width of node {:?}. Expected {}. Actual {}", node15, 215f32, size.width);
    assert_eq!(size.height, 215f32, "height of node {:?}. Expected {}. Actual {}", node15, 215f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node15, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node15, 20f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn absolute_resolved_insets__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node04 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node05 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                direction: taffy::style::Direction::Rtl,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                direction: taffy::style::Direction::Rtl,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Scroll,
                    y: taffy::style::Overflow::Scroll,
                },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node10, node11, node12, node13, node14, node15],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
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
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node0, 200f32, size.height);
    assert_eq!(location.x, 200f32, "x of node {:?}. Expected {}. Actual {}", node0, 200f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node00, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node00, 0f32, size.height);
    assert_eq!(location.x, 165f32, "x of node {:?}. Expected {}. Actual {}", node00, 165f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node00, 35f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node01, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node01, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node01, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node01, 20f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node02, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node02, 0f32, size.height);
    assert_eq!(location.x, 180f32, "x of node {:?}. Expected {}. Actual {}", node02, 180f32, location.x);
    assert_eq!(location.y, 180f32, "y of node {:?}. Expected {}. Actual {}", node02, 180f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node03, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node03, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node03, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node03, 20f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node04, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node04, 0f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node04, 50f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node04, 50f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node05, 160f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node05, 160f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node05, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node05, 20f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node1, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node10, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node10, 0f32, size.height);
    assert_eq!(location.x, 165f32, "x of node {:?}. Expected {}. Actual {}", node10, 165f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node10, 35f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node11, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node11, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node11, 35f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node12, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node12, 0f32, size.height);
    assert_eq!(location.x, 180f32, "x of node {:?}. Expected {}. Actual {}", node12, 180f32, location.x);
    assert_eq!(location.y, 165f32, "y of node {:?}. Expected {}. Actual {}", node12, 165f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node13, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node13, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node13, 35f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node13, 20f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node14, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node14, 0f32, size.height);
    assert_eq!(location.x, 65f32, "x of node {:?}. Expected {}. Actual {}", node14, 65f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node14, 50f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 145f32, "width of node {:?}. Expected {}. Actual {}", node15, 145f32, size.width);
    assert_eq!(size.height, 145f32, "height of node {:?}. Expected {}. Actual {}", node15, 145f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node15, 35f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node15, 20f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn absolute_resolved_insets__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node04 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node05 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::AUTO,
                right: auto(),
                top: taffy::style::LengthPercentageAuto::AUTO,
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: percent(1f32), right: auto(), top: percent(1f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: auto(), right: percent(1f32), top: auto(), bottom: percent(1f32) },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            inset: taffy::geometry::Rect { left: length(30f32), right: auto(), top: length(30f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_percent(1f32),
            },
            inset: taffy::geometry::Rect { left: length(0f32), right: auto(), top: length(0f32), bottom: auto() },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Scroll,
                    y: taffy::style::Overflow::Scroll,
                },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(15f32),
                    right: length(15f32),
                    top: length(15f32),
                    bottom: length(15f32),
                },
                border: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node10, node11, node12, node13, node14, node15],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
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
    assert_eq!(size.width, 540f32, "width of node {:?}. Expected {}. Actual {}", node, 540f32, size.width);
    assert_eq!(size.height, 270f32, "height of node {:?}. Expected {}. Actual {}", node, 270f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 270f32, "width of node {:?}. Expected {}. Actual {}", node0, 270f32, size.width);
    assert_eq!(size.height, 270f32, "height of node {:?}. Expected {}. Actual {}", node0, 270f32, size.height);
    assert_eq!(location.x, 270f32, "x of node {:?}. Expected {}. Actual {}", node0, 270f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node00, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node00, 0f32, size.height);
    assert_eq!(location.x, 235f32, "x of node {:?}. Expected {}. Actual {}", node00, 235f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node00, 35f32, location.y);
    let layout = taffy.layout(node01).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node01, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node01, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node01, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node01, 20f32, location.y);
    let layout = taffy.layout(node02).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node02, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node02, 0f32, size.height);
    assert_eq!(location.x, 250f32, "x of node {:?}. Expected {}. Actual {}", node02, 250f32, location.x);
    assert_eq!(location.y, 250f32, "y of node {:?}. Expected {}. Actual {}", node02, 250f32, location.y);
    let layout = taffy.layout(node03).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node03, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node03, 0f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node03, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node03, 20f32, location.y);
    let layout = taffy.layout(node04).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node04, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node04, 0f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node04, 50f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node04, 50f32, location.y);
    let layout = taffy.layout(node05).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 230f32, "width of node {:?}. Expected {}. Actual {}", node05, 230f32, size.width);
    assert_eq!(size.height, 230f32, "height of node {:?}. Expected {}. Actual {}", node05, 230f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node05, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node05, 20f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 270f32, "width of node {:?}. Expected {}. Actual {}", node1, 270f32, size.width);
    assert_eq!(size.height, 270f32, "height of node {:?}. Expected {}. Actual {}", node1, 270f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    let layout = taffy.layout(node10).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node10, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node10, 0f32, size.height);
    assert_eq!(location.x, 235f32, "x of node {:?}. Expected {}. Actual {}", node10, 235f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node10, 35f32, location.y);
    let layout = taffy.layout(node11).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node11, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node11, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node11, 35f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node11, 20f32, location.y);
    let layout = taffy.layout(node12).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node12, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node12, 0f32, size.height);
    assert_eq!(location.x, 250f32, "x of node {:?}. Expected {}. Actual {}", node12, 250f32, location.x);
    assert_eq!(location.y, 235f32, "y of node {:?}. Expected {}. Actual {}", node12, 235f32, location.y);
    let layout = taffy.layout(node13).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node13, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node13, 0f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node13, 35f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node13, 20f32, location.y);
    let layout = taffy.layout(node14).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node14, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node14, 0f32, size.height);
    assert_eq!(location.x, 65f32, "x of node {:?}. Expected {}. Actual {}", node14, 65f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node14, 50f32, location.y);
    let layout = taffy.layout(node15).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 215f32, "width of node {:?}. Expected {}. Actual {}", node15, 215f32, size.width);
    assert_eq!(size.height, 215f32, "height of node {:?}. Expected {}. Actual {}", node15, 215f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node15, 35f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node15, 20f32, location.y);
}
