#[test]
#[allow(non_snake_case)]
fn multiline_column_max_height__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node9 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node16 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node17 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node18 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node19 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::from_length(200f32) },
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
    if layout.size.width != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 80f32, layout.size.width);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 20f32, layout.size.height);
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
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 20f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 40f32, layout.location.y);
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
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 60f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 60f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.y), 80f32, layout.location.y);
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
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.y), 100f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 120f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.y), 120f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 140f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.y), 140f32, layout.location.y);
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
    let layout = taffy.layout(node8).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 160f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.y), 160f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node8,
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
            node8,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node9).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 180f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.y), 180f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node9,
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
            node9,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.y), 0f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 40f32, layout.location.x);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 40f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 60f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.y), 60f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.y), 80f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.y), 100f32, layout.location.y);
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
    let layout = taffy.layout(node16).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 120f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.y), 120f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
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
            node16,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node17).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 140f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.y), 140f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
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
            node17,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node18).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 160f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.y), 160f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
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
            node18,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node19).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 180f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.y), 180f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
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
fn multiline_column_max_height__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node9 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node13 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node14 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node15 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node16 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node17 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node18 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node19 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(40f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::from_length(200f32) },
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
    if layout.size.width != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 80f32, layout.size.width);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 20f32, layout.size.height);
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
    if layout.location.y != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.y), 20f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node2, stringify!(location.y), 40f32, layout.location.y);
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
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 60f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node3, stringify!(location.y), 60f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node4, stringify!(location.y), 80f32, layout.location.y);
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
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node5, stringify!(location.y), 100f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 120f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node6, stringify!(location.y), 120f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 140f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node7, stringify!(location.y), 140f32, layout.location.y);
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
    let layout = taffy.layout(node8).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 160f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node8, stringify!(location.y), 160f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node8,
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
            node8,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node9).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(size.height), 20f32, layout.size.height);
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 180f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node9, stringify!(location.y), 180f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node9,
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
            node9,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(location.y), 0f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 40f32, layout.location.x);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 40f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node13,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 60f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node13, stringify!(location.y), 60f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node14,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 80f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node14, stringify!(location.y), 80f32, layout.location.y);
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
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node15,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node15, stringify!(location.y), 100f32, layout.location.y);
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
    let layout = taffy.layout(node16).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 120f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node16, stringify!(location.y), 120f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node16,
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
            node16,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node17).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 140f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node17, stringify!(location.y), 140f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node17,
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
            node17,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node18).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 160f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node18, stringify!(location.y), 160f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node18,
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
            node18,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node19).unwrap();
    if layout.size.width != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(size.width), 40f32, layout.size.width);
    }
    if layout.size.height != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
            stringify!(size.height),
            20f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 180f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node19, stringify!(location.y), 180f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node19,
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
            node19,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
