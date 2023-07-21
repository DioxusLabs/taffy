#[test]
fn grid_auto_columns_fixed_width() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node9 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node10 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node11 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node12 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node13 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node14 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node15 = taffy.new_leaf(taffy::style::Style { ..Default::default() });
    let node = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            grid_template_rows: vec![length(40f32), auto(), length(40f32), auto()],
            grid_template_columns: vec![length(40f32), auto(), length(40f32), auto()],
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(200f32),
                height: taffy::style::Dimension::Length(200f32),
            },
            ..Default::default()
        },
        &[
            node0, node1, node2, node3, node4, node5, node6, node7, node8, node9, node10, node11, node12, node13,
            node14, node15,
        ],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node1, 60f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 100f32, "x of node {:?}. Expected {}. Actual {}", node2, 100f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node3, 60f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 140f32, "x of node {:?}. Expected {}. Actual {}", node3, 140f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node4, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node5, 60f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node5, 60f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node5, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node5, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node6, 60f32, size.height);
    assert_eq!(location.x, 100f32, "x of node {:?}. Expected {}. Actual {}", node6, 100f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node6, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node7);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node7, 60f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node7, 60f32, size.height);
    assert_eq!(location.x, 140f32, "x of node {:?}. Expected {}. Actual {}", node7, 140f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node7, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node8);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node8, 0f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node8, 100f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node9);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node9, 60f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node9, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node9, 40f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node9, 100f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node10, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node10, 40f32, size.height);
    assert_eq!(location.x, 100f32, "x of node {:?}. Expected {}. Actual {}", node10, 100f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node10, 100f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node11, 60f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node11, 40f32, size.height);
    assert_eq!(location.x, 140f32, "x of node {:?}. Expected {}. Actual {}", node11, 140f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node11, 100f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node12);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node12, 40f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node12, 60f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node12, 0f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node12, 140f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node13);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node13, 60f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node13, 60f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node13, 40f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node13, 140f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node14);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node14, 40f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node14, 60f32, size.height);
    assert_eq!(location.x, 100f32, "x of node {:?}. Expected {}. Actual {}", node14, 100f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node14, 140f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node15);
    assert_eq!(size.width, 60f32, "width of node {:?}. Expected {}. Actual {}", node15, 60f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node15, 60f32, size.height);
    assert_eq!(location.x, 140f32, "x of node {:?}. Expected {}. Actual {}", node15, 140f32, location.x);
    assert_eq!(location.y, 140f32, "y of node {:?}. Expected {}. Actual {}", node15, 140f32, location.y);
}
