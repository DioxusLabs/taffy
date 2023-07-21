#[test]
fn grid_align_items_baseline_complex() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(20f32),
            height: taffy::style::Dimension::Length(20f32),
        },
        ..Default::default()
    });
    let node10 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        ..Default::default()
    });
    let node1 = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(20f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        },
        &[node10],
    );
    let node20 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        ..Default::default()
    });
    let node2 = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(20f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        },
        &[node20],
    );
    let node3 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(20f32),
            height: taffy::style::Dimension::Length(20f32),
        },
        ..Default::default()
    });
    let node4 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(20f32),
            height: taffy::style::Dimension::Length(20f32),
        },
        ..Default::default()
    });
    let node5 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(20f32),
            height: taffy::style::Dimension::Length(20f32),
        },
        ..Default::default()
    });
    let node60 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        ..Default::default()
    });
    let node6 = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(20f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        },
        &[node60],
    );
    let node70 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(5f32) },
        ..Default::default()
    });
    let node7 = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(20f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        },
        &[node70],
    );
    let node8 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(20f32),
            height: taffy::style::Dimension::Length(20f32),
        },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            align_items: Some(taffy::style::AlignItems::Baseline),
            grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
            grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(120f32),
                height: taffy::style::Dimension::Length(120f32),
            },
            ..Default::default()
        },
        &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node, 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node0, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node1, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1, 40f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node1, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10);
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node10, 0f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node10, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node2, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node2, 80f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node2, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node20);
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node20, 0f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node20, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node20, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node20, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node3, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node3, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node4, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node4, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node4, 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node5, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node5, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node5, 80f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node5, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node6, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node6, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node6, 0f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node6, 90f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node60);
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node60, 0f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node60, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node60, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node60, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node7);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node7, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node7, 20f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node7, 40f32, location.x);
    assert_eq!(location.y, 95f32, "y of node {:?}. Expected {}. Actual {}", node7, 95f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node70);
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node70, 0f32, size.width);
    assert_eq!(size.height, 5f32, "height of node {:?}. Expected {}. Actual {}", node70, 5f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node70, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node70, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node8);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node8, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node8, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node8, 80f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node8, 80f32, location.y);
}
