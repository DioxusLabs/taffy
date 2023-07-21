#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() });
    let node1 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() });
    let node2 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() });
    let node3 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() });
    let node4 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() });
    let node = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(113f32),
                height: taffy::style::Dimension::Length(100f32),
            },
            ..Default::default()
        },
        &[node0, node1, node2, node3, node4],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 113f32, "width of node {:?}. Expected {}. Actual {}", node, 113f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 23f32, "width of node {:?}. Expected {}. Actual {}", node0, 23f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 22f32, "width of node {:?}. Expected {}. Actual {}", node1, 22f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node1, 100f32, size.height);
    assert_eq!(location.x, 23f32, "x of node {:?}. Expected {}. Actual {}", node1, 23f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 23f32, "width of node {:?}. Expected {}. Actual {}", node2, 23f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node2, 100f32, size.height);
    assert_eq!(location.x, 45f32, "x of node {:?}. Expected {}. Actual {}", node2, 45f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3);
    assert_eq!(size.width, 22f32, "width of node {:?}. Expected {}. Actual {}", node3, 22f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node3, 100f32, size.height);
    assert_eq!(location.x, 68f32, "x of node {:?}. Expected {}. Actual {}", node3, 68f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4);
    assert_eq!(size.width, 23f32, "width of node {:?}. Expected {}. Actual {}", node4, 23f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node4, 100f32, size.height);
    assert_eq!(location.x, 90f32, "x of node {:?}. Expected {}. Actual {}", node4, 90f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node4, 0f32, location.y);
}
