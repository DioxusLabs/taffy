#[test]
#[allow(non_snake_case)]
fn bevy_issue_9530__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_direction: taffy::style::FlexDirection::Column,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11_text = "HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH" ;
    let node11 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Center),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                flex_grow: 1f32,
                margin: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text(node11_text, crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(1f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node10, node11, node12],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Center),
                align_content: Some(taffy::style::AlignContent::Center),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(300f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::AUTO,
                    right: taffy::style::LengthPercentageAuto::AUTO,
                    top: taffy::style::LengthPercentageAuto::AUTO,
                    bottom: taffy::style::LengthPercentageAuto::AUTO,
                },
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
    if layout.size.width != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 300f32, layout.size.width);
    }
    if layout.size.height != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 300f32, layout.size.height);
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
    if layout.scroll_height() != 140f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            140f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0).unwrap();
    if layout.size.width != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 300f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 0f32, layout.size.height);
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
    if layout.size.width != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 300f32, layout.size.width);
    }
    if layout.size.height != 420f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(size.height),
            420f32,
            layout.size.height
        );
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
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 260f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 260f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
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
    if layout.size.width != 220f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 220f32, layout.size.width);
    }
    if layout.size.height != 240f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(size.height),
            240f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 90f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.y), 90f32, layout.location.y);
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
    if layout.size.width != 260f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 260f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 350f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 350f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_9530__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_direction: taffy::style::FlexDirection::Column,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11_text = "HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH\u{200b}HHHH" ;
    let node11 = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_items: Some(taffy::style::AlignItems::Center),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                flex_grow: 1f32,
                margin: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            crate::TestNodeContext::ahem_text(node11_text, crate::WritingMode::Horizontal),
        )
        .unwrap();
    let node12 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_percent(1f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(1f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(20f32),
                    right: length(20f32),
                    top: length(20f32),
                    bottom: length(20f32),
                },
                ..Default::default()
            },
            &[node10, node11, node12],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Center),
                align_content: Some(taffy::style::AlignContent::Center),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(300f32),
                    height: taffy::style::Dimension::from_length(300f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::AUTO,
                    right: taffy::style::LengthPercentageAuto::AUTO,
                    top: taffy::style::LengthPercentageAuto::AUTO,
                    bottom: taffy::style::LengthPercentageAuto::AUTO,
                },
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
    if layout.size.width != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 300f32, layout.size.width);
    }
    if layout.size.height != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 300f32, layout.size.height);
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
    if layout.scroll_width() != 20f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_width()),
            20f32,
            layout.scroll_width()
        );
    }
    #[cfg(feature = "content_size")]
    if layout.scroll_height() != 100f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node,
            stringify!(scroll_height()),
            100f32,
            layout.scroll_height()
        );
    }
    let layout = taffy.layout(node0).unwrap();
    if layout.size.width != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 300f32, layout.size.width);
    }
    if layout.size.height != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.height), 0f32, layout.size.height);
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
    if layout.size.width != 340f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(size.width), 340f32, layout.size.width);
    }
    if layout.size.height != 380f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node1,
            stringify!(size.height),
            380f32,
            layout.size.height
        );
    }
    if layout.location.x != -20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node1, stringify!(location.x), -20f32, layout.location.x);
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
    let layout = taffy.layout(node10).unwrap();
    if layout.size.width != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node10, stringify!(size.width), 300f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node10,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
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
    if layout.size.width != 260f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(size.width), 260f32, layout.size.width);
    }
    if layout.size.height != 200f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node11,
            stringify!(size.height),
            200f32,
            layout.size.height
        );
    }
    if layout.location.x != 40f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.x), 40f32, layout.location.x);
    }
    if layout.location.y != 90f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node11, stringify!(location.y), 90f32, layout.location.y);
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
    if layout.size.width != 300f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(size.width), 300f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node12,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 20f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.x), 20f32, layout.location.x);
    }
    if layout.location.y != 310f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node12, stringify!(location.y), 310f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
