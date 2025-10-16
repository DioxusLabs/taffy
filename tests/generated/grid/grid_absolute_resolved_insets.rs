#[test]
#[allow(non_snake_case)]
fn grid_absolute_resolved_insets__border_box() {
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
                display: taffy::style::Display::Grid,
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
                display: taffy::style::Display::Grid,
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
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 400f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 400f32, layout.size.width);
    }
    if layout.size.height != 200f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 200f32, layout.size.height);
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
    if layout.size.width != 200f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 200f32, layout.size.width);
    }
    if layout.size.height != 200f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(size.height),
            200f32,
            layout.size.height
        );
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
    let layout = taffy.layout(node00).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
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
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node01).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node01,
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
            node01,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node02).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 180f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.x), 180f32, layout.location.x);
    }
    if layout.location.y != 180f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.y), 180f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node02,
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
            node02,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node03).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node03,
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
            node03,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node04).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(location.y), 50f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node04,
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
            node04,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node05).unwrap();
    if layout.size.width != 160f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node05, stringify!(size.width), 160f32, layout.size.width);
    }
    if layout.size.height != 160f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node05,
            stringify!(size.height),
            160f32,
            layout.size.height
        );
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node05, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node05, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node05,
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
            node05,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node1).unwrap();
    if layout.size.width != 200f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 200f32, layout.size.width);
    }
    if layout.size.height != 200f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(size.height),
            200f32,
            layout.size.height
        );
    }
    if layout.location.x != 200f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 200f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 0f32, layout.location.y);
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
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
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
            node10,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node11).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
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
            node11,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node12).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 165f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 165f32, layout.location.x);
    }
    if layout.location.y != 165f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 165f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
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
            node12,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node13).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
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
            node13,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node14).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.y), 50f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
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
            node14,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node15).unwrap();
    if layout.size.width != 145f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(size.width), 145f32, layout.size.width);
    }
    if layout.size.height != 145f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(size.height),
            145f32,
            layout.size.height
        );
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
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
            node15,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn grid_absolute_resolved_insets__content_box() {
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
                display: taffy::style::Display::Grid,
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
                display: taffy::style::Display::Grid,
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
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 540f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 540f32, layout.size.width);
    }
    if layout.size.height != 270f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 270f32, layout.size.height);
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
    if layout.size.width != 270f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 270f32, layout.size.width);
    }
    if layout.size.height != 270f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(size.height),
            270f32,
            layout.size.height
        );
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
    let layout = taffy.layout(node00).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
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
    if layout.scroll_height() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node01).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node01,
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
            node01,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node02).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 250f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.x), 250f32, layout.location.x);
    }
    if layout.location.y != 250f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.y), 250f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node02,
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
            node02,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node03).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node03,
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
            node03,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node04).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node04, stringify!(location.y), 50f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node04,
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
            node04,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node05).unwrap();
    if layout.size.width != 230f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node05, stringify!(size.width), 230f32, layout.size.width);
    }
    if layout.size.height != 230f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node05,
            stringify!(size.height),
            230f32,
            layout.size.height
        );
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node05, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node05, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node05,
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
            node05,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node1).unwrap();
    if layout.size.width != 270f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 270f32, layout.size.width);
    }
    if layout.size.height != 270f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(size.height),
            270f32,
            layout.size.height
        );
    }
    if layout.location.x != 270f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 270f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 0f32, layout.location.y);
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
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
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
            node10,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node11).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
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
            node11,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node12).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 235f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 235f32, layout.location.x);
    }
    if layout.location.y != 235f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 235f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
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
            node12,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node13).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
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
            node13,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node14).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.width), 0f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.height), 0f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.y), 50f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
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
            node14,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node15).unwrap();
    if layout.size.width != 215f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(size.width), 215f32, layout.size.width);
    }
    if layout.size.height != 215f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(size.height),
            215f32,
            layout.size.height
        );
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.y), 20f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
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
            node15,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
