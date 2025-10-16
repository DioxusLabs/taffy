#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_moderate__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    taffy.disable_rounding();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(0.45f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: percent(0.05f32),
                right: percent(0.05f32),
                top: percent(0.05f32),
                bottom: percent(0.05f32),
            },
            padding: taffy::geometry::Rect {
                left: length(3f32),
                right: length(3f32),
                top: length(3f32),
                bottom: length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(0.5f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: length(5f32),
                    right: length(5f32),
                    top: length(5f32),
                    bottom: length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: percent(0.03f32),
                    right: percent(0.03f32),
                    top: percent(0.03f32),
                    bottom: percent(0.03f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(200f32), height: auto() },
                padding: taffy::geometry::Rect {
                    left: length(3f32),
                    right: length(3f32),
                    top: length(3f32),
                    bottom: length(3f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if (layout.size.width - 200f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 200f32, layout.size.width);
    }
    if (layout.size.height - 42.15625f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(size.height),
            42.15625f32,
            layout.size.height
        );
    }
    if (layout.location.x - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.x), 0f32, layout.location.x);
    }
    if (layout.location.y - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_width() - 0f32).abs() >= 0.1 {
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
    if (layout.scroll_height() - 0f32).abs() >= 0.1 {
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
    if (layout.size.width - 97f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 97f32, layout.size.width);
    }
    if (layout.size.height - 26.15625f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(size.height),
            26.15625f32,
            layout.size.height
        );
    }
    if (layout.location.x - 8f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.x), 8f32, layout.location.x);
    }
    if (layout.location.y - 8f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.y), 8f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_width() - 0f32).abs() >= 0.1 {
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
    if (layout.scroll_height() - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00).unwrap();
    if (layout.size.width - 38.40625f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(size.width),
            38.40625f32,
            layout.size.width
        );
    }
    if (layout.size.height - 6f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.height), 6f32, layout.size.height);
    }
    if (layout.location.x - 10.078125f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(location.x),
            10.078125f32,
            layout.location.x
        );
    }
    if (layout.location.y - 10.078125f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(location.y),
            10.078125f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_width() - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(scroll_width()),
            0f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_height() - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_moderate__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    taffy.disable_rounding();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(0.45f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: percent(0.05f32),
                right: percent(0.05f32),
                top: percent(0.05f32),
                bottom: percent(0.05f32),
            },
            padding: taffy::geometry::Rect {
                left: length(3f32),
                right: length(3f32),
                top: length(3f32),
                bottom: length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(0.5f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: length(5f32),
                    right: length(5f32),
                    top: length(5f32),
                    bottom: length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: percent(0.03f32),
                    right: percent(0.03f32),
                    top: percent(0.03f32),
                    bottom: percent(0.03f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(200f32), height: auto() },
                padding: taffy::geometry::Rect {
                    left: length(3f32),
                    right: length(3f32),
                    top: length(3f32),
                    bottom: length(3f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if (layout.size.width - 206f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 206f32, layout.size.width);
    }
    if (layout.size.height - 44f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 44f32, layout.size.height);
    }
    if (layout.location.x - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.x), 0f32, layout.location.x);
    }
    if (layout.location.y - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_width() - 0f32).abs() >= 0.1 {
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
    if (layout.scroll_height() - 0f32).abs() >= 0.1 {
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
    if (layout.size.width - 112f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 112f32, layout.size.width);
    }
    if (layout.size.height - 28f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 28f32, layout.size.height);
    }
    if (layout.location.x - 8f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.x), 8f32, layout.location.x);
    }
    if (layout.location.y - 8f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(location.y), 8f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_width() - 0f32).abs() >= 0.1 {
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
    if (layout.scroll_height() - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00).unwrap();
    if (layout.size.width - 51f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.width), 51f32, layout.size.width);
    }
    if (layout.size.height - 6f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.height), 6f32, layout.size.height);
    }
    if (layout.location.x - 11f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.x), 11f32, layout.location.x);
    }
    if (layout.location.y - 11f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.y), 11f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_width() - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(scroll_width()),
            0f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if (layout.scroll_height() - 0f32).abs() >= 0.1 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
