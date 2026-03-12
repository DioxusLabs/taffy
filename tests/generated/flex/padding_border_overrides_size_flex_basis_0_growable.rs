#[test]
#[allow(non_snake_case)]
fn padding_border_overrides_size_flex_basis_0_growable__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            padding: taffy::geometry::Rect {
                left: length(8f32),
                right: length(4f32),
                top: length(2f32),
                bottom: length(6f32),
            },
            border: taffy::geometry::Rect {
                left: length(7f32),
                right: length(3f32),
                top: length(1f32),
                bottom: length(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node0, node1]).unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 34f32, "width of node {:?}. Expected {}. Actual {}", node, 34f32, size.width);
    assert_eq!(size.height, 14f32, "height of node {:?}. Expected {}. Actual {}", node, 14f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 14f32, "height of node {:?}. Expected {}. Actual {}", node0, 14f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 6f32, "width of node {:?}. Expected {}. Actual {}", node1, 6f32, size.width);
    assert_eq!(size.height, 12f32, "height of node {:?}. Expected {}. Actual {}", node1, 12f32, size.height);
    assert_eq!(location.x, 28f32, "x of node {:?}. Expected {}. Actual {}", node1, 28f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn padding_border_overrides_size_flex_basis_0_growable__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            padding: taffy::geometry::Rect {
                left: length(8f32),
                right: length(4f32),
                top: length(2f32),
                bottom: length(6f32),
            },
            border: taffy::geometry::Rect {
                left: length(7f32),
                right: length(3f32),
                top: length(1f32),
                bottom: length(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            ..Default::default()
        })
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
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 46f32, "width of node {:?}. Expected {}. Actual {}", node, 46f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node, 26f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 34f32, "width of node {:?}. Expected {}. Actual {}", node0, 34f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node0, 26f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node1, 12f32, size.width);
    assert_eq!(size.height, 12f32, "height of node {:?}. Expected {}. Actual {}", node1, 12f32, size.height);
    assert_eq!(location.x, 34f32, "x of node {:?}. Expected {}. Actual {}", node1, 34f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn padding_border_overrides_size_flex_basis_0_growable__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            padding: taffy::geometry::Rect {
                left: length(8f32),
                right: length(4f32),
                top: length(2f32),
                bottom: length(6f32),
            },
            border: taffy::geometry::Rect {
                left: length(7f32),
                right: length(3f32),
                top: length(1f32),
                bottom: length(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 34f32, "width of node {:?}. Expected {}. Actual {}", node, 34f32, size.width);
    assert_eq!(size.height, 14f32, "height of node {:?}. Expected {}. Actual {}", node, 14f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 14f32, "height of node {:?}. Expected {}. Actual {}", node0, 14f32, size.height);
    assert_eq!(location.x, 6f32, "x of node {:?}. Expected {}. Actual {}", node0, 6f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 6f32, "width of node {:?}. Expected {}. Actual {}", node1, 6f32, size.width);
    assert_eq!(size.height, 12f32, "height of node {:?}. Expected {}. Actual {}", node1, 12f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn padding_border_overrides_size_flex_basis_0_growable__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            padding: taffy::geometry::Rect {
                left: length(8f32),
                right: length(4f32),
                top: length(2f32),
                bottom: length(6f32),
            },
            border: taffy::geometry::Rect {
                left: length(7f32),
                right: length(3f32),
                top: length(1f32),
                bottom: length(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::from_length(0f32),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(12f32),
                height: taffy::style::Dimension::from_length(12f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
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
    assert_eq!(size.width, 46f32, "width of node {:?}. Expected {}. Actual {}", node, 46f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node, 26f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 34f32, "width of node {:?}. Expected {}. Actual {}", node0, 34f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node0, 26f32, size.height);
    assert_eq!(location.x, 12f32, "x of node {:?}. Expected {}. Actual {}", node0, 12f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node1, 12f32, size.width);
    assert_eq!(size.height, 12f32, "height of node {:?}. Expected {}. Actual {}", node1, 12f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}
