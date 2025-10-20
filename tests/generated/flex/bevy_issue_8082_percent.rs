#[test]
#[allow(non_snake_case)]
fn bevy_issue_8082_percent__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(0.5f32), height: auto() },
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                align_items: Some(taffy::style::AlignItems::Stretch),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::FlexStart),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(400f32),
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
    if layout.size.width != 200f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 200f32, layout.size.width);
    }
    if layout.size.height != 400f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 400f32, layout.size.height);
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
    if layout.size.width != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 100f32, layout.size.width);
    }
    if layout.size.height != 100f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(size.height),
            100f32,
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
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(size.height),
            50f32,
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
    let layout = taffy.layout(node01).unwrap();
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node01,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.y), 0f32, layout.location.y);
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
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node02,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.y), 50f32, layout.location.y);
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
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node03,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.y), 50f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_8082_percent__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node03 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            overflow: taffy::geometry::Point { x: taffy::style::Overflow::Visible, y: taffy::style::Overflow::Visible },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(50f32),
                height: taffy::style::Dimension::from_length(50f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                align_items: Some(taffy::style::AlignItems::FlexStart),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                size: taffy::geometry::Size { width: taffy::style::Dimension::from_percent(0.5f32), height: auto() },
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Visible,
                    y: taffy::style::Overflow::Visible,
                },
                align_items: Some(taffy::style::AlignItems::Stretch),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::FlexStart),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(400f32),
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
    if layout.size.width != 200f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.width), 200f32, layout.size.width);
    }
    if layout.size.height != 400f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node, stringify!(size.height), 400f32, layout.size.height);
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
    if layout.size.width != 100f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node0, stringify!(size.width), 100f32, layout.size.width);
    }
    if layout.size.height != 100f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node0,
            stringify!(size.height),
            100f32,
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
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node00, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node00,
            stringify!(size.height),
            50f32,
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
    let layout = taffy.layout(node01).unwrap();
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node01,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node01, stringify!(location.y), 0f32, layout.location.y);
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
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node02,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 0f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.x), 0f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node02, stringify!(location.y), 50f32, layout.location.y);
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
    if layout.size.width != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(size.width), 50f32, layout.size.width);
    }
    if layout.size.height != 50f32 {
        mismatches += 1;
        eprintln!(
            "{:?}.{} mismatch: expected {} actual {}",
            node03,
            stringify!(size.height),
            50f32,
            layout.size.height
        );
    }
    if layout.location.x != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.x), 50f32, layout.location.x);
    }
    if layout.location.y != 50f32 {
        mismatches += 1;
        eprintln!("{:?}.{} mismatch: expected {} actual {}", node03, stringify!(location.y), 50f32, layout.location.y);
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
    assert!(mismatches == 0, "Detected {mismatches} mismatch(es)");
}
