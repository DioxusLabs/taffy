#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_with_padding_margin__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
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
    let node00 = taffy
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
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(0.6f32),
                    height: auto(),
                },
                margin: taffy::geometry::Rect {
                    left: length(5f32),
                    right: length(5f32),
                    top: length(5f32),
                    bottom: length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(3f32),
                    right: length(3f32),
                    top: length(3f32),
                    bottom: length(3f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![fr(1f32), fr(4f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 190f32, "width of node {:?}. Expected {}. Actual {}", node0, 190f32, size.width);
    assert_eq!(size.height, 41f32, "height of node {:?}. Expected {}. Actual {}", node0, 41f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 92f32, "width of node {:?}. Expected {}. Actual {}", node00, 92f32, size.width);
    assert_eq!(size.height, 25f32, "height of node {:?}. Expected {}. Actual {}", node00, 25f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node00, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node00, 8f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node000, 36f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node000, 6f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node000, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node000, 10f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 149f32, "height of node {:?}. Expected {}. Actual {}", node1, 149f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 51f32, "y of node {:?}. Expected {}. Actual {}", node1, 51f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_with_padding_margin__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
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
    let node00 = taffy
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
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(0.6f32),
                    height: auto(),
                },
                margin: taffy::geometry::Rect {
                    left: length(5f32),
                    right: length(5f32),
                    top: length(5f32),
                    bottom: length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(3f32),
                    right: length(3f32),
                    top: length(3f32),
                    bottom: length(3f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_rows: vec![fr(1f32), fr(4f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 190f32, "width of node {:?}. Expected {}. Actual {}", node0, 190f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node0, 42f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 103f32, "width of node {:?}. Expected {}. Actual {}", node00, 103f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node00, 26f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node00, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node00, 8f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node000, 48f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node000, 6f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node000, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node000, 10f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 148f32, "height of node {:?}. Expected {}. Actual {}", node1, 148f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 52f32, "y of node {:?}. Expected {}. Actual {}", node1, 52f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_with_padding_margin__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
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
    let node00 = taffy
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
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(0.6f32),
                    height: auto(),
                },
                margin: taffy::geometry::Rect {
                    left: length(5f32),
                    right: length(5f32),
                    top: length(5f32),
                    bottom: length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(3f32),
                    right: length(3f32),
                    top: length(3f32),
                    bottom: length(3f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![fr(1f32), fr(4f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 190f32, "width of node {:?}. Expected {}. Actual {}", node0, 190f32, size.width);
    assert_eq!(size.height, 41f32, "height of node {:?}. Expected {}. Actual {}", node0, 41f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 92f32, "width of node {:?}. Expected {}. Actual {}", node00, 92f32, size.width);
    assert_eq!(size.height, 25f32, "height of node {:?}. Expected {}. Actual {}", node00, 25f32, size.height);
    assert_eq!(location.x, 90f32, "x of node {:?}. Expected {}. Actual {}", node00, 90f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node00, 8f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node000, 36f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node000, 6f32, size.height);
    assert_eq!(location.x, 46f32, "x of node {:?}. Expected {}. Actual {}", node000, 46f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node000, 10f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 149f32, "height of node {:?}. Expected {}. Actual {}", node1, 149f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 51f32, "y of node {:?}. Expected {}. Actual {}", node1, 51f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_with_padding_margin__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node000 = taffy
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
    let node00 = taffy
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
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(0.6f32),
                    height: auto(),
                },
                margin: taffy::geometry::Rect {
                    left: length(5f32),
                    right: length(5f32),
                    top: length(5f32),
                    bottom: length(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: length(3f32),
                    right: length(3f32),
                    top: length(3f32),
                    bottom: length(3f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![fr(1f32), fr(4f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 190f32, "width of node {:?}. Expected {}. Actual {}", node0, 190f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node0, 42f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0, 5f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 103f32, "width of node {:?}. Expected {}. Actual {}", node00, 103f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node00, 26f32, size.height);
    assert_eq!(location.x, 79f32, "x of node {:?}. Expected {}. Actual {}", node00, 79f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node00, 8f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 47f32, "width of node {:?}. Expected {}. Actual {}", node000, 47f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node000, 6f32, size.height);
    assert_eq!(location.x, 46f32, "x of node {:?}. Expected {}. Actual {}", node000, 46f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node000, 10f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 148f32, "height of node {:?}. Expected {}. Actual {}", node1, 148f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 52f32, "y of node {:?}. Expected {}. Actual {}", node1, 52f32, location.y);
}
