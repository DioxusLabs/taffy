#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_moderate__border_box_ltr() {
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 200f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert!(
        (size.height - 42.15625f32).abs() < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node,
        42.15625f32,
        size.height
    );
    assert!((location.x - 0f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert!((location.y - 0f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 97f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node0, 97f32, size.width);
    assert!(
        (size.height - 26.15625f32).abs() < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node0,
        26.15625f32,
        size.height
    );
    assert!((location.x - 8f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert!((location.y - 8f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert!(
        (size.width - 38.40625f32).abs() < 0.1,
        "width of node {:?}. Expected {}. Actual {}",
        node00,
        38.40625f32,
        size.width
    );
    assert!((size.height - 6f32).abs() < 0.1, "height of node {:?}. Expected {}. Actual {}", node00, 6f32, size.height);
    assert!(
        (location.x - 10.078125f32).abs() < 0.1,
        "x of node {:?}. Expected {}. Actual {}",
        node00,
        10.078125f32,
        location.x
    );
    assert!(
        (location.y - 10.078125f32).abs() < 0.1,
        "y of node {:?}. Expected {}. Actual {}",
        node00,
        10.078125f32,
        location.y
    );
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_moderate__content_box_ltr() {
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 206f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node, 206f32, size.width);
    assert!((size.height - 44f32).abs() < 0.1, "height of node {:?}. Expected {}. Actual {}", node, 44f32, size.height);
    assert!((location.x - 0f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert!((location.y - 0f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 112f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node0, 112f32, size.width);
    assert!(
        (size.height - 28f32).abs() < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node0,
        28f32,
        size.height
    );
    assert!((location.x - 8f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert!((location.y - 8f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 51f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node00, 51f32, size.width);
    assert!((size.height - 6f32).abs() < 0.1, "height of node {:?}. Expected {}. Actual {}", node00, 6f32, size.height);
    assert!((location.x - 11f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node00, 11f32, location.x);
    assert!((location.y - 11f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node00, 11f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_moderate__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    taffy.disable_rounding();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
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
                direction: taffy::style::Direction::Rtl,
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
                direction: taffy::style::Direction::Rtl,
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 200f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert!(
        (size.height - 42.15625f32).abs() < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node,
        42.15625f32,
        size.height
    );
    assert!((location.x - 0f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert!((location.y - 0f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 97f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node0, 97f32, size.width);
    assert!(
        (size.height - 26.15625f32).abs() < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node0,
        26.15625f32,
        size.height
    );
    assert!((location.x - 95f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node0, 95f32, location.x);
    assert!((location.y - 8f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert!(
        (size.width - 38.40625f32).abs() < 0.1,
        "width of node {:?}. Expected {}. Actual {}",
        node00,
        38.40625f32,
        size.width
    );
    assert!((size.height - 6f32).abs() < 0.1, "height of node {:?}. Expected {}. Actual {}", node00, 6f32, size.height);
    assert!(
        (location.x - 48.515625f32).abs() < 0.1,
        "x of node {:?}. Expected {}. Actual {}",
        node00,
        48.515625f32,
        location.x
    );
    assert!(
        (location.y - 10.078125f32).abs() < 0.1,
        "y of node {:?}. Expected {}. Actual {}",
        node00,
        10.078125f32,
        location.y
    );
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_moderate__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    taffy.disable_rounding();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
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
                direction: taffy::style::Direction::Rtl,
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
                direction: taffy::style::Direction::Rtl,
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 206f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node, 206f32, size.width);
    assert!((size.height - 44f32).abs() < 0.1, "height of node {:?}. Expected {}. Actual {}", node, 44f32, size.height);
    assert!((location.x - 0f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert!((location.y - 0f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 112f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node0, 112f32, size.width);
    assert!(
        (size.height - 28f32).abs() < 0.1,
        "height of node {:?}. Expected {}. Actual {}",
        node0,
        28f32,
        size.height
    );
    assert!((location.x - 86f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node0, 86f32, location.x);
    assert!((location.y - 8f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert!((size.width - 51f32).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", node00, 51f32, size.width);
    assert!((size.height - 6f32).abs() < 0.1, "height of node {:?}. Expected {}. Actual {}", node00, 6f32, size.height);
    assert!((location.x - 50f32).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", node00, 50f32, location.x);
    assert!((location.y - 11f32).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", node00, 11f32, location.y);
}
