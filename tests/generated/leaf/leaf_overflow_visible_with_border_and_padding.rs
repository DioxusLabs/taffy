#[test]
#[allow(non_snake_case)]
fn leaf_overflow_visible_with_border_and_padding__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node_text = "HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH" ;
    let node = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(45f32),
                    height: taffy::style::Dimension::from_length(45f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(4f32),
                    top: length(2f32),
                    bottom: length(6f32),
                },
                border: taffy::geometry::Rect {
                    left: length(7f32),
                    right: length(3f32),
                    top: length(1f32),
                    bottom: length(5f32),
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text(node_text, crate::WritingMode::Horizontal),
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 45f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 45f32, layout.size.width);
    }
    if layout.size.height != 45f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 45f32, layout.size.height);
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
    if layout.scroll_width() != 183f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            183f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 33f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            33f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn leaf_overflow_visible_with_border_and_padding__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node_text = "HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH" ;
    let node = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(45f32),
                    height: taffy::style::Dimension::from_length(45f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(8f32),
                    right: length(4f32),
                    top: length(2f32),
                    bottom: length(6f32),
                },
                border: taffy::geometry::Rect {
                    left: length(7f32),
                    right: length(3f32),
                    top: length(1f32),
                    bottom: length(5f32),
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text(node_text, crate::WritingMode::Horizontal),
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 67f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 67f32, layout.size.width);
    }
    if layout.size.height != 59f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 59f32, layout.size.height);
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
    if layout.scroll_width() != 161f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            161f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 19f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            19f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
