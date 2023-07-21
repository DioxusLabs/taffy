#[test]
fn justify_content_column_space_evenly() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            flex_direction: taffy::style::FlexDirection::Column,
            justify_content: Some(taffy::style::JustifyContent::SpaceEvenly),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(100f32),
                height: taffy::style::Dimension::Length(100f32),
            },
            ..Default::default()
        },
        &[node0, node1, node2],
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
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0, 100f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 18f32, "y of node {:?}. Expected {}. Actual {}", node0, 18f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node1, 100f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 45f32, "y of node {:?}. Expected {}. Actual {}", node1, 45f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node2, 100f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node2, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 73f32, "y of node {:?}. Expected {}. Actual {}", node2, 73f32, location.y);
}
