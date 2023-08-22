#[test]
fn grid_percent_tracks_definite_overflow() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
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
                grid_template_rows: vec![percent(0.5f32), percent(0.8f32)],
                grid_template_columns: vec![percent(0.4f32), percent(0.4f32), percent(0.4f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(120f32),
                    height: taffy::style::Dimension::Length(60f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node0, 48f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node0, 30f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node1, 48f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node1, 30f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node1, 48f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node2, 48f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node2, 30f32, size.height);
    assert_eq!(location.x, 96f32, "x of node {:?}. Expected {}. Actual {}", node2, 96f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node3, 48f32, size.width);
    assert_eq!(size.height, 48f32, "height of node {:?}. Expected {}. Actual {}", node3, 48f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node3, 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node4, 48f32, size.width);
    assert_eq!(size.height, 48f32, "height of node {:?}. Expected {}. Actual {}", node4, 48f32, size.height);
    assert_eq!(location.x, 48f32, "x of node {:?}. Expected {}. Actual {}", node4, 48f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node4, 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 48f32, "width of node {:?}. Expected {}. Actual {}", node5, 48f32, size.width);
    assert_eq!(size.height, 48f32, "height of node {:?}. Expected {}. Actual {}", node5, 48f32, size.height);
    assert_eq!(location.x, 96f32, "x of node {:?}. Expected {}. Actual {}", node5, 96f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node5, 30f32, location.y);
}
