#[test]
#[allow(non_snake_case)]
fn grid_relayout_vertical_text__border_box() {
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
            crate::TestNodeContext::ahem_text(
                "HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH",
                crate::WritingMode::Vertical,
            ),
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
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                grid_template_rows: vec![length(40f32)],
                grid_template_columns: vec![min_content()],
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 60f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 60f32, layout.size.height);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 40f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn grid_relayout_vertical_text__content_box() {
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
            crate::TestNodeContext::ahem_text(
                "HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH",
                crate::WritingMode::Vertical,
            ),
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
            crate::TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH", crate::WritingMode::Horizontal),
        )
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
                grid_template_rows: vec![length(40f32)],
                grid_template_columns: vec![min_content()],
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 60f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 60f32, layout.size.height);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 40f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
