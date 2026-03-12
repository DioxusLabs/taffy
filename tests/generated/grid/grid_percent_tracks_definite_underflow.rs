#[test]
#[allow(non_snake_case)]
fn grid_percent_tracks_definite_underflow__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![percent(0.3f32), percent(0.6f32)],
                grid_template_columns: vec![percent(0.1f32), percent(0.2f32), percent(0.3f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(60f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node0, 12f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node0, 18f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node1, 24f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node1, 18f32, size.height);
    assert_eq!(location.x, 12f32, "x of node {:?}. Expected {}. Actual {}", node1, 12f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node2, 36f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node2, 18f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node2, 36f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node3, 12f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node3, 36f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node3, 18f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node4, 24f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node4, 36f32, size.height);
    assert_eq!(location.x, 12f32, "x of node {:?}. Expected {}. Actual {}", node4, 12f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node4, 18f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node5, 36f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node5, 36f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node5, 36f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node5, 18f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_tracks_definite_underflow__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_rows: vec![percent(0.3f32), percent(0.6f32)],
                grid_template_columns: vec![percent(0.1f32), percent(0.2f32), percent(0.3f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(60f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node0, 12f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node0, 18f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node1, 24f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node1, 18f32, size.height);
    assert_eq!(location.x, 12f32, "x of node {:?}. Expected {}. Actual {}", node1, 12f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node2, 36f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node2, 18f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node2, 36f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node3, 12f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node3, 36f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node3, 18f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node4, 24f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node4, 36f32, size.height);
    assert_eq!(location.x, 12f32, "x of node {:?}. Expected {}. Actual {}", node4, 12f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node4, 18f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node5, 36f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node5, 36f32, size.height);
    assert_eq!(location.x, 36f32, "x of node {:?}. Expected {}. Actual {}", node5, 36f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node5, 18f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_tracks_definite_underflow__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node1 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node2 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node3 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node4 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node5 =
        taffy.new_leaf(taffy::style::Style { direction: taffy::style::Direction::Rtl, ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                grid_template_rows: vec![percent(0.3f32), percent(0.6f32)],
                grid_template_columns: vec![percent(0.1f32), percent(0.2f32), percent(0.3f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(60f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node0, 12f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node0, 18f32, size.height);
    assert_eq!(location.x, 108f32, "x of node {:?}. Expected {}. Actual {}", node0, 108f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node1, 24f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node1, 18f32, size.height);
    assert_eq!(location.x, 84f32, "x of node {:?}. Expected {}. Actual {}", node1, 84f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node2, 36f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node2, 18f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node2, 48f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node3, 12f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node3, 36f32, size.height);
    assert_eq!(location.x, 108f32, "x of node {:?}. Expected {}. Actual {}", node3, 108f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node3, 18f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node4, 24f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node4, 36f32, size.height);
    assert_eq!(location.x, 84f32, "x of node {:?}. Expected {}. Actual {}", node4, 84f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node4, 18f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node5, 36f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node5, 36f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node5, 48f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node5, 18f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_tracks_definite_underflow__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
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
                grid_template_rows: vec![percent(0.3f32), percent(0.6f32)],
                grid_template_columns: vec![percent(0.1f32), percent(0.2f32), percent(0.3f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(120f32),
                    height: taffy::style::Dimension::from_length(60f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node0, 12f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node0, 18f32, size.height);
    assert_eq!(location.x, 108f32, "x of node {:?}. Expected {}. Actual {}", node0, 108f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node1, 24f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node1, 18f32, size.height);
    assert_eq!(location.x, 84f32, "x of node {:?}. Expected {}. Actual {}", node1, 84f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node2, 36f32, size.width);
    assert_eq!(size.height, 18f32, "height of node {:?}. Expected {}. Actual {}", node2, 18f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node2, 48f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 12f32, "width of node {:?}. Expected {}. Actual {}", node3, 12f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node3, 36f32, size.height);
    assert_eq!(location.x, 108f32, "x of node {:?}. Expected {}. Actual {}", node3, 108f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node3, 18f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 24f32, "width of node {:?}. Expected {}. Actual {}", node4, 24f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node4, 36f32, size.height);
    assert_eq!(location.x, 84f32, "x of node {:?}. Expected {}. Actual {}", node4, 84f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node4, 18f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 36f32, "width of node {:?}. Expected {}. Actual {}", node5, 36f32, size.width);
    assert_eq!(size.height, 36f32, "height of node {:?}. Expected {}. Actual {}", node5, 36f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node5, 48f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node5, 18f32, location.y);
}
