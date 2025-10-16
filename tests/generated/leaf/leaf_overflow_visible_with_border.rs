#[test]
#[allow(non_snake_case)]
fn leaf_overflow_visible_with_border__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node_text = "HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH" ;
    let node = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(45f32),
                    height: taffy::style::Dimension::from_length(45f32),
                },
                border: taffy::geometry::Rect {
                    left: length(4f32),
                    right: length(2f32),
                    top: length(1f32),
                    bottom: length(3f32),
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
    if layout.scroll_width() != 171f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            171f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 29f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            29f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn leaf_overflow_visible_with_border__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node_text = "HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH\u{200b}HHHHHHHHHHHHHHHHHHHHH" ;
    let node = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(45f32),
                    height: taffy::style::Dimension::from_length(45f32),
                },
                border: taffy::geometry::Rect {
                    left: length(4f32),
                    right: length(2f32),
                    top: length(1f32),
                    bottom: length(3f32),
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
    if layout.size.width != 51f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 51f32, layout.size.width);
    }
    if layout.size.height != 49f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 49f32, layout.size.height);
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
    if layout.scroll_width() != 165f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            165f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 25f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            25f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
