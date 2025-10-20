#[test]
#[allow(non_snake_case)]
fn wrap_column__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(31f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(32f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(33f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(34f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(100f32),
                    height: taffy::style::Dimension::from_length(100f32),
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
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 100f32, layout.size.width);
    }
    if layout.size.height != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 100f32, layout.size.height);
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 31f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 31f32, layout.size.height);
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 32f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 32f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 31f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 31f32, layout.location.y);
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
    if layout.size.height != 33f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 33f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 63f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 63f32, layout.location.y);
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 34f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 34f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 0f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn wrap_column__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(31f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(32f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(33f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(30f32),
                height: taffy::style::Dimension::from_length(34f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(100f32),
                    height: taffy::style::Dimension::from_length(100f32),
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
    let mut mismatches = 0u32;
    let layout = taffy.layout(node).unwrap();
    if layout.size.width != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 100f32, layout.size.width);
    }
    if layout.size.height != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 100f32, layout.size.height);
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 31f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 31f32, layout.size.height);
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 32f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.height), 32f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 31f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 31f32, layout.location.y);
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
    if layout.size.height != 33f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 33f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 63f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 63f32, layout.location.y);
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
    if layout.size.width != 30f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.width), 30f32, layout.size.width);
    }
    if layout.size.height != 34f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 34f32, layout.size.height);
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 0f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
