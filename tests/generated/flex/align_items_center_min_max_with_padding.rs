#[test]
#[allow(non_snake_case)]
fn align_items_center_min_max_with_padding__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(62f32),
                height: taffy::style::Dimension::from_length(62f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::Center),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(320f32),
                    height: taffy::style::Dimension::from_length(72f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(320f32),
                    height: taffy::style::Dimension::from_length(504f32),
                },
                padding: taffy::geometry::Rect { left: zero(), right: zero(), top: length(8f32), bottom: length(8f32) },
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
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node, 320f32, size.width);
    assert_eq!(size.height, 78f32, "height of node {:?}. Expected {}. Actual {}", node, 78f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 62f32, "width of node {:?}. Expected {}. Actual {}", node0, 62f32, size.width);
    assert_eq!(size.height, 62f32, "height of node {:?}. Expected {}. Actual {}", node0, 62f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn align_items_center_min_max_with_padding__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::from_length(62f32),
                height: taffy::style::Dimension::from_length(62f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_items: Some(taffy::style::AlignItems::Center),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(320f32),
                    height: taffy::style::Dimension::from_length(72f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(320f32),
                    height: taffy::style::Dimension::from_length(504f32),
                },
                padding: taffy::geometry::Rect { left: zero(), right: zero(), top: length(8f32), bottom: length(8f32) },
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
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node, 320f32, size.width);
    assert_eq!(size.height, 88f32, "height of node {:?}. Expected {}. Actual {}", node, 88f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 62f32, "width of node {:?}. Expected {}. Actual {}", node0, 62f32, size.width);
    assert_eq!(size.height, 62f32, "height of node {:?}. Expected {}. Actual {}", node0, 62f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 13f32, "y of node {:?}. Expected {}. Actual {}", node0, 13f32, location.y);
}
