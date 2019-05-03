#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]);
    let node1 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]);
    let node2 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]);
    let node3 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]);
    let node4 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]);
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(113f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1, node2, node3, node4],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 113f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 23f32);
    assert_eq!(stretch.layout(node0).size.height, 100f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 22f32);
    assert_eq!(stretch.layout(node1).size.height, 100f32);
    assert_eq!(stretch.layout(node1).location.x, 23f32);
    assert_eq!(stretch.layout(node1).location.y, 0f32);
    assert_eq!(stretch.layout(node2).size.width, 23f32);
    assert_eq!(stretch.layout(node2).size.height, 100f32);
    assert_eq!(stretch.layout(node2).location.x, 45f32);
    assert_eq!(stretch.layout(node2).location.y, 0f32);
    assert_eq!(stretch.layout(node3).size.width, 22f32);
    assert_eq!(stretch.layout(node3).size.height, 100f32);
    assert_eq!(stretch.layout(node3).location.x, 68f32);
    assert_eq!(stretch.layout(node3).location.y, 0f32);
    assert_eq!(stretch.layout(node4).size.width, 23f32);
    assert_eq!(stretch.layout(node4).size.height, 100f32);
    assert_eq!(stretch.layout(node4).location.x, 90f32);
    assert_eq!(stretch.layout(node4).location.y, 0f32);
}
