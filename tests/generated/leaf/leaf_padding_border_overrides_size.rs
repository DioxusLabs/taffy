#[test]
#[allow(non_snake_case)]
fn leaf_padding_border_overrides_size__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
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
        })
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 22f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 22f32, layout.size.width);
    }
    if layout.size.height != 14f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 14f32, layout.size.height);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn leaf_padding_border_overrides_size__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
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
        })
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 34f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 34f32, layout.size.width);
    }
    if layout.size.height != 26f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 26f32, layout.size.height);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
