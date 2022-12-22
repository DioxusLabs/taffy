#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(113f32),
                    height: taffy::style::Dimension::Points(100f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 113f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 23f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 23f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 23f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 23f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 45f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 23f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 68f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node4).unwrap().size.width, 23f32);
    assert_eq!(taffy.layout(node4).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node4).unwrap().location.x, 90f32);
    assert_eq!(taffy.layout(node4).unwrap().location.y, 0f32);
}
