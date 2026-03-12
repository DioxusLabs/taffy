#[test]
#[allow(non_snake_case)]
fn flex_grow_0_min_size__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("one", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("two", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("three", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("four", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(100f32),
                },
                border: taffy::geometry::Rect {
                    left: length(1f32),
                    right: length(1f32),
                    top: length(1f32),
                    bottom: length(1f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node0, 30f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node0, 98f32, size.height);
    assert_eq!(location.x, 1f32, "x of node {:?}. Expected {}. Actual {}", node0, 1f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node1, 98f32, size.height);
    assert_eq!(location.x, 31f32, "x of node {:?}. Expected {}. Actual {}", node1, 31f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node1, 1f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node2, 98f32, size.height);
    assert_eq!(location.x, 61f32, "x of node {:?}. Expected {}. Actual {}", node2, 61f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node2, 1f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node3, 98f32, size.height);
    assert_eq!(location.x, 111f32, "x of node {:?}. Expected {}. Actual {}", node3, 111f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node3, 1f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn flex_grow_0_min_size__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("one", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("two", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("three", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("four", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(100f32),
                },
                border: taffy::geometry::Rect {
                    left: length(1f32),
                    right: length(1f32),
                    top: length(1f32),
                    bottom: length(1f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 402f32, "width of node {:?}. Expected {}. Actual {}", node, 402f32, size.width);
    assert_eq!(size.height, 102f32, "height of node {:?}. Expected {}. Actual {}", node, 102f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node0, 30f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0, 100f32, size.height);
    assert_eq!(location.x, 1f32, "x of node {:?}. Expected {}. Actual {}", node0, 1f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node1, 100f32, size.height);
    assert_eq!(location.x, 31f32, "x of node {:?}. Expected {}. Actual {}", node1, 31f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node1, 1f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node2, 100f32, size.height);
    assert_eq!(location.x, 61f32, "x of node {:?}. Expected {}. Actual {}", node2, 61f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node2, 1f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node3, 100f32, size.height);
    assert_eq!(location.x, 111f32, "x of node {:?}. Expected {}. Actual {}", node3, 111f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node3, 1f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn flex_grow_0_min_size__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("one", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("two", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("three", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("four", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                direction: taffy::style::Direction::Rtl,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(100f32),
                },
                border: taffy::geometry::Rect {
                    left: length(1f32),
                    right: length(1f32),
                    top: length(1f32),
                    bottom: length(1f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node0, 30f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node0, 98f32, size.height);
    assert_eq!(location.x, 369f32, "x of node {:?}. Expected {}. Actual {}", node0, 369f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node1, 98f32, size.height);
    assert_eq!(location.x, 339f32, "x of node {:?}. Expected {}. Actual {}", node1, 339f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node1, 1f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node2, 98f32, size.height);
    assert_eq!(location.x, 289f32, "x of node {:?}. Expected {}. Actual {}", node2, 289f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node2, 1f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 98f32, "height of node {:?}. Expected {}. Actual {}", node3, 98f32, size.height);
    assert_eq!(location.x, 249f32, "x of node {:?}. Expected {}. Actual {}", node3, 249f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node3, 1f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn flex_grow_0_min_size__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("one", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("two", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("three", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                flex_grow: 0f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::from_percent(0f32),
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("four", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(100f32),
                },
                border: taffy::geometry::Rect {
                    left: length(1f32),
                    right: length(1f32),
                    top: length(1f32),
                    bottom: length(1f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 402f32, "width of node {:?}. Expected {}. Actual {}", node, 402f32, size.width);
    assert_eq!(size.height, 102f32, "height of node {:?}. Expected {}. Actual {}", node, 102f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node0, 30f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0, 100f32, size.height);
    assert_eq!(location.x, 371f32, "x of node {:?}. Expected {}. Actual {}", node0, 371f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node1, 100f32, size.height);
    assert_eq!(location.x, 341f32, "x of node {:?}. Expected {}. Actual {}", node1, 341f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node1, 1f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node2, 100f32, size.height);
    assert_eq!(location.x, 291f32, "x of node {:?}. Expected {}. Actual {}", node2, 291f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node2, 1f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node3, 100f32, size.height);
    assert_eq!(location.x, 251f32, "x of node {:?}. Expected {}. Actual {}", node3, 251f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node3, 1f32, location.y);
}
