#[test]
#[allow(non_snake_case)]
fn android_news_feed__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(120f32),
                height: taffy::style::Dimension::from_length(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(36f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(72f32),
                height: taffy::style::Dimension::from_length(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(174f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(1080f32), height: auto() },
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
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 1080f32, layout.size.width);
    }
    if layout.size.height != 240f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 240f32, layout.size.height);
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
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 1080f32, layout.size.width);
    }
    if layout.size.height != 240f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(size.height),
            240f32,
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
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(size.width),
            1080f32,
            layout.size.width
        );
    }
    if layout.size.height != 240f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(size.height),
            240f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.y), 0f32, layout.location.y);
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
    let layout = taffy.layout(node000).unwrap();
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000,
            stringify!(size.width),
            1080f32,
            layout.size.width
        );
    }
    if layout.size.height != 144f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000,
            stringify!(size.height),
            144f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node000, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node000, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000,
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
            node000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0000).unwrap();
    if layout.size.width != 1044f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(size.width),
            1044f32,
            layout.size.width
        );
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(size.height),
            120f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 24f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(location.y),
            24f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
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
            node0000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00000).unwrap();
    if layout.size.width != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(size.width),
            120f32,
            layout.size.width
        );
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(size.height),
            120f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
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
            node00000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node000000).unwrap();
    if layout.size.width != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(size.width),
            120f32,
            layout.size.width
        );
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(size.height),
            120f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
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
            node000000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00001).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 39f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(size.height),
            39f32,
            layout.size.height
        );
    }
    if layout.location.x != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(location.x),
            120f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
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
            node00001,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node000010).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
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
            node000010,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node000011).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
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
            node000011,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001).unwrap();
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
            stringify!(size.width),
            1080f32,
            layout.size.width
        );
    }
    if layout.size.height != 96f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
            stringify!(size.height),
            96f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node001, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 144f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
            stringify!(location.y),
            144f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
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
            node001,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0010).unwrap();
    if layout.size.width != 906f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(size.width),
            906f32,
            layout.size.width
        );
    }
    if layout.size.height != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(size.height),
            72f32,
            layout.size.height
        );
    }
    if layout.location.x != 174f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(location.x),
            174f32,
            layout.location.x
        );
    }
    if layout.location.y != 24f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(location.y),
            24f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
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
            node0010,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00100).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(size.height),
            72f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
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
            node00100,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001000).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(size.height),
            72f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
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
            node001000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00101).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 39f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(size.height),
            39f32,
            layout.size.height
        );
    }
    if layout.location.x != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(location.x),
            72f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
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
            node00101,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001010).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
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
            node001010,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001011).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
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
            node001011,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn android_news_feed__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(120f32),
                height: taffy::style::Dimension::from_length(120f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node000000],
        )
        .unwrap();
    let node000010 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node000011 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node000010, node000011],
        )
        .unwrap();
    let node0000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(36f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00000, node00001],
        )
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node001000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(72f32),
                height: taffy::style::Dimension::from_length(72f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00100 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node001000],
        )
        .unwrap();
    let node001010 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node001011 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_content: Some(taffy::style::AlignContent::Stretch),
            flex_shrink: 1f32,
            ..Default::default()
        })
        .unwrap();
    let node00101 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 1f32,
                margin: taffy::geometry::Rect { left: zero(), right: length(36f32), top: zero(), bottom: zero() },
                padding: taffy::geometry::Rect {
                    left: length(36f32),
                    right: length(36f32),
                    top: length(21f32),
                    bottom: length(18f32),
                },
                ..Default::default()
            },
            &[node001010, node001011],
        )
        .unwrap();
    let node0010 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Stretch),
                margin: taffy::geometry::Rect {
                    left: length(174f32),
                    right: zero(),
                    top: length(24f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00100, node00101],
        )
        .unwrap();
    let node001 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node0010],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_shrink: 0f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_shrink: 0f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_length(1080f32), height: auto() },
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
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 1080f32, layout.size.width);
    }
    if layout.size.height != 240f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 240f32, layout.size.height);
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
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 1080f32, layout.size.width);
    }
    if layout.size.height != 240f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(size.height),
            240f32,
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
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(size.width),
            1080f32,
            layout.size.width
        );
    }
    if layout.size.height != 240f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(size.height),
            240f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(location.y), 0f32, layout.location.y);
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
    let layout = taffy.layout(node000).unwrap();
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000,
            stringify!(size.width),
            1080f32,
            layout.size.width
        );
    }
    if layout.size.height != 144f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000,
            stringify!(size.height),
            144f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node000, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node000, stringify!(location.y), 0f32, layout.location.y);
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000,
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
            node000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0000).unwrap();
    if layout.size.width != 1044f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(size.width),
            1044f32,
            layout.size.width
        );
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(size.height),
            120f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 24f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
            stringify!(location.y),
            24f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0000,
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
            node0000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00000).unwrap();
    if layout.size.width != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(size.width),
            120f32,
            layout.size.width
        );
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(size.height),
            120f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00000,
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
            node00000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node000000).unwrap();
    if layout.size.width != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(size.width),
            120f32,
            layout.size.width
        );
    }
    if layout.size.height != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(size.height),
            120f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000000,
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
            node000000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00001).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 39f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(size.height),
            39f32,
            layout.size.height
        );
    }
    if layout.location.x != 120f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(location.x),
            120f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00001,
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
            node00001,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node000010).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000010,
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
            node000010,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node000011).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node000011,
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
            node000011,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001).unwrap();
    if layout.size.width != 1080f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
            stringify!(size.width),
            1080f32,
            layout.size.width
        );
    }
    if layout.size.height != 96f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
            stringify!(size.height),
            96f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node001, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 144f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
            stringify!(location.y),
            144f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001,
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
            node001,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0010).unwrap();
    if layout.size.width != 906f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(size.width),
            906f32,
            layout.size.width
        );
    }
    if layout.size.height != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(size.height),
            72f32,
            layout.size.height
        );
    }
    if layout.location.x != 174f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(location.x),
            174f32,
            layout.location.x
        );
    }
    if layout.location.y != 24f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
            stringify!(location.y),
            24f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0010,
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
            node0010,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00100).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(size.height),
            72f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00100,
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
            node00100,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001000).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(size.height),
            72f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(location.x),
            0f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001000,
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
            node001000,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node00101).unwrap();
    if layout.size.width != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(size.width),
            72f32,
            layout.size.width
        );
    }
    if layout.size.height != 39f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(size.height),
            39f32,
            layout.size.height
        );
    }
    if layout.location.x != 72f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(location.x),
            72f32,
            layout.location.x
        );
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
            stringify!(location.y),
            0f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00101,
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
            node00101,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001010).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001010,
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
            node001010,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node001011).unwrap();
    if layout.size.width != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(size.width),
            0f32,
            layout.size.width
        );
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(size.height),
            0f32,
            layout.size.height
        );
    }
    if layout.location.x != 36f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(location.x),
            36f32,
            layout.location.x
        );
    }
    if layout.location.y != 21f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
            stringify!(location.y),
            21f32,
            layout.location.y
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_width() != 0f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node001011,
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
            node001011,
            stringify!(scroll_height()),
            0f32,
            layout.scroll_height()
        );
    }
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
