#[test]
#[allow(non_snake_case)]
fn grid_min_content_flex_single_item_margin_percent__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(10f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: percent(0.2f32),
                right: percent(0.1f32),
                top: percent(0.05f32),
                bottom: percent(0.15f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                grid_column: taffy::geometry::Line {
                    start: taffy::style::GridPlacement::Span(2u16),
                    end: taffy::style::GridPlacement::Auto,
                },
                margin: taffy::geometry::Rect {
                    left: percent(0.2f32),
                    right: percent(0.1f32),
                    top: percent(0.05f32),
                    bottom: percent(0.15f32),
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), min_content(), fr(1f32)],
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 80f32, layout.size.width);
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 120f32, layout.size.height);
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
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            0f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 10f32, layout.size.width);
    }
    if layout.size.height != 38f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 38f32, layout.size.height);
    }
    if layout.location.x != 42f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 42f32, layout.location.x);
    }
    if layout.location.y != 1f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 1f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node2,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 40f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node3,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 28f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.width), 28f32, layout.size.width);
    }
    if layout.size.height != 32f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.height), 32f32, layout.size.height);
    }
    if layout.location.x != 48f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.x), 48f32, layout.location.x);
    }
    if layout.location.y != 42f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.y), 42f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node4,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.y), 80f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node5,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.width), 10f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.y), 80f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node6,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.y), 80f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node7,
            stringify!(scroll_width()),
            0f32,
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn grid_min_content_flex_single_item_margin_percent__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(10f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: percent(0.2f32),
                right: percent(0.1f32),
                top: percent(0.05f32),
                bottom: percent(0.15f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                grid_column: taffy::geometry::Line {
                    start: taffy::style::GridPlacement::Span(2u16),
                    end: taffy::style::GridPlacement::Auto,
                },
                margin: taffy::geometry::Rect {
                    left: percent(0.2f32),
                    right: percent(0.1f32),
                    top: percent(0.05f32),
                    bottom: percent(0.15f32),
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text("HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), min_content(), fr(1f32)],
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 80f32, layout.size.width);
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 120f32, layout.size.height);
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
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            0f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 10f32, layout.size.width);
    }
    if layout.size.height != 38f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 38f32, layout.size.height);
    }
    if layout.location.x != 42f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 42f32, layout.location.x);
    }
    if layout.location.y != 1f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 1f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node2,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 40f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node3,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 28f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.width), 28f32, layout.size.width);
    }
    if layout.size.height != 32f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.height), 32f32, layout.size.height);
    }
    if layout.location.x != 48f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.x), 48f32, layout.location.x);
    }
    if layout.location.y != 42f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.y), 42f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node4,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.y), 80f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node5,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 10f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.width), 10f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.y), 80f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node6,
            stringify!(scroll_width()),
            0f32,
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.height), 40f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.y), 80f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node7,
            stringify!(scroll_width()),
            0f32,
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
