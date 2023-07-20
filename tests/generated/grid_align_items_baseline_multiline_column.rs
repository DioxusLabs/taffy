#[test]
fn grid_align_items_baseline_multiline_column() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(50f32),
            height: taffy::style::Dimension::Length(50f32),
        },
        ..Default::default()
    });
    let node10 = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(20f32),
            height: taffy::style::Dimension::Length(20f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(30f32),
                height: taffy::style::Dimension::Length(50f32),
            },
            ..Default::default()
        },
        &[node10],
    );
    let node20 = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(10f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node2 = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(40f32),
                height: taffy::style::Dimension::Length(70f32),
            },
            ..Default::default()
        },
        &[node20],
    );
    let node3 = taffy.new_leaf(taffy::style::Style {
        display: taffy::style::Display::Grid,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(50f32),
            height: taffy::style::Dimension::Length(20f32),
        },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            align_items: Some(taffy::style::AlignItems::Baseline),
            grid_template_columns: vec![auto(), auto()],
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(100f32),
                height: taffy::style::Dimension::Length(100f32),
            },
            ..Default::default()
        },
        &[node0, node1, node2, node3],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node0, 50f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node1, 30f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node1, 50f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node1, 50f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node1, 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10);
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node10, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node10, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 70f32, "height of node {:?}. Expected {}. Actual {}", node2, 70f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node2, 90f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node20);
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node20, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node20, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node20, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node20, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3);
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node3, 50f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node3, 20f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node3, 50f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node3, 80f32, location.y);
}
