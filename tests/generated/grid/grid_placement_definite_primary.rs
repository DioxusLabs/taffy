#[test]
#[allow(non_snake_case)]
fn grid_placement_definite_primary__border_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_columns: vec![percent(0.33f32), percent(0.74f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
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
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node0, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node0, 67f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node1, 148f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node1, 67f32, size.height);
    assert_eq!(location.x, 66f32, "x of node {:?}. Expected {}. Actual {}", node1, 66f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node2, 66f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node2, 66f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node2, 67f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node3, 148f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node3, 66f32, size.height);
    assert_eq!(location.x, 66f32, "x of node {:?}. Expected {}. Actual {}", node3, 66f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node3, 67f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node4, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node4, 67f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 133f32, "y of node {:?}. Expected {}. Actual {}", node4, 133f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_placement_definite_primary__content_box_ltr() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_columns: vec![percent(0.33f32), percent(0.74f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
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
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node0, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node0, 67f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node1, 148f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node1, 67f32, size.height);
    assert_eq!(location.x, 66f32, "x of node {:?}. Expected {}. Actual {}", node1, 66f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node2, 66f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node2, 66f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node2, 67f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node3, 148f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node3, 66f32, size.height);
    assert_eq!(location.x, 66f32, "x of node {:?}. Expected {}. Actual {}", node3, 66f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node3, 67f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node4, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node4, 67f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 133f32, "y of node {:?}. Expected {}. Actual {}", node4, 133f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_placement_definite_primary__border_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                direction: taffy::style::Direction::Rtl,
                grid_template_columns: vec![percent(0.33f32), percent(0.74f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
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
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node0, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node0, 67f32, size.height);
    assert_eq!(location.x, 134f32, "x of node {:?}. Expected {}. Actual {}", node0, 134f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node1, 148f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node1, 67f32, size.height);
    assert_eq!(location.x, -14f32, "x of node {:?}. Expected {}. Actual {}", node1, -14f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node2, 66f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node2, 66f32, size.height);
    assert_eq!(location.x, 134f32, "x of node {:?}. Expected {}. Actual {}", node2, 134f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node2, 67f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node3, 148f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node3, 66f32, size.height);
    assert_eq!(location.x, -14f32, "x of node {:?}. Expected {}. Actual {}", node3, -14f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node3, 67f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node4, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node4, 67f32, size.height);
    assert_eq!(location.x, 134f32, "x of node {:?}. Expected {}. Actual {}", node4, 134f32, location.x);
    assert_eq!(location.y, 133f32, "y of node {:?}. Expected {}. Actual {}", node4, 133f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_placement_definite_primary__content_box_rtl() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(2i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            direction: taffy::style::Direction::Rtl,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                direction: taffy::style::Direction::Rtl,
                grid_template_columns: vec![percent(0.33f32), percent(0.74f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200f32),
                    height: taffy::style::Dimension::from_length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
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
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node0, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node0, 67f32, size.height);
    assert_eq!(location.x, 134f32, "x of node {:?}. Expected {}. Actual {}", node0, 134f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node1, 148f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node1, 67f32, size.height);
    assert_eq!(location.x, -14f32, "x of node {:?}. Expected {}. Actual {}", node1, -14f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node2, 66f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node2, 66f32, size.height);
    assert_eq!(location.x, 134f32, "x of node {:?}. Expected {}. Actual {}", node2, 134f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node2, 67f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 148f32, "width of node {:?}. Expected {}. Actual {}", node3, 148f32, size.width);
    assert_eq!(size.height, 66f32, "height of node {:?}. Expected {}. Actual {}", node3, 66f32, size.height);
    assert_eq!(location.x, -14f32, "x of node {:?}. Expected {}. Actual {}", node3, -14f32, location.x);
    assert_eq!(location.y, 67f32, "y of node {:?}. Expected {}. Actual {}", node3, 67f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 66f32, "width of node {:?}. Expected {}. Actual {}", node4, 66f32, size.width);
    assert_eq!(size.height, 67f32, "height of node {:?}. Expected {}. Actual {}", node4, 67f32, size.height);
    assert_eq!(location.x, 134f32, "x of node {:?}. Expected {}. Actual {}", node4, 134f32, location.x);
    assert_eq!(location.y, 133f32, "y of node {:?}. Expected {}. Actual {}", node4, 133f32, location.y);
}
