#[test]
#[allow(non_snake_case)]
fn content_size_with_border__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node5 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node6 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node7 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node8 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node9 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node10 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node11 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node12 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node13 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node14 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node15 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node16 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node17 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node18 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node19 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                overflow: taffy::geometry::Point { x: taffy::style::Overflow::Clip, y: taffy::style::Overflow::Scroll },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(100f32),
                    height: taffy::style::Dimension::from_length(50f32),
                },
                border: taffy::geometry::Rect {
                    left: length(7f32),
                    right: length(3f32),
                    top: length(1f32),
                    bottom: length(5f32),
                },
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
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 100f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 50f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 156f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            156f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 1f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.y), 1f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node1).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 11f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 11f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node2).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 21f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node2,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node2,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node3).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 31f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 31f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node3,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node3,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node4).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 41f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.y), 41f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node4,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node4,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node5).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 51f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.y), 51f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node5,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node5,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node6).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 61f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.y), 61f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node6,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node6,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node7).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 71f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.y), 71f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node7,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node7,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node8).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 81f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.y), 81f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node8,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node8,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node9).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 91f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.y), 91f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node9,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node9,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 101f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.y), 101f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node11).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 111f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.y), 111f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node12).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 121f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 121f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node13).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 131f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.y), 131f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node14).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 141f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.y), 141f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node15).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 151f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.y), 151f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node16).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 161f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.y), 161f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node17).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 171f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.y), 171f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node18).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 181f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.y), 181f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node19).unwrap();
    if layout.size.width != 75f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(size.width), 75f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 191f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.y), 191f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 125f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(scroll_width()),
            125f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn content_size_with_border__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node1 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node3 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node5 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node6 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node7 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node8 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node9 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node10 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node11 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node12 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node13 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node14 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node15 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node16 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node17 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node18 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node19 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("XXXXXXXXXXXXXXXXXXXX", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                overflow: taffy::geometry::Point { x: taffy::style::Overflow::Clip, y: taffy::style::Overflow::Scroll },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(100f32),
                    height: taffy::style::Dimension::from_length(50f32),
                },
                border: taffy::geometry::Rect {
                    left: length(7f32),
                    right: length(3f32),
                    top: length(1f32),
                    bottom: length(5f32),
                },
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
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 110f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 110f32, layout.size.width);
    }
    if layout.size.height != 56f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 56f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 150f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            150f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 1f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.y), 1f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node1).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 11f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 11f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node2).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 21f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node2,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node2,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node3).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 31f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 31f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node3,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node3,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node4).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 41f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.y), 41f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node4,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node4,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node5).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 51f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.y), 51f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node5,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node5,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node6).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 61f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.y), 61f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node6,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node6,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node7).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 71f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.y), 71f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node7,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node7,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node8).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 81f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.y), 81f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node8,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node8,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node9).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.height), 10f32, layout.size.height);
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 91f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.y), 91f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node9,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node9,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 101f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.y), 101f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node11).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 111f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.y), 111f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node12).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 121f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 121f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node13).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 131f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.y), 131f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node14).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 141f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.y), 141f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node15).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 151f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.y), 151f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node16).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 161f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.y), 161f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node17).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 171f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.y), 171f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node18).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 181f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.y), 181f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node19).unwrap();
    if layout.size.width != 85f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(size.width), 85f32, layout.size.width);
    }
    if layout.size.height != 10f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(size.height),
            10f32,
            layout.size.height
        );
    }
    if layout.location.x != 7f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.x), 7f32, layout.location.x);
    }
    if layout.location.y != 191f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.y), 191f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 115f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(scroll_width()),
            115f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
