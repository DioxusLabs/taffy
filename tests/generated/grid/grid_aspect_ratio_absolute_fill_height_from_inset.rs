#[test]
#[allow(non_snake_case)]
fn grid_aspect_ratio_absolute_fill_height_from_inset__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            aspect_ratio: Some(3f32),
            inset: taffy::geometry::Rect {
                left: percent(0.1f32),
                right: percent(0.1f32),
                top: percent(0.05f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(300f32),
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
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node0, 320f32, size.width);
    assert_eq!(size.height, 107f32, "height of node {:?}. Expected {}. Actual {}", node0, 107f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 15f32, "y of node {:?}. Expected {}. Actual {}", node0, 15f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_aspect_ratio_absolute_fill_height_from_inset__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            aspect_ratio: Some(3f32),
            inset: taffy::geometry::Rect {
                left: percent(0.1f32),
                right: percent(0.1f32),
                top: percent(0.05f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(400f32),
                    height: taffy::style::Dimension::from_length(300f32),
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
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 300f32, "height of node {:?}. Expected {}. Actual {}", node, 300f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node0, 320f32, size.width);
    assert_eq!(size.height, 107f32, "height of node {:?}. Expected {}. Actual {}", node0, 107f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 15f32, "y of node {:?}. Expected {}. Actual {}", node0, 15f32, location.y);
}
